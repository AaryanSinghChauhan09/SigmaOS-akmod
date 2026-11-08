//! Command-line options for akmod.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Add mode
    Add,
    /// Build mode
    Build,
    /// Install mode
    Install,
    /// Remove mode
    Remove,
    /// List mode
    List,
    /// Status mode
    Status,
    /// Config mode
    Config,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Module name
    pub module_name: String,
    /// Kernel version
    pub kernel_version: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            module_name: String::new(),
            kernel_version: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
