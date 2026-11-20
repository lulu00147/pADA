#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
mod arrow_c_data;
pub use arrow_c_data::{ArrowArray, ArrowSchema};
#[allow(clippy::all, unsafe_op_in_unsafe_fn)]
mod bindings {
    use crate::arrow_c_data::{ArrowArray, ArrowSchema};
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
#[allow(clippy::all)]
pub use bindings::*;
mod string;
pub use string::*;
pub const DuckDBError: duckdb_state = duckdb_state_DuckDBError;
pub const DuckDBSuccess: duckdb_state = duckdb_state_DuckDBSuccess;
pub use self::error::*;
mod error;
