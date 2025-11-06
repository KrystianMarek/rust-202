//! # LLD Linker and Build Optimizations
//!
//! Information about modern Rust build improvements including the LLD linker,
//! which became the default for certain platforms in recent versions.

/// Build configuration examples and linker information
///
/// ## Why?
/// The LLD linker (LLVM's linker) provides significantly faster link times
/// compared to traditional linkers like GNU ld, especially for large projects.
/// This improves the developer experience with faster iteration cycles.
///
/// ## Comparison
/// - **Rust (LLD)**: Fast linking, integrated with LLVM toolchain
/// - **C/C++ (GNU ld)**: Traditional, slower for large binaries
/// - **Go**: Fast linking built-in (single-pass linker)
/// - **Python**: No linking (interpreted)
pub struct BuildOptimizations;

impl BuildOptimizations {
    /// Information about LLD linker benefits
    ///
    /// LLD is now the default linker for:
    /// - x86_64-unknown-linux-gnu (as of Rust 1.71)
    /// - Windows MSVC targets
    /// - macOS (via lld-compatible linker)
    pub fn linker_info() -> &'static str {
        "LLD (LLVM Linker) provides:
        - Faster link times (up to 2x faster than GNU ld)
        - Better parallelization
        - Consistent behavior across platforms
        - Integrated with Rust's LLVM backend"
    }

    /// Compile-time optimization features
    ///
    /// ## Example optimization levels:
    /// - `opt-level = 0`: No optimization (debug)
    /// - `opt-level = 1`: Basic optimization
    /// - `opt-level = 2`: Some optimization
    /// - `opt-level = 3`: Full optimization (release default)
    /// - `opt-level = "s"`: Optimize for size
    /// - `opt-level = "z"`: Optimize for size aggressively
    pub fn optimization_levels() -> Vec<(&'static str, &'static str)> {
        vec![
            ("0", "Debug: No optimization, fastest compile time"),
            ("1", "Basic: Some optimization, reasonable compile time"),
            ("2", "Default: Good optimization, moderate compile time"),
            ("3", "Release: Full optimization, slower compile time"),
            ("s", "Size: Optimize for binary size"),
            ("z", "Min Size: Aggressively optimize for size"),
        ]
    }

    /// Link-Time Optimization (LTO) benefits
    ///
    /// ## Why?
    /// LTO performs whole-program optimization across all compilation units,
    /// enabling better inlining and dead code elimination than possible
    /// with per-compilation-unit optimization.
    ///
    /// Unlike C where LTO setup can be complex, Rust makes it simple:
    /// just set `lto = true` in Cargo.toml
    pub fn lto_benefits() -> &'static str {
        "Link-Time Optimization (LTO) provides:
        - Better inlining across crate boundaries
        - More effective dead code elimination
        - Smaller binary sizes
        - Better runtime performance
        - Trade-off: Longer compile times"
    }
}

/// Example: Compile-time feature flags
///
/// Demonstrates conditional compilation, a zero-cost abstraction
/// that eliminates unused code paths at compile-time.
///
/// ## Why?
/// Feature flags allow shipping a single codebase that compiles differently
/// based on target platform or enabled features, with zero runtime cost.
pub struct FeatureFlags;

impl FeatureFlags {
    /// Get platform-specific information
    pub fn platform_info() -> &'static str {
        #[cfg(target_os = "linux")]
        {
            "Running on Linux with LLD linker by default"
        }
        #[cfg(target_os = "macos")]
        {
            "Running on macOS with ld64 (LLD-compatible)"
        }
        #[cfg(target_os = "windows")]
        {
            "Running on Windows with LLD-link"
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            "Running on other platform"
        }
    }

    /// Demonstrate conditional compilation
    pub fn debug_info() -> &'static str {
        #[cfg(debug_assertions)]
        {
            "Debug mode: assertions enabled"
        }
        #[cfg(not(debug_assertions))]
        {
            "Release mode: assertions disabled"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_info() {
        let info = BuildOptimizations::linker_info();
        assert!(info.contains("LLD"));
    }

    #[test]
    fn test_optimization_levels() {
        let levels = BuildOptimizations::optimization_levels();
        assert_eq!(levels.len(), 6);
        assert_eq!(levels[0].0, "0");
        assert_eq!(levels[3].0, "3");
    }

    #[test]
    fn test_platform_info() {
        let info = FeatureFlags::platform_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_debug_info() {
        let info = FeatureFlags::debug_info();
        assert!(!info.is_empty());
    }
}
