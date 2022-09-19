use reqwest::Error;

#[derive(Debug)]
pub(crate) enum GetTemplateError {
    ReqwestError(Error),
    NoMediaFound,
}

impl From<Error> for GetTemplateError {
    fn from(error: Error) -> Self {
        GetTemplateError::ReqwestError(error)
    }
}
