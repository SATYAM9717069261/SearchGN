use std::path::PathBuf;

pub struct MergeJob {
    pub left_path: PathBuf,
    pub right_path: PathBuf,
    pub output_path: PathBuf,
    pub round: usize,
    pub output_id: usize,
}
