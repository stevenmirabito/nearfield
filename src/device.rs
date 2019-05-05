extern crate nearfield_sys;

use baud_rate::{BaudRate, InternalBaudRate};
use error::NFCResult;
use modulation::{InternalModulation, Modulation};
use modulation_type::ModulationType;
use std::ffi::CStr;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use target::{InternalTarget, Target};
use utils;
use {Context, Error};
use utils::status_to_result;

type InternalDevice = nearfield_sys::nfc_device;

pub type ConnectionString = nearfield_sys::nfc_connstring;
pub type Mode = nearfield_sys::nfc_mode;
pub type Property = nearfield_sys::nfc_property;

pub struct Device {
    device: *mut InternalDevice,
}

impl Device {
    pub fn name(&mut self) -> &'static str {
        return unsafe {
            CStr::from_ptr(nearfield_sys::nfc_device_get_name(&mut *self.device))
                .to_str()
                .unwrap()
        };
    }

    pub fn connection_string(&mut self) -> &'static str {
        return unsafe {
            CStr::from_ptr(nearfield_sys::nfc_device_get_name(&mut *self.device))
                .to_str()
                .unwrap()
        };
    }

    pub fn baud_rate(&mut self, nmt: ModulationType) -> Result<BaudRate, Error> {
        let baud_rate: *const InternalBaudRate = ptr::null_mut();
        let status = unsafe {
            nearfield_sys::nfc_device_get_supported_baud_rate(
                &mut *self.device,
                nmt.into(),
                &baud_rate,
            )
        } as i32;

        let status = status_to_result(status);

        match status {
            Ok(()) => unsafe { Ok(BaudRate::from(*baud_rate)) },
            Err(status) => Err(status),
        }
    }

    pub fn idle(&mut self) -> NFCResult<()> {
        let status = unsafe { nearfield_sys::nfc_idle(&mut *self.device) } as i32;
        utils::status_to_result(status)
    }

    pub fn abort(&mut self) -> NFCResult<()> {
        let status = unsafe { nearfield_sys::nfc_abort_command(&mut *self.device) } as i32;
        utils::status_to_result(status)
    }

    pub fn set_property_int(&mut self, property: Property, value: i32) -> NFCResult<()> {
        let status = unsafe {
            nearfield_sys::nfc_device_set_property_int(&mut *self.device, property, value)
        } as i32;
        utils::status_to_result(status)
    }

    pub fn set_property_bool(&mut self, property: Property, enable: bool) -> NFCResult<()> {
        let status = unsafe {
            nearfield_sys::nfc_device_set_property_bool(&mut *self.device, property, enable)
        } as i32;
        utils::status_to_result(status)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { nearfield_sys::nfc_close(&mut *self.device) };
    }
}

pub struct Initiator(Device);

impl From<Device> for Initiator {
    fn from(device: Device) -> Initiator {
        Initiator(device)
    }
}

// Allow an Initiator to use all Device functions
impl Deref for Initiator {
    type Target = Device;
    fn deref(&self) -> &Device {
        &self.0
    }
}

impl DerefMut for Initiator {
    fn deref_mut(&mut self) -> &mut Device {
        &mut self.0
    }
}

impl Initiator {
    pub fn new(context: *mut Context) -> Result<Initiator, Error> {
        let raw_device = unsafe { nearfield_sys::nfc_open(context, ptr::null_mut()) };

        if raw_device.is_null() {
            return Err(Error::OpenDevice);
        }

        let status = unsafe { nearfield_sys::nfc_initiator_init(&mut *raw_device) } as i32;
        let error = status_to_result(status);
        match error {
            Ok(()) => Ok(Device { device: raw_device }.into()),
            Err(error) => Err(error),
        }
    }

    pub fn with_conn_string(
        context: *mut Context,
        mut conn_string: ConnectionString,
    ) -> Result<Initiator, Error> {
        let raw_device = unsafe { nearfield_sys::nfc_open(context, conn_string.as_mut_ptr()) };

        if raw_device.is_null() {
            return Err(Error::OpenDevice);
        }

        let status = unsafe { nearfield_sys::nfc_initiator_init(&mut *raw_device) } as i32;
        let error = status_to_result(status);
        match error {
            Ok(()) => Ok(Device { device: raw_device }.into()),
            Err(error) => Err(error),
        }
    }

    pub fn poll(
        &mut self,
        modulations: &[Modulation],
        num_polls: u8,
        poll_period: u8,
    ) -> Result<Target, Error> {
        let int_mods: Vec<InternalModulation> =
            modulations.iter().map(Clone::clone).map(Modulation::into).collect();
        let mut raw_target: InternalTarget;

        raw_target = unsafe { mem::uninitialized() };
        let status = unsafe {
            nearfield_sys::nfc_initiator_poll_target(
                &mut *self.device,
                int_mods.as_ptr(),
                int_mods.len(),
                num_polls,
                poll_period,
                &mut raw_target,
            )
        };

        match status_to_result(status) {
            // Successfully read at least one target
            Ok(()) => Ok(Target::from_sys(raw_target)),
            Err(error) => Err(error)
        }
    }
}

//pub struct Target(Device);
//
//impl From<Device> for Target {
//    fn from(device: Device) -> Target { Target(device) }
//}
//
//// Allow a Target to use all Device functions
//impl Deref for Target {
//    type Target = Device;
//    fn deref(&self) -> &Device { &self.0 }
//}
//
//impl DerefMut for Target {
//    fn deref_mut(&mut self) -> &mut Target { &mut self.0 }
//}
//
//impl Target {
//    pub fn new(context: *mut Context) -> Result<Target, Error> {
//        let raw_device = unsafe { nearfield_sys::nfc_open(context, ptr::null_mut()) };
//
//        if raw_device.is_null() {
//            return Err(Error::OpenDevice);
//        }
//
//        let mut status = unsafe { nearfield_sys::nfc_target_init(&mut *raw_device) } as i32;
//        let error = Error::from_sys(status);
//        match error {
//            Error::Success => Ok(Target { device: raw_device }),
//            _ => Err(error),
//        }
//    }
//
//    pub fn with_conn_string(context: *mut Context, mut conn_string: ConnectionString) -> Result<Target, Error> {
//        let raw_device = unsafe { nearfield_sys::nfc_open(context, conn_string.as_mut_ptr()) };
//
//        if raw_device.is_null() {
//            return Err(Error::OpenDevice);
//        }
//
//        let mut status = unsafe { nearfield_sys::nfc_target_init(&mut *raw_device) } as i32;
//        let error = Error::from_sys(status);
//        match error {
//            Error::Success => Ok(Target { device: raw_device }),
//            _ => Err(error),
//        }
//    }
//}
