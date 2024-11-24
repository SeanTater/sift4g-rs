use crate::containers::{Alignment, Record};

pub fn select_alignments(
    alignment_strings: &mut Vec<String>,
    alignments: &[Alignment],
    queries: &[Record],
    median_threshold: f64,
) {
    // Implementation for selecting alignments
}

pub fn delete_selected_alignments(alignment_strings: Vec<String>) {
    // Implementation for deleting selected alignments
}

pub fn output_selected_alignments(alignment_strings: &[String], queries: &[Record], out_dir: &str) {
    // Implementation for outputting selected alignments
}
