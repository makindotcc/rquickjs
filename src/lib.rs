//! # High-level bindings to quickjs
//! The `quickrjs` crate provides safe high-level bindings to the [quickjs](https://bellard.org/quickjs/) javascript engine.
//! This crate is heavily inspired by the [rlua](https://crates.io/crates/rlua) crate.
//!
//! # The `Runtime` and `Context` objects
//! The main entry point of this library is the [`Runtime`] struct.
//! It represents the interperter state and is used to create [`Context`]
//! objects. As the quickjs library does not support threading the runtime is locked behind a
//! mutex. Multiple threads cannot run as script or create objects from the same runtime at the
//! same time.
//! The [`Context`] object represents a global environment and a stack. Contexts of the same runtime
//! can share javascript objects like in browser between frames of the same origin.
//!
//! [`Runtime`]: struct.Runtime.html
//! [`Context`]: struct.Context.html

#![allow(clippy::needless_lifetimes)]

use quick_error::quick_error;
use std::{ffi::NulError, str};

mod context;
mod runtime;
pub use context::{Context, ContextBuilder, Ctx};
pub use runtime::Runtime;
mod markers;
mod value;
use std::result::Result as StdResult;
use std::string::String as StdString;
pub use value::*;

quick_error! {
    /// Error type of the library
    #[derive(Debug,Clone,PartialEq)]
    pub enum Error{
        /// A problem with allocation
        Allocation{
            display("Allocation failed while creating object")
        }
        InvalidString(e: NulError){
            display("string contained internal null bytes: {}",e)
            from()
            cause(e)
        }
        Utf8(e: str::Utf8Error){
            display("Conversion from string failed: {}",e)
            from()
            cause(e)
        }
        /// An error from quickjs which we do not know
        /// the specifics about. Should eventually be removed
        Unknown{
            display("quickjs library created a unknown error")
        }
        Exception(e: StdString){
            display("exception generated by quickjs: {}",e)
        }
        FromJsConversion{from: &'static str, to: &'static str, message: Option<StdString>} {
            display("error converting from js from type '{}', to '{}': {}",from,to,message.as_ref().unwrap_or(&StdString::new()))
        }
        ToJsConversion{from: &'static str, to: &'static str, message: Option<StdString>} {
            display("error converting from type '{}', to '{}': {}",from,to,message.as_ref().unwrap_or(&StdString::new()))
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn base_runtime() {
        let _rt = Runtime::new().unwrap();
    }
}
