use error::Error;
use error::NFCResult;

pub(crate) fn status_to_result(sys_error: i32) -> NFCResult<()> {
    if sys_error == 0 {
        return Ok(());
    }

    Err(match sys_error {
        nearfield_sys::NFC_ECHIP => Error::Chip,
        nearfield_sys::NFC_EDEVNOTSUPP => Error::NotSupported,
        nearfield_sys::NFC_EINVARG => Error::InvalidArgument,
        nearfield_sys::NFC_EIO => Error::InputOutput,
        nearfield_sys::NFC_EMFCAUTHFAIL => Error::MifareAuthFailure,
        nearfield_sys::NFC_ENOTIMPL => Error::NotImplemented,
        nearfield_sys::NFC_ENOTSUCHDEV => Error::NoSuchDevice,
        nearfield_sys::NFC_EOPABORTED => Error::OperationAborted,
        nearfield_sys::NFC_EOVFLOW => Error::BufferOverflow,
        nearfield_sys::NFC_ERFTRANS => Error::Transmission,
        nearfield_sys::NFC_ESOFT => Error::Software,
        nearfield_sys::NFC_ETGRELEASED => Error::TargetReleased,
        nearfield_sys::NFC_ETIMEOUT => Error::TimedOut,
        _ => Error::Unknown,
    })
}
