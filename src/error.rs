use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UploadError {
    message: String,
}

impl UploadError {
    pub fn new(message: &str) -> UploadError {
        UploadError {
            message: String::from(message),
        }
    }
}

impl Display for UploadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for UploadError {}
unsafe impl Send for UploadError {}