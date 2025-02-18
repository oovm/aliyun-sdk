#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod application;
mod client;

pub use crate::{
    application::{Application, ApplicationOutput, ApplicationResponse, ApplicationUsage, DashModels, ResponseFormat},
    client::DashScope,
};
pub use aliyun_error::{AliError, AliErrorKind};
