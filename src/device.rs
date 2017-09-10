extern crate nearfield_sys;

use std::ptr;
use std::ffi::CStr;
use ::{Context, Error};
use error::NFCResult;
use utils;

type InternalDevice = nearfield_sys::nfc_device;

pub type ConnectionString = nearfield_sys::nfc_connstring;
pub type Mode = nearfield_sys::nfc_mode;
pub type Modulation = nearfield_sys::nfc_modulation_type;
pub type BaudRate = nearfield_sys::nfc_baud_rate;
pub type Property = nearfield_sys::nfc_property;

pub struct Device {
    device: *mut InternalDevice,
}

impl Device {
    pub(crate) fn new(context: *mut Context, mut conn_string: ConnectionString) -> Result<Device, Error> {
        let device = unsafe { nearfield_sys::nfc_open(context, conn_string.as_mut_ptr()) };

        if device.is_null() {
            return Err(Error::OpenDevice);
        }

        return Ok(Device {
            device,
        });
    }

    pub fn name(&mut self) -> &'static str {
        return unsafe { CStr::from_ptr(nearfield_sys::nfc_device_get_name(&mut *self.device)).to_str().unwrap() };
    }

    pub fn connection_string(&mut self) -> &'static str {
        return unsafe { CStr::from_ptr(nearfield_sys::nfc_device_get_name(&mut *self.device)).to_str().unwrap() };
    }

    pub fn modulation(&mut self, mode: Mode) -> Result<Modulation, Error> {
        let modulation: *const Modulation = ptr::null_mut();
        let status = unsafe { nearfield_sys::nfc_device_get_supported_modulation(&mut *self.device, mode, &modulation) } as i32;
        let status = Error::from_sys(status);

        match status {
            Error::Success => unsafe { Ok(*modulation) },
            _ => Err(status)
        }
    }

    pub fn baud_rate(&mut self, nmt: Modulation) -> Result<BaudRate, Error> {
        let baud_rate: *const BaudRate = ptr::null_mut();
        let status = unsafe { nearfield_sys::nfc_device_get_supported_baud_rate(&mut *self.device, nmt, &baud_rate) } as i32;
        let status = Error::from_sys(status);

        match status {
            Error::Success => unsafe { Ok(*baud_rate) },
            _ => Err(status)
        }
    }

    pub fn idle(&mut self) -> NFCResult {
        let status = unsafe { nearfield_sys::nfc_idle(&mut *self.device) } as i32;
        return utils::status_to_result(status);
    }

    pub fn abort(&mut self) -> NFCResult {
        let status = unsafe { nearfield_sys::nfc_abort_command(&mut *self.device) } as i32;
        return utils::status_to_result(status);
    }

    pub fn set_property_int(&mut self, property: Property, value: i32) -> NFCResult {
        let status = unsafe { nearfield_sys::nfc_device_set_property_int(&mut *self.device, property, value) } as i32;
        return utils::status_to_result(status);
    }

    pub fn set_property_bool(&mut self, property: Property, enable: bool) -> NFCResult {
        let status = unsafe { nearfield_sys::nfc_device_set_property_bool(&mut *self.device, property, enable) } as i32;
        return utils::status_to_result(status);
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { nearfield_sys::nfc_close(&mut *self.device) };
    }
}
