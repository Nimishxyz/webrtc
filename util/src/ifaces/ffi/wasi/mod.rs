use crate::ifaces::Interface;
use std::io::Error;

/// Query the local system for all interface addresses.
///
/// On WASI, interface enumeration is not supported, so this returns an empty list.
pub fn ifaces() -> Result<Vec<Interface>, Error> {
    // WASI doesn't support interface enumeration
    Ok(Vec::new())
}
