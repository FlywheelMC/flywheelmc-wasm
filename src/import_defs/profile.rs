use crate::runner::WasmRunnerInstance;
use crate::sig::ImportFuncs;
use crate::runner::WasmCallCtx;
use crate::types::{ WasmPtr, WasmAnyPtr, WasmResult };
use super::define;
use flywheelmc_common::prelude::*;
use flywheelmc_players::player::Player;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_profile_from_session,);
}


async fn flywheel_profile_from_session(
    mut ctx          : WasmCallCtx<'_>,
        session_id   : u64,
        out_uuid     : WasmPtr<u128>,
        out_name_ptr : WasmPtr<WasmAnyPtr>,
        out_name_len : WasmPtr<u32>
) -> WasmResult<u32> {

    let Some(player) = AsyncWorld.query::<(&WasmRunnerInstance,)>().entity(ctx.runner()).get(|(runner,)| {
        runner.players.get_by_left(&session_id).cloned()
    }).ok().flatten()
        else { return Ok(0); };

    let Ok((uuid, name,)) = AsyncWorld.query::<(&Player,)>().entity(player).get(|(player,)| {
        (player.uuid(), player.username().to_string(),)
    }) else { return Ok(0); };

    ctx.mem_write(out_uuid, uuid.as_u128())?;
    let name_len = name.len();
    let name_ptr = ctx.mem_alloc_write_str(&name).await?;
    ctx.mem_write(out_name_ptr, name_ptr)?;
    ctx.mem_write(out_name_len, name_len as u32)?;
    Ok(1)
}
