//! Kernel version management for akmod.

/// Kernel version
#[derive(Debug, Clone, PartialEq)]
pub struct KernelVersion {
    /// Kernel version string
    pub version: String,
    /// Kernel architecture
    pub architecture: String,
    /// Kernel release date
    pub release_date: u64,
    /// Kernel config path
    pub config_path: String,
    /// Kernel modules path
    pub modules_path: String,
    /// Kernel headers path
    pub headers_path: String,
    /// Kernel source path
    pub source_path: String,
    /// Kernel installed
    pub installed: bool,
    /// Kernel running
    pub running: bool,
}

impl KernelVersion {
    /// Create new kernel version
    pub fn new(version: String, architecture: String) -> Self {
        Self {
            version,
            architecture,
            release_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            config_path: String::new(),
            modules_path: String::new(),
            headers_path: String::new(),
            source_path: String::new(),
            installed: false,
            running: false,
        }
    }
    
    /// Set config path
    pub fn set_config_path(&mut self, path: String) {
        self.config_path = path;
    }
    
    /// Set modules path
    pub fn set_modules_path(&mut self, path: String) {
        self.modules_path = path;
    }
    
    /// Set headers path
    pub fn set_headers_path(&mut self, path: String) {
        self.headers_path = path;
    }
    
    /// Set source path
    pub fn set_source_path(&mut self, path: String) {
        self.source_path = path;
    }
    
    /// Mark as installed
    pub fn mark_installed(&mut self) {
        self.installed = true;
    }
    
    /// Mark as running
    pub fn mark_running(&mut self) {
        self.running = true;
    }
    
    /// Check if kernel is installed
    pub fn is_installed(&self) -> bool {
        self.installed
    }
    
    /// Check if kernel is running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Get kernel age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.release_date
    }
}

/// Kernel info
#[derive(Debug, Clone)]
pub struct KernelInfo {
    /// Kernel name
    pub name: String,
    /// Kernel version
    pub version: String,
    /// Kernel architecture
    pub architecture: String,
    /// Kernel release date
    pub release_date: u64,
    /// Kernel config path
    pub config_path: String,
    /// Kernel modules path
    pub modules_path: String,
    /// Kernel headers path
    pub headers_path: String,
    /// Kernel source path
    pub source_path: String,
    /// Kernel installed
    pub installed: bool,
    /// Kernel running
    pub running: bool,
    /// Kernel modules count
    pub modules_count: usize,
    /// Kernel drivers count
    pub drivers_count: usize,
}

impl KernelInfo {
    /// Create new kernel info
    pub fn new(version: KernelVersion) -> Self {
        Self {
            name: "linux".into(),
            version: version.version,
            architecture: version.architecture,
            release_date: version.release_date,
            config_path: version.config_path,
            modules_path: version.modules_path,
            headers_path: version.headers_path,
            source_path: version.source_path,
            installed: version.installed,
            running: version.running,
            modules_count: 0,
            drivers_count: 0,
        }
    }
    
    /// Set modules count
    pub fn set_modules_count(&mut self, count: usize) {
        self.modules_count = count;
    }
    
    /// Set drivers count
    pub fn set_drivers_count(&mut self, count: usize) {
        self.drivers_count = count;
    }
    
    /// Get kernel age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.release_date
    }
}

/// Kernel config
#[derive(Debug, Clone)]
pub struct KernelConfig {
    /// Kernel version
    pub version: String,
    /// Kernel architecture
    pub architecture: String,
    /// Kernel config options
    pub options: std::collections::HashMap<String, String>,
    /// Kernel modules to build
    pub modules_to_build: Vec<String>,
    /// Kernel modules to exclude
    pub modules_to_exclude: Vec<String>,
    /// Kernel build parallel jobs
    pub parallel_jobs: u32,
    /// Kernel build optimization level
    pub optimization_level: String,
    /// Kernel build debug info
    pub debug_info: bool,
    /// Kernel build strip symbols
    pub strip_symbols: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            version: String::new(),
            architecture: String::new(),
            options: std::collections::HashMap::new(),
            modules_to_build: vec![],
            modules_to_exclude: vec![],
            parallel_jobs: 4,
            optimization_level: "O2".into(),
            debug_info: false,
            strip_symbols: true,
        }
    }
}

impl KernelConfig {
    /// Create new kernel config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set version
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    
    /// Set architecture
    pub fn set_architecture(&mut self, architecture: String) {
        self.architecture = architecture;
    }
    
    /// Set option
    pub fn set_option(&mut self, key: String, value: String) {
        self.options.insert(key, value);
    }
    
    /// Get option
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
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
    
    /// Set parallel jobs
    pub fn set_parallel_jobs(&mut self, jobs: u32) {
        self.parallel_jobs = jobs;
    }
    
    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: String) {
        self.optimization_level = level;
    }
    
    /// Enable debug info
    pub fn enable_debug_info(&mut self) {
        self.debug_info = true;
    }
    
    /// Disable debug info
    pub fn disable_debug_info(&mut self) {
        self.debug_info = false;
    }
    
    /// Enable strip symbols
    pub fn enable_strip_symbols(&mut self) {
        self.strip_symbols = true;
    }
    
    /// Disable strip symbols
    pub fn disable_strip_symbols(&mut self) {
        self.strip_symbols = false;
    }
}
