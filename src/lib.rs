extern crate nearfield_sys;

mod utils;
mod error;

use std::{ptr, slice};
use std::ffi::CStr;
use error::NFCResult;
use device::ConnectionString;

pub mod device;

pub use error::Error;
pub use device::Device;

pub type Context = nearfield_sys::nfc_context;
pub type Driver = nearfield_sys::nfc_driver;

pub struct NFC {
    context: *mut Context,
}

impl NFC {
    pub fn new() -> Result<NFC, Error> {
        let mut context: *mut Context = ptr::null_mut();
        unsafe { nearfield_sys::nfc_init(&mut context); }

        if context.is_null() {
            return Err(Error::Software);
        }

        return Ok(NFC {
            context
        });
    }

    pub fn version() -> &'static str {
        return unsafe { CStr::from_ptr(nearfield_sys::nfc_version()).to_str().unwrap() };
    }

    pub fn register_driver(driver: Driver) -> NFCResult {
        let status = unsafe { nearfield_sys::nfc_register_driver(&driver) } as i32;
        return utils::status_to_result(status);
    }

    pub fn list_devices(&mut self) -> Result<&[ConnectionString], Error> {
        let connection_strings: *mut ConnectionString = ptr::null_mut();
        let connection_strings_len: usize = 0;

        let status = unsafe { nearfield_sys::nfc_list_devices(self.context, connection_strings, connection_strings_len) } as i32;
        let status = Error::from_sys(status);

        match status {
            Error::Success => unsafe { Ok(slice::from_raw_parts(connection_strings, connection_strings_len)) },
            _ => Err(status),
        }
    }

    pub fn open(&mut self, conn_string: ConnectionString) -> Result<Device, Error> {
        return Device::new(self.context, conn_string);
    }
}

impl Drop for NFC {
    fn drop(&mut self) {
        unsafe { nearfield_sys::nfc_exit(&mut *self.context) };
    }
}