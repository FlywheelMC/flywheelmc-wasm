use crate::sig::ImportFuncs;
use crate::types::{ WasmResult, WasmPtr, WasmAnyPtr };
use flywheelmc_common::prelude::*;


pub mod system;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    system::define_all(import_funcs);
}

macro define( $import_funcs:expr, $func:ident $(,)? ) {
    $import_funcs.define( stringify!( $func ), $func )
}
