use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProjectorError {
    msg: String,
}

impl ProjectorError {
    pub fn new(msg: &str) -> ProjectorError {
        ProjectorError {
            msg: msg.to_string(),
        }
    }
}

impl Error for ProjectorError {}

impl fmt::Display for ProjectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProjectorError: {}", self.msg)
    }
}

pub type ProjectorResult<T> = Result<T, Box<dyn Error>>;

#[macro_export]
macro_rules! proj_err {
    ($msg:expr) => {
        Err(Box::new(ProjectorError::new($msg)))
    };
}
