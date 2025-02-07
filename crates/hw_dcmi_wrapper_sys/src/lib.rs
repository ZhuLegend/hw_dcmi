//! Rust bindings for the [DaVinci Card Management Interface][https://support.huawei.com/enterprise/zh/doc/EDOC1100349020/426cffd9] (DCMI), a C-based programmatic
//! interface for monitoring and managing various states within Huawei NPUs.
//!
//! It is intended to be a platform for building 3rd-party applications, and is also the
//! underlying library for Huawei's npu-smi tool.
//!
//! See [`hw_dcmi_wrapper`][https://github.com/ZhuLegend/hw_dcmi_wrapper] for a safe wrapper over top of these bindings.
//!
//! ## About Bindings
//!
//! These binding is generated by [bindgen](https://github.com/rust-lang/rust-bindgen), and you can obtain the binding of dynamic links by enabling the 'load_dynamic'
//! feature (via [`libloading`](https://github.com/nagisa/rust_libloading)). Otherwise, the binding of static links will be generated.
//!
//! By default, the library searches for DCMI components in the `/usr/local/dcmi` directory.
//! You can override this path by setting the `HW_DCMI_PATH` environment variable.
//!
//! If you want to regenerate bindings, you can set `HW_DCMI_BINDING_BUILD` to `true` to regenerate bindings,
//! the generated bindings will be saved in:
//!
//! - Static link: `hw_dcmi_wrapper_sys/src/bindings.rs`
//! - Dynamic link: `hw_dcmi_wrapper_sys/src/bindings_dyn.rs`

// bindgen generates code that triggers these lints
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

// We avoid generating layout tests because they cause a large number of
// warnings and according to commentary are not useful. See
// https://github.com/rust-lang/rust-bindgen/issues/1651 for more.
#[cfg(not(feature = "load_dynamic"))]
pub mod bindings;

#[cfg(feature = "load_dynamic")]
pub mod bindings_dyn;

#[cfg(feature = "load_dynamic")]
pub use bindings_dyn as bindings;
