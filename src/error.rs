use hidapi::HidError;

pub type CosmicboxResult<T> = Result<T, CosmicboxError>;

#[derive(Debug)]
pub enum CosmicboxError {
    ProtocolError(&'static str),
}

impl From<HidError> for CosmicboxError {
    fn from(error: HidError) -> CosmicboxError {
        match error {
            _ => CosmicboxError::ProtocolError("default"),
        }
    }
}
