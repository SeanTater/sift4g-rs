use anyhow::{bail, ensure, Result};
use clap::Parser;
use clap::ValueEnum;
use sift4g_rs::sift_prediction::delete_shotgun_database;
use std::path::Path;

use sift4g_rs::database_alignment::align_database;
use sift4g_rs::database_search::search_database;
use sift4g_rs::select_alignments::{
    delete_selected_alignments, output_selected_alignments, select_alignments,
};
use sift4g_rs::sift_prediction::{output_shotgun_database, sift_predictions};
use sift4g_rs::swsharp::{create_e_value_params, delete_e_value_params, scorer_create_matrix};
use sift4g_rs::utils::{check_data, delete_fasta_chains, is_extant_path, read_fasta_chains};

#[derive(Parser, Debug)]
#[command(name = "SIFT4G", version, author, about)]
struct Args {
    /// Input fasta query file
    #[arg(short = 'q', long, required = true)]
    query: String,

    /// Input fasta database file
    #[arg(short = 'd', long, required = true)]
    database: String,

    /// Gap opening penalty (default: 10)
    #[arg(short = 'g', long, default_value_t = 10)]
    gap_open: isize,

    /// Gap extension penalty (default: 1)
    #[arg(short = 'e', long, default_value_t = 1)]
    gap_extend: isize,

    /// Similarity matrix (default: BLOSUM_62)
    #[arg(long, default_value = "BLOSUM_62")]
    matrix: String,

    /// E-value threshold (default: 0.0001)
    #[arg(long, default_value_t = 0.0001)]
    evalue: f64,

    /// Maximum number of alignments to be outputted (default: 400)
    #[arg(long, default_value_t = 400)]
    max_aligns: usize,

    /// Algorithm used for alignment (default: SW)
    #[arg(long, default_value = "SW")]
    algorithm: Algorithm,

    /// Output directory for SIFT predictions (default: current directory)
    #[arg(short = 'o', long, default_value = ".")]
    out: String,

    /// Print sub results to output directory
    #[arg(long)]
    sub_results: bool,

    /// Output format for the alignment file (default: bm9)
    #[arg(long, default_value = "BM9")]
    outfmt: OutFormat,

    /// Length of kmers used for database search (possible values: 3, 4, 5) (default: 5)
    #[arg(long, default_value_t = 5)]
    kmer_length: usize,

    /// Number of database sequences passed on to the Smith-Waterman part (default: 5000)
    #[arg(long, default_value_t = 5000)]
    max_candidates: usize,

    /// Alignment diversity threshold (default: 2.75)
    #[arg(long, default_value_t = 2.75)]
    median_threshold: f64,

    /// Directory containing substitution files for each query (default: current directory)
    #[arg(long, default_value = ".")]
    subst: String,

    /// Sequence identity threshold (default: 100)
    #[arg(long, default_value_t = 100)]
    seq_id: usize,

    /// Number of threads used in thread pool (default: 8)
    #[arg(short = 't', long, default_value_t = 8)]
    threads: usize,
}

#[derive(ValueEnum, Clone, Debug, strum::Display)]
enum Algorithm {
    SW,
    NW,
    HW,
    OV,
}

#[derive(ValueEnum, Clone, Debug, strum::Display)]
enum OutFormat {
    BM0,
    BM8,
    BM9,
    LIGHT,
}

impl From<&str> for OutFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bm0" => OutFormat::BM0,
            "bm8" => OutFormat::BM8,
            "bm9" => OutFormat::BM9,
            "light" => OutFormat::LIGHT,
            _ => panic!("Unknown output format: {}", s),
        }
    }
}

impl From<&str> for Algorithm {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "sw" => Algorithm::SW,
            "nw" => Algorithm::NW,
            "hw" => Algorithm::HW,
            "ov" => Algorithm::OV,
            _ => panic!("Unknown algorithm: {}", s),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    ensure!(!args.query.is_empty(), "missing option -q (query file)");
    ensure!(
        Path::new(&args.query).exists(),
        "invalid query file path '{}'",
        &args.query
    );

    ensure!(
        !args.database.is_empty(),
        "missing option -d (database file)"
    );
    ensure!(
        Path::new(&args.database).exists(),
        "invalid database file path '{}'",
        &args.database
    );

    ensure!(
        args.kmer_length > 2 && args.kmer_length < 6,
        "kmer_length possible values = 3,4,5"
    );
    ensure!(args.max_candidates > 0, "invalid max candidates number");

    ensure!(args.evalue > 0.0, "invalid evalue");
    if !args.out.is_empty() {
        ensure!(
            Path::new(&args.out).exists(),
            "invalid out directory path '{}'",
            &args.out
        );
    }

    if !args.subst.is_empty() {
        ensure!(
            Path::new(&args.subst).exists(),
            "invalid substitutions directory path '{}'",
            &args.subst
        );
    }

    let mut queries = Vec::new();
    read_fasta_chains(&mut queries, &args.query);
    check_data(&queries, &args.subst)?;

    if queries.is_empty() {
        bail!("** EXITING! No valid queries to process. **");
    }

    let indices = search_database(
        &args.database,
        &queries,
        args.kmer_length,
        args.max_candidates,
        args.threads,
    );

    let scorer = scorer_create_matrix(&args.matrix, args.gap_open, args.gap_extend);
    let evalue_params = create_e_value_params(indices.len(), &scorer);

    let (alignments, alignments_lenghts, database) = align_database(
        &args.database,
        &queries,
        &indices,
        &args.algorithm.to_string(),
        &evalue_params,
        args.evalue,
        args.max_aligns,
        &scorer,
    );

    delete_e_value_params(&evalue_params);

    if args.sub_results {
        let alignments_path = format!("{}{}.txt", &args.out, "alignments");
        output_shotgun_database(
            &alignments,
            alignments_lenghts.len(),
            &queries,
            &alignments_path,
            &args.outfmt.to_string(),
        );
    }

    let mut alignment_strings = Vec::new();
    select_alignments(
        &mut alignment_strings,
        &alignments,
        &queries,
        args.median_threshold,
    );

    delete_shotgun_database(alignments, alignments_lenghts.len());
    delete_fasta_chains(database);

    if args.sub_results {
        output_selected_alignments(&alignment_strings, &queries, &args.out);
    }

    sift_predictions(
        &alignment_strings,
        &queries,
        &args.subst,
        args.seq_id,
        &args.out,
    );

    delete_selected_alignments(alignment_strings);
    delete_fasta_chains(queries);

    Ok(())
}
