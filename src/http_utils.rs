use std::fmt::{Display, Formatter};
use crate::err::{ErrorCode, HttpError};
use crate::wellknown::{METHOD_CONNECT, METHOD_DELETE, METHOD_GET, METHOD_HEAD, METHOD_OPTIONS, METHOD_PATCH, METHOD_POST, METHOD_PUT, METHOD_TRACE};

pub fn err_msg(code: ErrorCode, message: &str) -> HttpError {
    HttpError {
        code,
        message: message.to_owned(),
        context: None,
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub enum HttpMethod {
    Options,
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl HttpMethod {
    pub fn from(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            METHOD_OPTIONS => Some(HttpMethod::Options),
            METHOD_GET => Some(HttpMethod::Get),
            METHOD_POST => Some(HttpMethod::Post),
            METHOD_PUT => Some(HttpMethod::Put),
            METHOD_DELETE => Some(HttpMethod::Delete),
            METHOD_HEAD => Some(HttpMethod::Head),
            METHOD_TRACE => Some(HttpMethod::Trace),
            METHOD_CONNECT => Some(HttpMethod::Connect),
            METHOD_PATCH => Some(HttpMethod::Patch),
            _ => None,
        }
    }
}
impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Options => f.write_str(METHOD_OPTIONS),
            HttpMethod::Get => f.write_str(METHOD_GET),
            HttpMethod::Post => f.write_str(METHOD_POST),
            HttpMethod::Put => f.write_str(METHOD_PUT),
            HttpMethod::Delete => f.write_str(METHOD_DELETE),
            HttpMethod::Head => f.write_str(METHOD_HEAD),
            HttpMethod::Trace => f.write_str(METHOD_TRACE),
            HttpMethod::Connect => f.write_str(METHOD_CONNECT),
            HttpMethod::Patch => f.write_str(METHOD_PATCH),
        }
    }
}
