use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use bytes::Bytes;
use http::StatusCode;
use log::{info, warn};
use rapid_fs::vfs::VfsErr;
use serde::{Serialize, Serializer};
use serde_json::json;
use thiserror::Error;
use crate::hypi_rapid_plugin::{InputSequence, PluginError};
// use crate::plugin::hypi_rapid_plugin::{InputSequence, PluginError};
use crate::wellknown::CODE_PIPELINE_PLUGIN_CONN_ERR;

#[derive(Debug, Clone)]
pub struct ErrorCode {
   pub name: String,
  pub  http_status: StatusCode,
}

impl ErrorCode {
    pub fn new(name: &str, status: StatusCode) -> Self {
        Self {
            name: name.to_string(),
            http_status: status,
        }
    }
}
impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.name.as_str())
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub code: ErrorCode,
    pub message: String,
    pub context: Option<HashMap<String, String>>,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap_or_else(|e| {
            warn!("Unable to serialise http error object. {}", e);
            json!({
                "code": crate::wellknown::CODE_JSON_ERR.name,
                "message": "An error occurred but the server fail to serialise it to JSON."
            })
                .to_string()
        });
        f.write_str(json.as_str())
    }
}

impl std::error::Error for HttpError {}

// impl<T> ToOwned for &HttpError<T> where T:Serialize{
//     type Owned = HttpError<T>;
//
//     fn to_owned(&self) -> Self::Owned {
//         HttpError{
//             code: std::mem::replace(&mut self.code,"".to_owned()),
//             message: "".to_string(),
//             context: None,
//         }
//     }
// }

//needed because the cache always returns an Arc<HttpError> from which we can't take the object so we clone it
impl From<&HttpError> for HttpError {
    fn from(value: &HttpError) -> Self {
        Self {
            code: value.code.clone(),
            message: value.message.to_owned(),
            context: value.context.to_owned(),
        }
    }
}
impl From<&HttpError> for Bytes {
    fn from(value: &HttpError) -> Self {
        serde_json::to_string(value)
            .unwrap_or_else(|_| {
                info!(
                    "Error serialising a HttpError object. code: {}, msg: {}",
                    value.code, value.message,
                );
                json!({
                    "code": value.code,
                    "message": value.message,
                    "context": {
                        "hypi_serialisation_error": "Failed to serialise the context for this error"
                    }
                })
                    .to_string()
            })
            .into()
    }
}

impl From<VfsErr> for HttpError {
    fn from(value: VfsErr) -> Self {
        match value {
            VfsErr::Domain(msg) => HttpError {
                code: crate::wellknown::CODE_FS_DOMAIN.to_owned(),
                message: msg,
                context: None,
            },
            VfsErr::FileNotFound(msg) => HttpError {
                code: crate::wellknown::CODE_FS_FILE_NOT_FOUND.to_owned(),
                message: msg,
                context: None,
            },
            VfsErr::SchemaFileNotFound(msg) => HttpError {
                code: crate::wellknown::CODE_FS_SCHEMA_FILE_NOT_FOUND.to_owned(),
                message: msg,
                context: None,
            },
            VfsErr::AbsolutePathNotSupported(e) => HttpError {
                code: crate::wellknown::CODE_FS_ABSOLUTE_PATH_NOT_SUPPORTED.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            VfsErr::DotPathsNotSupported(e) => HttpError {
                code: crate::wellknown::CODE_FS_DOT_PATHS_NOT_SUPPORTED.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            VfsErr::JsonErr(e) => HttpError {
                code: crate::wellknown::CODE_JSON_ERR.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            VfsErr::Io(e) => HttpError {
                code: crate::wellknown::CODE_FS_IO_ERROR.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            VfsErr::Utf8(e) => HttpError {
                code: crate::wellknown::CODE_INVALID_UTF8.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            VfsErr::StripPrefixErr(e) => HttpError {
                code: crate::wellknown::CODE_FS_IO_ERROR.to_owned(),
                message: format!("{}", e),
                context: None,
            },
        }
    }
}
impl From<PipelineError> for HttpError {
    fn from(value: PipelineError) -> Self {
        match value {
            PipelineError::RapidScript(e) => match e {
                RapidScriptError::SyntaxErr { msg, pos, exp } => HttpError {
                    code: crate::wellknown::CODE_RAPID_SCRIPT_SYNTAX_ERR.to_owned(),
                    message: msg,
                    context: Some(HashMap::from([
                        ("column".to_string(), pos.to_string()),
                        ("script".to_owned(), exp), //todo is this safe to return?
                    ])),
                },
                RapidScriptError::EvalErr(msg) => HttpError {
                    code: crate::wellknown::CODE_RAPID_SCRIPT_ERR.to_owned(),
                    message: msg,
                    context: None,
                },
            },
            PipelineError::UnsupportedProtocol(msg) => HttpError {
                code: crate::wellknown::CODE_UNSUPPORTED_PROTOCOL.to_owned(),
                message: msg,
                context: None,
            },
            PipelineError::Json(e) => HttpError {
                code: crate::wellknown::CODE_JSON_ERR.to_owned(),
                message: format!("{}", e),
                context: None,
            },
            PipelineError::BoaScript(msg) => HttpError {
                code: crate::wellknown::CODE_SCRIPT_ERR.to_owned(),
                message: msg,
                context: None,
            },
            PipelineError::Docker(e) =>
            //we could break the docker error down but probably not a good idea to be too detailed about this
                {
                    HttpError {
                        code: crate::wellknown::CODE_DOCKER_ERR.to_owned(),
                        message: format!("Internal error. {}", e.to_string()),
                        context: None,
                    }
                }
            PipelineError::DockerConn(e) => HttpError {
                code: crate::wellknown::CODE_DOCKER_ERR.to_owned(),
                message: format!("Internal error. {}", e.to_string()),
                context: None,
            },
            PipelineError::Vfs(vfs) => HttpError {
                code: crate::wellknown::CODE_FORM_FILE_ERR.to_owned(),
                message: format!("{}", vfs),
                context: None,
            },
            PipelineError::EmptyPipeline => HttpError {
                code: crate::wellknown::CODE_PIPELINE_EMPTY.to_owned(),
                message: "Server error, configured pipeline has no steps to execute.".to_string(),
                context: None,
            },
            PipelineError::PluginStatusErr(e) => {
                let code =crate::wellknown::GRPC_ERRS.get(&e.code()).unwrap().to_owned();
                // let code = grpc_code2status(e.code());
                // let http_status = StatusCode::from_u16(code).unwrap();
                HttpError {
                    code, /*ErrorCode::of(
                              format!("{}{}", CODE_PIPELINE_PLUGIN_STATUS_PREFIX, code,).as_str(),
                              http_status,
                          )*/
                    message: format!("Internal error. {}", e.to_string()),
                    context: None,
                }
            }
            PipelineError::PluginChannelErr(e) => HttpError {
                code: ErrorCode::new(
                    CODE_PIPELINE_PLUGIN_CONN_ERR,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
                message: format!("Internal error. {}", e.to_string()),
                context: None,
            },
            PipelineError::DuplicateSequenceId => HttpError {
                code: crate::wellknown::CODE_PIPELINE_DUPLICATE_SEQ_ID.clone(),
                message: "Internal error.".to_string(),
                context: None,
            },
            PipelineError::PluginErr(msg, plugin) => HttpError {
                code: crate::wellknown::CODE_PIPELINE_PLUGIN_BEHAVIOUR.clone(),
                message: msg,
                context: Some(HashMap::from([("plugin".into(), plugin)])),
            },
            PipelineError::PluginSeqErr(e) => HttpError {
                code: ErrorCode {
                    name: e.code.clone(),
                    http_status: StatusCode::from_u16(e.status as u16).unwrap(),
                },
                message: e.message,
                context: Some(
                    e.context
                        .iter()
                        .map(|e| {
                            (
                                e.key.to_owned(),
                                e.value
                                    .first()
                                    .map(|v| v.to_owned())
                                    .unwrap_or_default()
                                    .to_string(),
                            )
                        })
                        .collect::<HashMap<String, String>>(),
                ),
            },
        }
    }
}


#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Error parsing rapid script expression. {0}")]
    RapidScript(RapidScriptError),
    #[error("Protocol not supported. {0}")]
    UnsupportedProtocol(String),
    #[error("Error parsing JSON. {0}")]
    Json(serde_json::Error),
    #[error("JavaScript error. {0}")]
    BoaScript(String),
    #[error("Docker error. {0}")]
    Docker(bollard::errors::Error),
    #[error("Docker error. {0}")]
    DockerConn(tonic::transport::Error),
    #[error("Plugin request error.")]
    PluginStatusErr(tonic::Status),
    #[error("File error. {0}")]
    Vfs(VfsErr),
    #[error("Invalid pipeline config.")]
    EmptyPipeline,
    #[error("Duplicate sequence ID.")]
    DuplicateSequenceId,
    #[error("Plugin Channel error. {0}")]
    PluginChannelErr(tokio::sync::mpsc::error::SendError<InputSequence>),
    #[error("Plugin misbehaved!. {0} - {1}")]
    PluginErr(String, String),
    #[error("Plugin HTTP error. {0}")]
    PluginSeqErr(PluginError),
}

#[derive(Debug, Error)]
pub enum RapidScriptError {
    #[error("Invalid syntax. {msg}")]
    SyntaxErr { msg: String, pos: i32, exp: String },
    #[error("Error evaluating RAPID script. {0}")]
    EvalErr(String),
}


impl Display for PluginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        {
            f.write_str("status:")?;
            f.write_str(self.status.to_string().as_str())?;
            f.write_str(",")?;
        }
        {
            f.write_str("code:")?;
            f.write_str(self.code.to_string().as_str())?;
            f.write_str(",")?;
        }
        {
            f.write_str("message:")?;
            f.write_str(self.message.to_string().as_str())?;
            f.write_str(",")
        }
        // {
        //     f.write_str("context:")?;
        //     f.write_str(self.context.to_string().as_str())?;
        //     f.write_str("")
        // }
    }
}
