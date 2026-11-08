//! Kernel module definitions for akmod.

/// Module status
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatus {
    /// Source only
    Source,
    /// Building
    Building,
    /// Built
    Built,
    /// Installed
    Installed,
    /// Failed
    Failed,
    /// Removed
    Removed,
}

/// Module type
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    /// Driver module
    Driver,
    /// Filesystem module
    Filesystem,
    /// Network module
    Network,
    /// Cryptography module
    Cryptography,
    /// Security module
    Security,
    /// Virtualization module
    Virtualization,
    /// Hardware module
    Hardware,
    /// Custom module
    Custom,
}

/// Module dependency
#[derive(Debug, Clone)]
pub struct ModuleDependency {
    /// Dependency module name
    pub module_name: String,
    /// Dependency version
    pub version: String,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Dependency optional
    pub optional: bool,
}

/// Dependency type
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    /// Required dependency
    Required,
    /// Recommended dependency
    Recommended,
    /// Suggested dependency
    Suggested,
    /// Build dependency
    Build,
}

/// Kernel module
#[derive(Debug, Clone)]
pub struct KernelModule {
    /// Module name
    pub name: String,
    /// Module version
    pub version: String,
    /// Module source path
    pub source_path: String,
    /// Module type
    pub module_type: ModuleType,
    /// Module status
    pub status: ModuleStatus,
    /// Module dependencies
    pub dependencies: Vec<ModuleDependency>,
    /// Module description
    pub description: String,
    /// Module author
    pub author: String,
    /// Module license
    pub license: String,
    /// Module created timestamp
    pub created_at: u64,
    /// Module updated timestamp
    pub updated_at: u64,
    /// Module built for kernels
    pub built_kernels: Vec<String>,
    /// Module installed for kernels
    pub installed_kernels: Vec<String>,
    /// Module build failures
    pub build_failures: Vec<BuildFailure>,
}

/// Build failure
#[derive(Debug, Clone)]
pub struct BuildFailure {
    /// Kernel version
    pub kernel_version: String,
    /// Failure timestamp
    pub timestamp: u64,
    /// Failure error message
    pub error_message: String,
    /// Failure log output
    pub log_output: String,
}

impl KernelModule {
    /// Create new kernel module
    pub fn new(name: String, version: String, source_path: String, module_type: ModuleType) -> Self {
        Self {
            name,
            version,
            source_path,
            module_type,
            status: ModuleStatus::Source,
            dependencies: vec![],
            description: String::new(),
            author: String::new(),
            license: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            built_kernels: vec![],
            installed_kernels: vec![],
            build_failures: vec![],
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set author
    pub fn set_author(&mut self, author: String) {
        self.author = author;
        self.update_timestamp();
    }
    
    /// Set license
    pub fn set_license(&mut self, license: String) {
        self.license = license;
        self.update_timestamp();
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, dependency: ModuleDependency) {
        self.dependencies.push(dependency);
        self.update_timestamp();
    }
    
    /// Remove dependency
    pub fn remove_dependency(&mut self, module_name: &str) -> bool {
        if let Some(pos) = self.dependencies.iter().position(|d| d.module_name == module_name) {
            self.dependencies.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set status
    pub fn set_status(&mut self, status: ModuleStatus) {
        self.status = status;
        self.update_timestamp();
    }
    
    /// Add built kernel
    pub fn add_built_kernel(&mut self, kernel_version: String) {
        if !self.built_kernels.contains(&kernel_version) {
            self.built_kernels.push(kernel_version);
            self.update_timestamp();
        }
    }
    
    /// Remove built kernel
    pub fn remove_built_kernel(&mut self, kernel_version: &str) -> bool {
        if let Some(pos) = self.built_kernels.iter().position(|k| k == kernel_version) {
            self.built_kernels.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Add installed kernel
    pub fn add_installed_kernel(&mut self, kernel_version: String) {
        if !self.installed_kernels.contains(&kernel_version) {
            self.installed_kernels.push(kernel_version);
            self.update_timestamp();
        }
    }
    
    /// Remove installed kernel
    pub fn remove_installed_kernel(&mut self, kernel_version: &str) -> bool {
        if let Some(pos) = self.installed_kernels.iter().position(|k| k == kernel_version) {
            self.installed_kernels.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Add build failure
    pub fn add_build_failure(&mut self, kernel_version: String, error_message: String, log_output: String) {
        let failure = BuildFailure {
            kernel_version,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            error_message,
            log_output,
        };
        self.build_failures.push(failure);
        self.update_timestamp();
    }
    
    /// Clear build failures
    pub fn clear_build_failures(&mut self) {
        self.build_failures.clear();
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if module is built for kernel
    pub fn is_built_for_kernel(&self, kernel_version: &str) -> bool {
        self.built_kernels.contains(&kernel_version.to_string())
    }
    
    /// Check if module is installed for kernel
    pub fn is_installed_for_kernel(&self, kernel_version: &str) -> bool {
        self.installed_kernels.contains(&kernel_version.to_string())
    }
    
    /// Check if module has dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }
    
    /// Get dependency count
    pub fn get_dependency_count(&self) -> usize {
        self.dependencies.len()
    }
    
    /// Get built kernel count
    pub fn get_built_kernel_count(&self) -> usize {
        self.built_kernels.len()
    }
    
    /// Get installed kernel count
    pub fn get_installed_kernel_count(&self) -> usize {
        self.installed_kernels.len()
    }
    
    /// Get build failure count
    pub fn get_build_failure_count(&self) -> usize {
        self.build_failures.len()
    }
    
    /// Get module age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
}
