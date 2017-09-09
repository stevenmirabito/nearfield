extern crate nearfield_sys;

mod utils;
mod error;
pub mod device;

use std::ptr;

pub use error::Error;
use error::NFCResult;

pub use device::Device;
use device::ConnectionString;

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

    pub fn destroy(&mut self) {
        unsafe { nearfield_sys::nfc_exit(&mut *self.context) };
    }

    pub fn register_driver(driver: Driver) -> NFCResult {
        let result = unsafe { nearfield_sys::nfc_register_driver(&driver) } as i32;
        return utils::status_to_result(result);
    }

    pub fn open(&mut self, mut conn_string: ConnectionString) -> Result<Device, Error> {
        return Device::new(self.context, conn_string);
    }
}