# SigmaOS-akmod Component Wiki

## Overview
SigmaOS-akmod provides kernel module support and kernel extension framework for the SigmaOS operating system. It demonstrates the kernel subsystem and kernel module management within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic kernel initialization.

## Principles
- Single Responsibility
- Kernel Abstraction
- Fail-Fast on Kernel Initialization Errors
- Deterministic Kernel Setup
- Clear Abstraction Over Kernel APIs
- Composition over Inheritance for kernel extensions

## Architecture
SigmaOS-akmod follows a kernel pattern:
- **Kernel Core**: Provides kernel module loading, device driver management, and kernel extension points
- **Module Management**: Supports kernel module loading, unloading, and dependency resolution
- **Device Driver**: Supports hardware device drivers, kernel-mediated hardware access, and driver binding
- **Error Handling**: Fail-fast on invalid kernel configurations or unsupported kernel types

## API
The component provides the following public APIs:
- `kernel::module::load(path)` - Load a kernel module from the given path
- `kernel::module::unload(name)` - Unload a kernel module by name
- `kernel::device::list()` - List detected devices and kernel-mediated hardware
- `kernel::extension::register(extension)` - Register a kernel extension point
- Error types for kernel failure scenarios or unsupported kernel types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific kernel configuration
2. **Act**: Execute the code under test (module loading, device detection)
3. **Assert**: Verify the expected kernel results

Example:
```rust
#[test]
fn test_kernel_module_aaa() {
    // Arrange
    let module_path = "/lib/modules/test-module.ko";
    
    // Act
    let result = kernel::module::load(module_path);
    
    // Assert
    assert!(result.is_ok());
    let loaded = result.unwrap();
    assert!(loaded.is_loaded());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Kernel initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all kernel failure scenarios
- Unit tests provided for module loading and device detection
- Integration tests for full kernel subsystem with actual kernel module execution

## TODO
- Add support for additional kernel features (module signatures, auto-loading, dependency resolution)
- Expand device detection capabilities (PCIe enumeration, device IDs, driver versions)
- Add extension features (kernel boot parameters, hot-plug handling, power management)
- Create integration tests with actual kernel module execution and hardware access
- Implement module persistence and kernel configuration management