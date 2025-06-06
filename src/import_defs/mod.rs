use crate::sig::ImportFuncs;
use crate::types::{ WasmResult, WasmPtr, WasmAnyPtr };
use crate::runner::WasmCallCtx;
use flywheelmc_common::prelude::*;


mod system;
mod rand;
mod player;
mod profile;
mod world;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_refuel,);
    define!(import_funcs, flywheel_next_event,);
    system::define_all(import_funcs);
    rand::define_all(import_funcs);
    player::define_all(import_funcs);
    profile::define_all(import_funcs);
    world::define_all(import_funcs);
}

macro define( $import_funcs:expr, $func:ident $(,)? ) {
    $import_funcs.define( stringify!( $func ), $func, );
}


async fn flywheel_refuel(
    mut ctx : WasmCallCtx<'_>
) -> WasmResult<()> {
    ctx.refuel();
    task::yield_now().await;
    Ok(())
}


async fn flywheel_next_event(
    mut ctx          : WasmCallCtx<'_>,
        out_id_ptr   : WasmPtr<WasmAnyPtr>,
        out_id_len   : WasmPtr<u32>,
        out_args_ptr : WasmPtr<WasmAnyPtr>,
        out_args_len : WasmPtr<u32>
) -> WasmResult<u32> {
    if let Some((id, args,)) = ctx.next_event().await {
        let id_len = id.len();
        let id_ptr = ctx.mem_alloc_write_str(id).await?;
        ctx.mem_write(out_id_ptr, id_ptr)?;
        ctx.mem_write(out_id_len, id_len as u32)?;
        let args_len = args.len();
        let args_ptr = ctx.mem_alloc_write_any(&args).await?;
        ctx.mem_write(out_args_ptr, args_ptr)?;
        ctx.mem_write(out_args_len, args_len as u32)?;
        Ok(1)
    } else { Ok(0) }
}
