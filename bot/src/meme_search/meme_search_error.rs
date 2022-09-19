use httpstatus::StatusCode;
use crate::meme_search::get_template_error::GetTemplateError;
use worker::Response;
use crate::handle_interactivity::status_code_to_response_error;

#[derive(Debug)]
pub(crate) enum MemeSearchError {
    ReqwestError(reqwest::Error),
    ParseError(GetTemplateError),
}

impl From<reqwest::Error> for MemeSearchError {
    fn from(error: reqwest::Error) -> Self {
        MemeSearchError::ReqwestError(error)
    }
}

impl From<GetTemplateError> for MemeSearchError {
    fn from(error: GetTemplateError) -> Self {
        MemeSearchError::ParseError(error)
    }
}

impl Into<worker::Result<Response>> for MemeSearchError {
    fn into(self) -> worker::Result<Response> {
        status_code_to_response_error(StatusCode::InternalServerError)
    }
}
