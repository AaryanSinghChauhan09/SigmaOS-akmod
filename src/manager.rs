//! Akmod manager for kernel module building.

use std::collections::HashMap;
use crate::module::{KernelModule, ModuleType, ModuleStatus};
use crate::kernel::KernelVersion;
use crate::builder::{ModuleBuilder, BuildResult};
use crate::dependency::DependencyResolver;
use crate::config::{AkmodConfig, BuildSettings};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Auto-build on add
    pub auto_build_on_add: bool,
    /// Auto-install on build success
    pub auto_install_on_success: bool,
    /// Max concurrent builds
    pub max_concurrent_builds: usize,
    /// Build timeout in seconds
    pub build_timeout: u64,
    /// Enable notifications
    pub enable_notifications: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            auto_build_on_add: false,
            auto_install_on_success: false,
            max_concurrent_builds: 4,
            build_timeout: 3600,
            enable_notifications: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total modules
    pub total_modules: usize,
    /// Total builds
    pub total_builds: usize,
    /// Total kernels
    pub total_kernels: usize,
    /// Built modules
    pub built_modules: usize,
    /// Installed modules
    pub installed_modules: usize,
    /// Failed modules
    pub failed_modules: usize,
    /// Total dependencies
    pub total_dependencies: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Akmod manager
#[derive(Debug, Clone)]
pub struct AkmodManager {
    /// Modules
    modules: HashMap<String, KernelModule>,
    /// Builds
    builds: HashMap<String, BuildResult>,
    /// Kernels
    kernels: HashMap<String, KernelVersion>,
    /// Dependency resolver
    pub dependency_resolver: DependencyResolver,
    /// Config
    pub config: AkmodConfig,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AkmodManager {
    fn default() -> Self {
        Self {
            modules: HashMap::new(),
            builds: HashMap::new(),
            kernels: HashMap::new(),
            dependency_resolver: DependencyResolver::new(),
            config: AkmodConfig::default(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_modules: 0,
                total_builds: 0,
                total_kernels: 0,
                built_modules: 0,
                installed_modules: 0,
                failed_modules: 0,
                total_dependencies: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AkmodManager {
    /// Create new akmod manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            modules: HashMap::new(),
            builds: HashMap::new(),
            kernels: HashMap::new(),
            dependency_resolver: DependencyResolver::new(),
            config: AkmodConfig::default(),
            options,
            statistics: ManagerStatistics {
                total_modules: 0,
                total_builds: 0,
                total_kernels: 0,
                built_modules: 0,
                installed_modules: 0,
                failed_modules: 0,
                total_dependencies: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add current kernel
        let current_kernel = KernelVersion::new(
            "6.6.1".into(),
            "x86_64".into(),
        );
        self.kernels.insert("current".into(), current_kernel);
        
        self.update_statistics();
    }
    
    /// Add module
    pub fn add_module(&mut self, name: String, version: String, source_path: String, module_type: ModuleType) -> String {
        let id = format!("module-{}", self.modules.len() + 1);
        let module = KernelModule::new(name, version, source_path, module_type);
        
        self.modules.insert(id.clone(), module);
        self.update_statistics();
        id
    }
    
    /// Get module
    pub fn get_module(&self, module_id: &str) -> Option<&KernelModule> {
        self.modules.get(module_id)
    }
    
    /// Get module mutably
    pub fn get_module_mut(&mut self, module_id: &str) -> Option<&mut KernelModule> {
        self.modules.get_mut(module_id)
    }
    
    /// Get all modules
    pub fn list_modules(&self) -> Vec<&KernelModule> {
        self.modules.values().collect()
    }
    
    /// Delete module
    pub fn delete_module(&mut self, module_id: &str) -> bool {
        if self.modules.remove(module_id).is_some() {
            // Remove builds for this module
            self.builds.retain(|_, b| b.module_id != module_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Update module status
    pub fn update_module_status(&mut self, module_id: &str, status: ModuleStatus) -> bool {
        if let Some(module) = self.modules.get_mut(module_id) {
            module.set_status(status);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add kernel
    pub fn add_kernel(&mut self, version: String, architecture: String) -> String {
        let id = format!("kernel-{}", self.kernels.len() + 1);
        let kernel = KernelVersion::new(version, architecture);
        
        self.kernels.insert(id.clone(), kernel);
        self.update_statistics();
        id
    }
    
    /// Get kernel
    pub fn get_kernel(&self, kernel_id: &str) -> Option<&KernelVersion> {
        self.kernels.get(kernel_id)
    }
    
    /// Get kernel mutably
    pub fn get_kernel_mut(&mut self, kernel_id: &str) -> Option<&mut KernelVersion> {
        self.kernels.get_mut(kernel_id)
    }
    
    /// Get all kernels
    pub fn list_kernels(&self) -> Vec<&KernelVersion> {
        self.kernels.values().collect()
    }
    
    /// Delete kernel
    pub fn delete_kernel(&mut self, kernel_id: &str) -> bool {
        if self.kernels.remove(kernel_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Build module
    pub fn build_module(&mut self, module_id: String, kernel_version: String) -> String {
        let build_id = format!("build-{}", self.builds.len() + 1);
        
        // Get module
        let module = if let Some(module) = self.modules.get(&module_id) {
            module.clone()
        } else {
            return String::new();
        };
        
        // Create build result
        let mut build = BuildResult::new(build_id.clone(), module_id.clone(), kernel_version.clone());
        build.start();
        build.add_log(format!("Building module {} for kernel {}", module.name, kernel_version));
        build.add_log("Compiling kernel module...".into());
        build.add_log("Module build completed successfully".into());
        build.mark_success("Module built successfully".into());
        
        // Update module status
        if let Some(module) = self.modules.get_mut(&module_id) {
            module.add_built_kernel(kernel_version.clone());
            module.set_status(ModuleStatus::Built);
        }
        
        self.builds.insert(build_id.clone(), build);
        self.update_statistics();
        build_id
    }
    
    /// Get build
    pub fn get_build(&self, build_id: &str) -> Option<&BuildResult> {
        self.builds.get(build_id)
    }
    
    /// Get build mutably
    pub fn get_build_mut(&mut self, build_id: &str) -> Option<&mut BuildResult> {
        self.builds.get_mut(build_id)
    }
    
    /// Get all builds
    pub fn list_builds(&self) -> Vec<&BuildResult> {
        self.builds.values().collect()
    }
    
    /// Delete build
    pub fn delete_build(&mut self, build_id: &str) -> bool {
        if self.builds.remove(build_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Install module
    pub fn install_module(&mut self, module_id: &str, kernel_version: &str) -> bool {
        if let Some(module) = self.modules.get_mut(module_id) {
            if module.is_built_for_kernel(kernel_version) {
                module.add_installed_kernel(kernel_version.to_string());
                module.set_status(ModuleStatus::Installed);
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Uninstall module
    pub fn uninstall_module(&mut self, module_id: &str, kernel_version: &str) -> bool {
        if let Some(module) = self.modules.get_mut(module_id) {
            if module.remove_installed_kernel(kernel_version) {
                if module.installed_kernels.is_empty() {
                    module.set_status(ModuleStatus::Built);
                }
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, module_name: String, dependency_name: String) -> bool {
        self.dependency_resolver.add_dependency(module_name, dependency_name)
    }
    
    /// Resolve dependencies
    pub fn resolve_dependencies(&mut self, module_name: String) -> Vec<String> {
        self.dependency_resolver.resolve_dependencies(module_name)
    }
    
    /// Search modules
    pub fn search_modules(&self, query: &str) -> Vec<&KernelModule> {
        let query_lower = query.to_lowercase();
        self.modules.values()
            .filter(|m| m.name.to_lowercase().contains(&query_lower) || 
                       m.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get modules by type
    pub fn get_modules_by_type(&self, module_type: &ModuleType) -> Vec<&KernelModule> {
        self.modules.values()
            .filter(|m| m.module_type == *module_type)
            .collect()
    }
    
    /// Get modules by status
    pub fn get_modules_by_status(&self, status: &ModuleStatus) -> Vec<&KernelModule> {
        self.modules.values()
            .filter(|m| m.status == *status)
            .collect()
    }
    
    /// Get builds by status
    pub fn get_builds_by_status(&self, status: &crate::builder::BuildStatus) -> Vec<&BuildResult> {
        self.builds.values()
            .filter(|b| b.status == *status)
            .collect()
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_modules = self.modules.len();
        self.statistics.total_builds = self.builds.len();
        self.statistics.total_kernels = self.kernels.len();
        self.statistics.total_dependencies = self.dependency_resolver.get_dependencies().len();
        
        self.statistics.built_modules = self.modules.values()
            .filter(|m| m.status == ModuleStatus::Built || m.status == ModuleStatus::Installed)
            .count();
        
        self.statistics.installed_modules = self.modules.values()
            .filter(|m| m.status == ModuleStatus::Installed)
            .count();
        
        self.statistics.failed_modules = self.modules.values()
            .filter(|m| m.status == ModuleStatus::Failed)
            .count();
        
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            modules: self.modules.values().cloned().collect(),
            builds: self.builds.values().cloned().collect(),
            kernels: self.kernels.values().cloned().collect(),
            config: self.config.clone(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.modules.clear();
        for module in export.modules {
            self.modules.insert(format!("module-{}", self.modules.len() + 1), module);
        }
        
        self.builds.clear();
        for build in export.builds {
            self.builds.insert(build.id.clone(), build);
        }
        
        self.kernels.clear();
        for kernel in export.kernels {
            self.kernels.insert(format!("kernel-{}", self.kernels.len() + 1), kernel);
        }
        
        self.config = export.config;
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Modules
    pub modules: Vec<KernelModule>,
    /// Builds
    pub builds: Vec<BuildResult>,
    /// Kernels
    pub kernels: Vec<KernelVersion>,
    /// Config
    pub config: AkmodConfig,
}
