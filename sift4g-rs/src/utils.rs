use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::containers::Record;

pub const BUFFER_SIZE: usize = 4096;

/// Checks if a path exists and returns its type.
///
/// Returns:
/// - 0 if the path is a directory
/// - 1 if the path is a regular file
/// - 2 if the path is something else
/// - -1 if the path does not exist
pub fn is_extant_path(path: &str) -> isize {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                0
            } else if metadata.is_file() {
                1
            } else {
                2
            }
        }
        Err(_) => -1,
    }
}

/// Creates a file name with the given path and extension.
///
/// # Arguments
/// * `name` - The base name of the file.
/// * `path` - The directory path to prepend, if any.
/// * `extension` - The file extension to append.
///
/// # Returns
/// A `String` containing the full file name.
pub fn create_file_name(name: &str, path: &str, extension: &str) -> String {
    let mut file_name = String::with_capacity(BUFFER_SIZE);

    if !path.is_empty() {
        file_name.push_str(path);
        file_name.push('/');
    }

    file_name.push_str(name);
    file_name.push_str(extension);

    file_name
}

/// Logs the progress of processing queries.
///
/// # Arguments
/// * `part` - The current part being processed.
/// * `total` - The total number of parts to process.
pub fn query_log(part: usize, total: usize) {
    eprint!(
        "* processing queries: {:.2}/100.00% *\r",
        100.0 * part as f64 / total as f64
    );
}

/// Logs the progress of processing database parts.
///
/// # Arguments
/// * `part` - The current part number being processed.
/// * `part_size` - The size of the current part in GB.
/// * `percentage` - The percentage of completion.
pub fn database_log(part: usize, part_size: f64, percentage: f64) {
    eprint!(
        "* processing database part {} (size ~{:.2} GB): {:.2}/100.00% *\r",
        part, part_size, percentage
    );
}

pub fn delete_fasta_chains(queries: Vec<Record>) {
    // Implementation for deleting FASTA chains
}

pub fn read_fasta_chains(queries: &mut Vec<Record>, query_file: &str) {
    // Implementation for reading FASTA chains
}

pub fn check_data(queries: &[Record], subst: &str) -> Result<()> {
    // Implementation for checking data
    Ok(())
}
