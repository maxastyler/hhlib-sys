use crate::bindings::*;
use crate::error_enum_or_value;
use crate::types::HydraHarpError;
use crate::types::HydraHarpError::*;

/// Contains the information of the device - the number it is (0 -> 7) and the serial of it.
#[derive(Debug, PartialEq)]
pub struct Device {
    id: i32,
    serial: [i8; 8],
}

impl Device {
    /// Try to open a device given a device id and return a result with either the opened device or an error
    pub fn open_device(id: i32) -> Result<Device, HydraHarpError> {
        let mut serial = [0i8; 8];
        return error_enum_or_value! {
            unsafe {
                HH_OpenDevice(id, serial.as_mut_ptr())},
                Device {id, serial}
        };
    }
}
