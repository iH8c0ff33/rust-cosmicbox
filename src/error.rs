use hidapi::HidError;

pub type CosmicBoxResult<T> = Result<T, CosmicboxError>;

#[derive(Debug)]
pub enum CosmicboxError {
    ProtocolError(&'static str),
}

impl From<HidError> for CosmicboxError {
    fn from(error: HidError) -> CosmicboxError {
        match error {
            _ => CosmicboxError::ProtocolError("generic protocol error"),
        }
    }
}
