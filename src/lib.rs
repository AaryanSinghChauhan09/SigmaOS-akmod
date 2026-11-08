//! SigmaOS-akmod — comprehensive kernel module builder component.
//!
//! Provides automatic kernel module building functionality with
//! dependency resolution, kernel version tracking, and build management.

pub mod module;
pub mod kernel;
pub mod builder;
pub mod dependency;
pub mod config;
pub mod manager;
pub mod options;

pub use module::{KernelModule, ModuleStatus, ModuleType, ModuleDependency};
pub use kernel::{KernelVersion, KernelInfo, KernelConfig};
pub use builder::{ModuleBuilder, BuildConfig, BuildResult, BuildStatus};
pub use dependency::{DependencyResolver, DependencyGraph, DependencyNode};
pub use config::{AkmodConfig, BuildSettings};
pub use manager::{AkmodManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_status() {
        let statuses = vec![
            ModuleStatus::Source,
            ModuleStatus::Building,
            ModuleStatus::Built,
            ModuleStatus::Installed,
            ModuleStatus::Failed,
            ModuleStatus::Removed,
        ];
        assert_eq!(statuses.len(), 6);
    }
    
    #[test]
    fn test_module_type() {
        let types = vec![
            ModuleType::Driver,
            ModuleType::Filesystem,
            ModuleType::Network,
            ModuleType::Cryptography,
            ModuleType::Security,
            ModuleType::Virtualization,
            ModuleType::Hardware,
            ModuleType::Custom,
        ];
        assert_eq!(types.len(), 8);
    }
    
    #[test]
    fn test_build_status() {
        let statuses = vec![
            BuildStatus::Pending,
            BuildStatus::Building,
            BuildStatus::Success,
            BuildStatus::Failed,
            BuildStatus::Cancelled,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_module_creation() {
        let module = KernelModule::new(
            "nvidia".into(),
            "550.67".into(),
            "/usr/src/nvidia-550.67".into(),
            ModuleType::Driver,
        );
        
        assert_eq!(module.name, "nvidia");
        assert_eq!(module.version, "550.67");
        assert_eq!(module.module_type, ModuleType::Driver);
    }
    
    #[test]
    fn test_kernel_version_creation() {
        let kernel = KernelVersion::new(
            "6.6.1".into(),
            "x86_64".into(),
        );
        
        assert_eq!(kernel.version, "6.6.1");
        assert_eq!(kernel.architecture, "x86_64");
    }
    
    #[test]
    fn test_build_config_creation() {
        let config = BuildConfig::default();
        assert_eq!(config.optimization_level, "O2");
        assert!(config.parallel_jobs > 0);
    }
    
    #[test]
    fn test_build_result_creation() {
        let result = BuildResult::new(
            "build-1".into(),
            "module-1".into(),
            "6.6.1".into(),
        );
        
        assert_eq!(result.id, "build-1");
        assert_eq!(result.module_id, "module-1");
        assert_eq!(result.kernel_version, "6.6.1");
    }
    
    #[test]
    fn test_dependency_resolver_creation() {
        let resolver = DependencyResolver::new();
        assert!(resolver.get_dependencies().is_empty());
    }
    
    #[test]
    fn test_add_dependency() {
        let mut resolver = DependencyResolver::new();
        resolver.add_dependency("module-a".into(), "module-b".into());
        
        // Both module-a and module-b are added as nodes
        assert_eq!(resolver.get_dependencies().len(), 2);
    }
    
    #[test]
    fn test_resolve_dependencies() {
        let mut resolver = DependencyResolver::new();
        resolver.add_dependency("module-a".into(), "module-b".into());
        resolver.add_dependency("module-b".into(), "module-c".into());
        
        let resolved = resolver.resolve_dependencies("module-a".into());
        assert!(resolved.contains(&"module-a".to_string()));
        assert!(resolved.contains(&"module-b".to_string()));
        assert!(resolved.contains(&"module-c".to_string()));
    }
    
    #[test]
    fn test_builder_creation() {
        let builder = ModuleBuilder::new();
        assert!(builder.get_builds().is_empty());
    }
    
    #[test]
    fn test_build_module() {
        let mut builder = ModuleBuilder::new();
        let module = KernelModule::new(
            "test".into(),
            "1.0".into(),
            "/usr/src/test".into(),
            ModuleType::Custom,
        );
        
        let build_id = builder.build_module(module, "6.6.1".into());
        
        assert!(builder.get_build(&build_id).is_some());
        assert_eq!(builder.get_builds().len(), 1);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AkmodManager::new();
        assert!(manager.list_modules().is_empty());
    }
    
    #[test]
    fn test_add_module() {
        let mut manager = AkmodManager::new();
        let module_id = manager.add_module(
            "nvidia".into(),
            "550.67".into(),
            "/usr/src/nvidia-550.67".into(),
            ModuleType::Driver,
        );
        
        assert!(manager.get_module(&module_id).is_some());
        assert_eq!(manager.list_modules().len(), 1);
    }
    
    #[test]
    fn test_build_module_in_manager() {
        let mut manager = AkmodManager::new();
        let module_id = manager.add_module(
            "test".into(),
            "1.0".into(),
            "/usr/src/test".into(),
            ModuleType::Custom,
        );
        
        let build_id = manager.build_module(module_id.clone(), "6.6.1".into());
        
        assert!(manager.get_build(&build_id).is_some());
        assert_eq!(manager.list_builds().len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AkmodManager::new();
        
        let module_id = manager.add_module(
            "test".into(),
            "1.0".into(),
            "/usr/src/test".into(),
            ModuleType::Custom,
        );
        manager.build_module(module_id, "6.6.1".into());
        
        let export = manager.export_data();
        
        let mut manager2 = AkmodManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_modules().len(), 1);
        assert_eq!(manager2.list_builds().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AkmodManager::new();
        
        manager.add_module("mod1".into(), "1.0".into(), "/usr/src/mod1".into(), ModuleType::Driver);
        manager.add_module("mod2".into(), "2.0".into(), "/usr/src/mod2".into(), ModuleType::Network);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_modules, 2);
        assert_eq!(stats.total_builds, 0);
    }
}
