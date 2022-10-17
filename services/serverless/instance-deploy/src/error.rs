#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl std::fmt::Display) -> Self {
        Self {
            message: format!("[Error] {}", message)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl<E: std::fmt::Debug> From<aws_sdk_elasticbeanstalk::types::SdkError<E>> for Error {
    fn from(err: aws_sdk_elasticbeanstalk::types::SdkError<E>) -> Self {
        Error::new(format!("[Elastic Beanstalk]: {:?}", err))
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<Box<T>> for Error {
    fn from(err: Box<T>) -> Self {
        Error::new(format!("[Error]: {:?}", err))
        
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new(format!("[Request Error] {:?}", err))
    }
}
