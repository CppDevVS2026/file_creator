/// Represents the command line arguments
///
/// pattern is for the create command
/// path is where the file will be created.
pub struct Cli {
    pub pattern: String,
    pub path: std::path::PathBuf,
}