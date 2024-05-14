use std::collections::HashMap;

use http::StatusCode;
use lazy_static::lazy_static;
use tonic::Code;

use crate::err::ErrorCode;

pub const CODE_PIPELINE_PLUGIN_STATUS_PREFIX: &str = "hypi_pipeline_plugin_status_";
pub const CODE_PIPELINE_PLUGIN_CONN_ERR: &str = "hypi_pipeline_plugin_conn_err";
pub const METHOD_OPTIONS: &str = "OPTIONS";
pub const METHOD_GET: &str = "GET";
pub const METHOD_POST: &str = "POST";
pub const METHOD_PUT: &str = "PUT";
pub const METHOD_DELETE: &str = "DELETE";
pub const METHOD_HEAD: &str = "HEAD";
pub const METHOD_TRACE: &str = "TRACE";
pub const METHOD_CONNECT: &str = "CONNECT";
pub const METHOD_PATCH: &str = "PATCH";
pub const FORM_URL_ENCODED_HDR: &str = "application/x-www-form-urlencoded";
pub const APPLICATION_JSON_HDR: &str = "application/json";
///The HTTP method doesn't fit into other standard fields so it is captured by RAPID and put in as a meta field called method
pub const HTTP_METHOD_META: &str = "method";
///The path where the assets for a service is available in a plugin's container.
pub const ASSETS_DIR: &str = "/home/rapid/files";
///The path where RAPID server uploads/saves temporary files - it is up to plugins to move the files to a permanent location. RAPID automatically deletes data in this directory periodically
pub const ASSETS_TMP_DIR: &str = "/home/rapid/files/.tmp";
pub const HDR_CONTENT_TYPE: &str = "content-type";
pub const HDR_HOST: &str = "host";
pub const HDR_STATUS: &str = "status";
lazy_static! {
    pub static ref CODE_UNKNOWN_DOMAIN: ErrorCode =
        ErrorCode::new("hypi_domain_not_found", StatusCode::NOT_FOUND);
    pub static ref CODE_FAILED_NO_CONTENT: ErrorCode =
        ErrorCode::new("hypi_failed_no_content", StatusCode::BAD_REQUEST);
    pub static ref CODE_FAILED_TO_BUILD_RESPONSE: ErrorCode =
        ErrorCode::new("hypi_failed_to_build_response", StatusCode::BAD_REQUEST);
    pub static ref CODE_ENDPOINT_INVALID_REGEX: ErrorCode =
        ErrorCode::new("hypi_endpoint_invalid_regex", StatusCode::BAD_REQUEST);
    pub static ref CODE_MISSING_METHOD: ErrorCode =
        ErrorCode::new("hypi_missing_method", StatusCode::BAD_REQUEST);
    pub static ref CODE_MISSING_HOST: ErrorCode =
        ErrorCode::new("hypi_missing_host", StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_SCHEMA_FILE_NOT_FOUND: ErrorCode =
        ErrorCode::new("hypi_schema_file_not_found", StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_ABSOLUTE_PATH_NOT_SUPPORTED: ErrorCode =
        ErrorCode::new("hypi_absolute_path_not_supported", StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_DOT_PATHS_NOT_SUPPORTED: ErrorCode =
        ErrorCode::new("hypi_dot_path_not_supported", StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_FILE_NOT_FOUND: ErrorCode =
        ErrorCode::new("hypi_file_not_found", StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_DOMAIN: ErrorCode =
        ErrorCode::new("hypi_domain_not_found", StatusCode::BAD_REQUEST);
    pub static ref CODE_RAPID_SCRIPT_ERR: ErrorCode =
        ErrorCode::new("hypi_rapid_script_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_RAPID_SCRIPT_SYNTAX_ERR: ErrorCode =
        ErrorCode::new("hypi_rapid_script_syntax_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_UNSUPPORTED_PROTOCOL: ErrorCode = ErrorCode::new(
        "hypi_pipeline_unsupported_protocol",
        StatusCode::BAD_REQUEST,
    );
    pub static ref CODE_PIPELINE_NOT_FOUND: ErrorCode =
        ErrorCode::new("hypi_pipeline_not_found", StatusCode::INTERNAL_SERVER_ERROR);
    pub static ref CODE_PIPELINE_EMPTY: ErrorCode =
        ErrorCode::new("hypi_pipeline_empty", StatusCode::INTERNAL_SERVER_ERROR);
    pub static ref CODE_PIPELINE_EL_UNSUPPORTED: ErrorCode = ErrorCode::new(
        "hypi_pipeline_unsupported_element",
        StatusCode::NOT_IMPLEMENTED,
    );
    pub static ref CODE_SCHEMA_INVALID: ErrorCode =
        ErrorCode::new("hypi_schema_invalid", StatusCode::BAD_REQUEST);
    pub static ref CODE_SEMANTICS_INVALID_SCHEMA_ROOT: ErrorCode = ErrorCode::new(
        "hypi_semantics_invalid_schema_root",
        StatusCode::BAD_REQUEST,
    );
    pub static ref CODE_SEMANTICS_DUPLICATE_TABLE: ErrorCode =
        ErrorCode::new("hypi_semantics_duplicate_table", StatusCode::BAD_REQUEST);
    pub static ref CODE_SEMANTICS_TABLE_NOT_FOUND: ErrorCode =
        ErrorCode::new("hypi_semantics_table_not_found", StatusCode::BAD_REQUEST);

    // pub static ref CODE_JSON_PARSE_ERR: ErrorCode = ErrorCode::new("hypi_json_parse_err",StatusCode::BAD_REQUEST);
    pub static ref CODE_FS_IO_ERROR: ErrorCode =
        ErrorCode::new("hypi_io_error", StatusCode::BAD_REQUEST);
    pub static ref CODE_INVALID_UTF8: ErrorCode =
        ErrorCode::new("hypi_invalid_utf8", StatusCode::BAD_REQUEST);
    pub static ref CODE_HTTP_INVALID_HEADER: ErrorCode =
        ErrorCode::new("hypi_http_invalid_header", StatusCode::BAD_REQUEST);
    pub static ref CODE_HTTP_INVALID_URI: ErrorCode =
        ErrorCode::new("hypi_http_invalid_uri", StatusCode::BAD_REQUEST);
    pub static ref CODE_HTTP_MISSING_HOST: ErrorCode =
        ErrorCode::new("hypi_http_missing_host", StatusCode::BAD_REQUEST);
    pub static ref CODE_HTTP_HYPER: ErrorCode =
        ErrorCode::new("hypi_http_hyper", StatusCode::BAD_REQUEST);
    pub static ref CODE_HTTP_IO: ErrorCode = ErrorCode::new("hypi_http_io_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_JSON_ERR: ErrorCode = ErrorCode::new("hypi_json_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_UNSUPPORTED: ErrorCode =
        ErrorCode::new("hypi_sql_unsupported_db_type", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_NO_CONNECTION: ErrorCode =
        ErrorCode::new("hypi_sql_no_connection", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_UNABLE_TO_CONNECT: ErrorCode =
        ErrorCode::new("hypi_sql_connection_failed", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_MEKADB_UNABLE_TO_CONNECT: ErrorCode =
        ErrorCode::new("hypi_sql_mekadb_connection_failed", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_MEKADB_REQ_ERR: ErrorCode =
        ErrorCode::new("hypi_sql_mekadb_req_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_SQL_MEKADB_RES_ERR: ErrorCode =
        ErrorCode::new("hypi_sql_mekadb_res_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_SCRIPT_ERR: ErrorCode =
        ErrorCode::new("hypi_script_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_PIPELINE_NO_STEPS: ErrorCode =
        ErrorCode::new("hypi_pipeline_no_steps", StatusCode::BAD_REQUEST);
    pub static ref CODE_PIPELINE_DUPLICATE_SEQ_ID: ErrorCode = ErrorCode::new(
        "hypi_pipeline_duplicate_seq_id",
        StatusCode::INTERNAL_SERVER_ERROR,
    );
    pub static ref CODE_PIPELINE_PLUGIN_BEHAVIOUR: ErrorCode = ErrorCode::new(
        "hypi_pipeline_bad_behaviour",
        StatusCode::INTERNAL_SERVER_ERROR,
    );
    pub static ref CODE_FORM_FILE_ERR: ErrorCode =
        ErrorCode::new("hypi_form_file_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_FORM_PAYLOAD_ERR: ErrorCode =
        ErrorCode::new("hypi_form_payload_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_FORM_IO_ERR: ErrorCode =
        ErrorCode::new("hypi_form_io_err", StatusCode::BAD_REQUEST);
    pub static ref CODE_DOCKER_ERR: ErrorCode =
        ErrorCode::new("hypi_docker_err", StatusCode::INTERNAL_SERVER_ERROR);

        pub static ref GRPC_ERRS: HashMap<Code, ErrorCode> = HashMap::from([
            (
                Code::Ok,
                ErrorCode::new("hypi_pipeline_plugin_status_ok", StatusCode::OK)
            ),
            (
                Code::Cancelled,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_cancelled",
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            ),
            (
                Code::Unknown,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_unknown",
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            ),
            (
                Code::InvalidArgument,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_invalidargument",
                    StatusCode::BAD_REQUEST
                )
            ),
            (
                Code::DeadlineExceeded,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_deadlineexceeded",
                    StatusCode::GATEWAY_TIMEOUT
                )
            ),
            (
                Code::NotFound,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_notfound",
                    StatusCode::NOT_FOUND
                )
            ),
            (
                Code::AlreadyExists,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_alreadyexists",
                    StatusCode::CONFLICT
                )
            ),
            (
                Code::PermissionDenied,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_permissiondenied",
                    StatusCode::FORBIDDEN
                )
            ),
            (
                Code::Unauthenticated,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_unauthenticated",
                    StatusCode::UNAUTHORIZED
                )
            ),
            (
                Code::ResourceExhausted,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_resourceexhausted",
                    StatusCode::TOO_MANY_REQUESTS
                )
            ),
            (
                Code::FailedPrecondition,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_failedprecondition",
                    StatusCode::PRECONDITION_FAILED
                )
            ),
            (
                Code::Aborted,
                ErrorCode::new("hypi_pipeline_plugin_status_aborted", StatusCode::GONE)
            ),
            (
                Code::OutOfRange,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_outofrange",
                    StatusCode::RANGE_NOT_SATISFIABLE
                )
            ),
            (
                Code::Unimplemented,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_unimplemented",
                    StatusCode::NOT_IMPLEMENTED
                )
            ),
            (
                Code::Internal,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_internal",
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            ),
            (
                Code::Unavailable,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_unavailable",
                    StatusCode::SERVICE_UNAVAILABLE
                )
            ),
            (
                Code::DataLoss,
                ErrorCode::new(
                    "hypi_pipeline_plugin_status_dataloss",
                    StatusCode::INSUFFICIENT_STORAGE
                )
            ),
        ]);
    }
/*    pub fn grpc_code2status(code: Code) -> u16 {
        match code {
            Code::Ok => 200,
            Code::Cancelled => 299,
            Code::Unknown => 500,
            Code::InvalidArgument => 400,
            Code::DeadlineExceeded => 504,
            Code::NotFound => 404,
            Code::AlreadyExists => 409,
            Code::PermissionDenied => 403,
            Code::ResourceExhausted => 429,
            Code::FailedPrecondition => 412,
            Code::Aborted => 299,
            Code::OutOfRange => 416,
            Code::Unimplemented => 501,
            Code::Internal => 500,
            Code::Unavailable => 503,
            Code::DataLoss => 505,
            Code::Unauthenticated => 401,
        }
    }*/
