#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
pub use self::windows::ifaces;

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use self::unix::ifaces;

#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(target_os = "wasi")]
pub use self::wasi::ifaces;
