extern crate nearfield_sys;

mod error;
mod utils;

use device::ConnectionString;
use error::NFCResult;
use std::ffi::CStr;
use std::{ptr, slice};

pub mod baud_rate;
pub mod device;
pub mod modulation;
pub mod modulation_type;
pub mod target;

pub use device::{Device, Initiator};
pub use error::Error;
use utils::status_to_result;

pub type Context = nearfield_sys::nfc_context;
pub type Driver = nearfield_sys::nfc_driver;

pub struct NFC {
    context: *mut Context,
}

impl NFC {
    pub fn new() -> Result<NFC, Error> {
        let mut context: *mut Context = ptr::null_mut();
        unsafe {
            nearfield_sys::nfc_init(&mut context);
        }

        if context.is_null() {
            return Err(Error::Software);
        }

        return Ok(NFC { context });
    }

    pub fn version() -> &'static str {
        return unsafe {
            CStr::from_ptr(nearfield_sys::nfc_version())
                .to_str()
                .unwrap()
        };
    }

    pub fn register_driver(driver: Driver) -> NFCResult<()> {
        let status = unsafe { nearfield_sys::nfc_register_driver(&driver) } as i32;
        return utils::status_to_result(status);
    }

    pub fn list_devices(&mut self) -> Result<&[ConnectionString], Error> {
        let connection_strings: *mut ConnectionString = ptr::null_mut();
        let connection_strings_len: usize = 0;

        let status = unsafe {
            nearfield_sys::nfc_list_devices(
                self.context,
                connection_strings,
                connection_strings_len,
            )
        } as i32;
        let status = status_to_result(status);

        match status {
            Ok(()) => unsafe {
                Ok(slice::from_raw_parts(
                    connection_strings,
                    connection_strings_len,
                ))
            },
            Err(e) => Err(e),
        }
    }

    pub fn open_initiator(&mut self) -> Result<Initiator, Error> {
        Initiator::new(self.context)
    }

    pub fn open_initiator_with_conn_string(
        &mut self,
        conn_string: ConnectionString,
    ) -> Result<Initiator, Error> {
        Initiator::with_conn_string(self.context, conn_string)
    }

    //    pub fn open_target_with_conn_string(&mut self, conn_string: ConnectionString) -> Result<Target, Error> {
    //        Target::with_conn_string(self.context, conn_string);
    //    }
}

impl Drop for NFC {
    fn drop(&mut self) {
        unsafe { nearfield_sys::nfc_exit(&mut *self.context) };
    }
}
