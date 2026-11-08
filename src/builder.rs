//! Module builder for akmod.

use crate::module::{KernelModule, ModuleStatus};
use crate::kernel::KernelVersion;

/// Build status
#[derive(Debug, Clone, PartialEq)]
pub enum BuildStatus {
    /// Pending
    Pending,
    /// Building
    Building,
    /// Success
    Success,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Build config
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Build parallel jobs
    pub parallel_jobs: u32,
    /// Build optimization level
    pub optimization_level: String,
    /// Build debug info
    pub debug_info: bool,
    /// Build strip symbols
    pub strip_symbols: bool,
    /// Build timeout in seconds
    pub timeout: u64,
    /// Build retry count
    pub retry_count: u32,
    /// Build retry delay in seconds
    pub retry_delay: u64,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            parallel_jobs: 4,
            optimization_level: "O2".into(),
            debug_info: false,
            strip_symbols: true,
            timeout: 3600,
            retry_count: 3,
            retry_delay: 60,
        }
    }
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Build ID
    pub id: String,
    /// Module ID
    pub module_id: String,
    /// Kernel version
    pub kernel_version: String,
    /// Build status
    pub status: BuildStatus,
    /// Build started timestamp
    pub started_at: u64,
    /// Build finished timestamp
    pub finished_at: Option<u64>,
    /// Build duration in seconds
    pub duration: Option<u64>,
    /// Build output
    pub output: String,
    /// Build error message
    pub error_message: Option<String>,
    /// Build log output
    pub log_output: String,
    /// Build retries
    pub retries: u32,
}

impl BuildResult {
    /// Create new build result
    pub fn new(id: String, module_id: String, kernel_version: String) -> Self {
        Self {
            id,
            module_id,
            kernel_version,
            status: BuildStatus::Pending,
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            finished_at: None,
            duration: None,
            output: String::new(),
            error_message: None,
            log_output: String::new(),
            retries: 0,
        }
    }
    
    /// Start build
    pub fn start(&mut self) {
        self.status = BuildStatus::Building;
        self.started_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Mark build as success
    pub fn mark_success(&mut self, output: String) {
        self.status = BuildStatus::Success;
        self.finished_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.duration = self.finished_at.map(|f| f - self.started_at);
        self.output = output;
    }
    
    /// Mark build as failed
    pub fn mark_failed(&mut self, error_message: String) {
        self.status = BuildStatus::Failed;
        self.finished_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.duration = self.finished_at.map(|f| f - self.started_at);
        self.error_message = Some(error_message);
    }
    
    /// Mark build as cancelled
    pub fn mark_cancelled(&mut self) {
        self.status = BuildStatus::Cancelled;
        self.finished_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.duration = self.finished_at.map(|f| f - self.started_at);
    }
    
    /// Add log output
    pub fn add_log(&mut self, log: String) {
        self.log_output.push_str(&log);
        self.log_output.push('\n');
    }
    
    /// Set output
    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }
    
    /// Increment retries
    pub fn increment_retries(&mut self) {
        self.retries += 1;
    }
    
    /// Check if build is running
    pub fn is_running(&self) -> bool {
        self.status == BuildStatus::Building
    }
    
    /// Check if build is finished
    pub fn is_finished(&self) -> bool {
        matches!(self.status, BuildStatus::Success | BuildStatus::Failed | BuildStatus::Cancelled)
    }
    
    /// Check if build is successful
    pub fn is_success(&self) -> bool {
        self.status == BuildStatus::Success
    }
    
    /// Check if build is failed
    pub fn is_failed(&self) -> bool {
        self.status == BuildStatus::Failed
    }
    
    /// Get build age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if let Some(finished) = self.finished_at {
            finished - self.started_at
        } else {
            now - self.started_at
        }
    }
}

/// Module builder
#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    /// Build results
    pub builds: std::collections::HashMap<String, BuildResult>,
    /// Build config
    pub config: BuildConfig,
    /// Builder running
    pub running: bool,
    /// Max concurrent builds
    pub max_concurrent: usize,
    /// Current concurrent builds
    pub current_concurrent: usize,
}

impl Default for ModuleBuilder {
    fn default() -> Self {
        Self {
            builds: std::collections::HashMap::new(),
            config: BuildConfig::default(),
            running: false,
            max_concurrent: 4,
            current_concurrent: 0,
        }
    }
}

impl ModuleBuilder {
    /// Create new module builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Start builder
    pub fn start(&mut self) {
        self.running = true;
    }
    
    /// Stop builder
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Build module
    pub fn build_module(&mut self, module: KernelModule, kernel_version: String) -> String {
        let build_id = format!("build-{}", self.builds.len() + 1);
        let mut build = BuildResult::new(build_id.clone(), module.name.clone(), kernel_version);
        
        // Simulate build
        build.start();
        build.add_log(format!("Building module {} for kernel {}", module.name, build.kernel_version));
        build.add_log("Compiling kernel module...".into());
        build.add_log("Module build completed successfully".into());
        build.mark_success("Module built successfully".into());
        
        self.builds.insert(build_id.clone(), build);
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
    pub fn get_builds(&self) -> Vec<&BuildResult> {
        self.builds.values().collect()
    }
    
    /// Delete build
    pub fn delete_build(&mut self, build_id: &str) -> bool {
        self.builds.remove(build_id).is_some()
    }
    
    /// Cancel build
    pub fn cancel_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            if build.is_running() {
                build.mark_cancelled();
                return true;
            }
        }
        false
    }
    
    /// Retry build
    pub fn retry_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            if build.is_failed() {
                build.increment_retries();
                build.status = BuildStatus::Pending;
                build.finished_at = None;
                build.duration = None;
                build.error_message = None;
                return true;
            }
        }
        false
    }
    
    /// Get builds by status
    pub fn get_builds_by_status(&self, status: &BuildStatus) -> Vec<&BuildResult> {
        self.builds.values()
            .filter(|b| b.status == *status)
            .collect()
    }
    
    /// Get builds by module
    pub fn get_builds_by_module(&self, module_id: &str) -> Vec<&BuildResult> {
        self.builds.values()
            .filter(|b| b.module_id == module_id)
            .collect()
    }
    
    /// Get builds by kernel
    pub fn get_builds_by_kernel(&self, kernel_version: &str) -> Vec<&BuildResult> {
        self.builds.values()
            .filter(|b| b.kernel_version == kernel_version)
            .collect()
    }
    
    /// Get running builds
    pub fn get_running_builds(&self) -> Vec<&BuildResult> {
        self.get_builds_by_status(&BuildStatus::Building)
    }
    
    /// Get successful builds
    pub fn get_successful_builds(&self) -> Vec<&BuildResult> {
        self.get_builds_by_status(&BuildStatus::Success)
    }
    
    /// Get failed builds
    pub fn get_failed_builds(&self) -> Vec<&BuildResult> {
        self.get_builds_by_status(&BuildStatus::Failed)
    }
    
    /// Get builder statistics
    pub fn get_statistics(&self) -> BuilderStatistics {
        BuilderStatistics {
            total_builds: self.builds.len(),
            running_builds: self.builds.values().filter(|b| b.is_running()).count(),
            successful_builds: self.builds.values().filter(|b| b.is_success()).count(),
            failed_builds: self.builds.values().filter(|b| b.is_failed()).count(),
            cancelled_builds: self.builds.values().filter(|b| b.status == BuildStatus::Cancelled).count(),
            total_retries: self.builds.values().map(|b| b.retries).sum(),
        }
    }
}

/// Builder statistics
#[derive(Debug, Clone)]
pub struct BuilderStatistics {
    /// Total builds
    pub total_builds: usize,
    /// Running builds
    pub running_builds: usize,
    /// Successful builds
    pub successful_builds: usize,
    /// Failed builds
    pub failed_builds: usize,
    /// Cancelled builds
    pub cancelled_builds: usize,
    /// Total retries
    pub total_retries: u32,
}
