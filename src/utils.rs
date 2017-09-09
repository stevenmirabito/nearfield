use error::Error;
use error::NFCResult;

pub fn status_to_result(sys_result: i32) -> NFCResult {
    let error = Error::from_sys(sys_result);

    match error {
        Error::Success => Ok(()),
        _ => Err(error),
    }
}