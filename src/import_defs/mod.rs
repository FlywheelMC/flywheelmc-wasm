use crate::sig::{ ImportFuncs, WasmCallCtx };
use crate::types::{ WasmResult, WasmPtr, WasmAnyPtr, TransactionId };
use flywheelmc_common::prelude::*;


mod system;
mod rand;
mod player;
mod profile;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_refuel,);
    define!(import_funcs, flywheel_next_event,);
    system::define_all(import_funcs);
    rand::define_all(import_funcs);
    player::define_all(import_funcs);
    profile::define_all(import_funcs);
}

macro define( $import_funcs:expr, $func:ident $(,)? ) {
    $import_funcs.define( stringify!( $func ), $func, );
}


async fn flywheel_refuel(mut ctx : WasmCallCtx<'_>) -> WasmResult<()> {
    ctx.refuel();
    Ok(())
}


async fn flywheel_next_event(
    _ctx          : WasmCallCtx<'_>,
    _out_id_ptr   : WasmPtr<WasmAnyPtr>,
    _out_id_len   : WasmPtr<u32>,
    _out_args_ptr : WasmPtr<WasmAnyPtr>,
    _out_args_len : WasmPtr<u32>
) -> WasmResult<u32> {
    // TODO: Next event
    Ok(0)
}
