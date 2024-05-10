include!(concat!(env!("OUT_DIR"), "/plugin.rs"));

pub mod http_utils;
pub mod wellknown;
pub mod err;
pub use hypi_rapid_plugin as plugin;
