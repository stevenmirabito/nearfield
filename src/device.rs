extern crate nearfield_sys;

use ::Context;
use ::Error;

type InternalDevice = nearfield_sys::nfc_device;
pub type ConnectionString = nearfield_sys::nfc_connstring;

pub struct Device {
    context: *mut Context,
    device: *mut InternalDevice,
}

impl Device {
    pub fn new(context: *mut Context, mut conn_string: ConnectionString) -> Result<Device, Error> {
        let device = unsafe { nearfield_sys::nfc_open(context, conn_string.as_mut_ptr()) };

        if device.is_null() {
            return Err(Error::OpenDevice);
        }

        return Ok(Device {
            context,
            device,
        });
    }
}