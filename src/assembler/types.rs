pub struct Metadata {
    /// Where to find include files
    pub search_path: std::path::PathBuf,
}
impl std::default::Default for Metadata {
    fn default() -> Self {
        Metadata {
            search_path: std::path::PathBuf::from(""),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LabelUse {
    /// Where was this label used?
    pub location: usize,
    /// Was this label used for a relative (branch) instruction
    pub is_relative: bool,
}
