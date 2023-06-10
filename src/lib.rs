#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(target_env = "gnu", target_env = "musl"))]
pub mod ld_preload;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod dyld_insert_libraries;

#[doc(hidden)]
#[cfg(any(target_env = "gnu", target_env = "musl"))]
pub use once_cell::race::OnceNonZeroUsize;
