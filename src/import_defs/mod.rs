use crate::sig::{ ImportFuncs, WasmCallCtx };
use crate::types::{ WasmResult, WasmPtr, WasmAnyPtr, TransactionId };
use flywheelmc_common::prelude::*;


mod system;
mod rand;
mod player;
mod profile;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    system::define_all(import_funcs);
    rand::define_all(import_funcs);
    player::define_all(import_funcs);
    profile::define_all(import_funcs);
}

macro define( $import_funcs:expr, $func:ident $(,)? ) {
    $import_funcs.define( stringify!( $func ), $func, );
}
