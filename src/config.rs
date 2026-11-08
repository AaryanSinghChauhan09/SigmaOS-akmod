//! Configuration for akmod.

/// Build settings
#[derive(Debug, Clone)]
pub struct BuildSettings {
    /// Parallel jobs
    pub parallel_jobs: u32,
    /// Optimization level
    pub optimization_level: String,
    /// Debug info
    pub debug_info: bool,
    /// Strip symbols
    pub strip_symbols: bool,
    /// Build timeout in seconds
    pub build_timeout: u64,
    /// Retry count
    pub retry_count: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
}

impl Default for BuildSettings {
    fn default() -> Self {
        Self {
            parallel_jobs: 4,
            optimization_level: "O2".into(),
            debug_info: false,
            strip_symbols: true,
            build_timeout: 3600,
            retry_count: 3,
            retry_delay: 60,
        }
    }
}

/// Akmod configuration
#[derive(Debug, Clone)]
pub struct AkmodConfig {
    /// Build settings
    pub build_settings: BuildSettings,
    /// Kernel versions to build for
    pub kernel_versions: Vec<String>,
    /// Modules to build
    pub modules_to_build: Vec<String>,
    /// Modules to exclude
    pub modules_to_exclude: Vec<String>,
    /// Auto-build on kernel update
    pub auto_build_on_kernel_update: bool,
    /// Auto-install after build
    pub auto_install_after_build: bool,
    /// Enable notifications
    pub enable_notifications: bool,
    /// Config file path
    pub config_path: String,
}

impl Default for AkmodConfig {
    fn default() -> Self {
        Self {
            build_settings: BuildSettings::default(),
            kernel_versions: vec![],
            modules_to_build: vec![],
            modules_to_exclude: vec![],
            auto_build_on_kernel_update: true,
            auto_install_after_build: false,
            enable_notifications: true,
            config_path: "/etc/akmod.conf".into(),
        }
    }
}

impl AkmodConfig {
    /// Create new akmod config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set build settings
    pub fn set_build_settings(&mut self, settings: BuildSettings) {
        self.build_settings = settings;
    }
    
    /// Add kernel version
    pub fn add_kernel_version(&mut self, version: String) {
        if !self.kernel_versions.contains(&version) {
            self.kernel_versions.push(version);
        }
    }
    
    /// Remove kernel version
    pub fn remove_kernel_version(&mut self, version: &str) -> bool {
        if let Some(pos) = self.kernel_versions.iter().position(|v| v == version) {
            self.kernel_versions.remove(pos);
            return true;
        }
        false
    }
    
    /// Add module to build
    pub fn add_module_to_build(&mut self, module: String) {
        if !self.modules_to_build.contains(&module) {
            self.modules_to_build.push(module);
        }
    }
    
    /// Remove module from build
    pub fn remove_module_from_build(&mut self, module: &str) -> bool {
        if let Some(pos) = self.modules_to_build.iter().position(|m| m == module) {
            self.modules_to_build.remove(pos);
            return true;
        }
        false
    }
    
    /// Add module to exclude
    pub fn add_module_to_exclude(&mut self, module: String) {
        if !self.modules_to_exclude.contains(&module) {
            self.modules_to_exclude.push(module);
        }
    }
    
    /// Remove module from exclude
    pub fn remove_module_from_exclude(&mut self, module: &str) -> bool {
        if let Some(pos) = self.modules_to_exclude.iter().position(|m| m == module) {
            self.modules_to_exclude.remove(pos);
            return true;
        }
        false
    }
    
    /// Enable auto-build on kernel update
    pub fn enable_auto_build(&mut self) {
        self.auto_build_on_kernel_update = true;
    }
    
    /// Disable auto-build on kernel update
    pub fn disable_auto_build(&mut self) {
        self.auto_build_on_kernel_update = false;
    }
    
    /// Enable auto-install after build
    pub fn enable_auto_install(&mut self) {
        self.auto_install_after_build = true;
    }
    
    /// Disable auto-install after build
    pub fn disable_auto_install(&mut self) {
        self.auto_install_after_build = false;
    }
    
    /// Enable notifications
    pub fn enable_notifications(&mut self) {
        self.enable_notifications = true;
    }
    
    /// Disable notifications
    pub fn disable_notifications(&mut self) {
        self.enable_notifications = false;
    }
    
    /// Set config path
    pub fn set_config_path(&mut self, path: String) {
        self.config_path = path;
    }
}
