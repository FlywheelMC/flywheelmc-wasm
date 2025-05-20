use crate::sig::ImportFuncs;
use crate::runner::WasmCallCtx;
use crate::types::{ WasmAnyPtr, WasmResult };
use super::define;
use flywheelmc_common::prelude::*;
use flywheelmc_players::world::{ WorldChunkActionEvent, WorldChunkAction };


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_world_set_blocks,);
}


async fn flywheel_world_set_blocks(
    ctx        : WasmCallCtx<'_>,
    session_id : u64,
    data_ptr   : WasmAnyPtr
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let mut ptr = data_ptr;

        let count = ctx.mem_read(ptr.assume_type::<u32>())?;
        ptr.shift_mut(4);

        let mut blocks = Vec::with_capacity(count as usize);
        for _ in 0..count {

            let x = ctx.mem_read(ptr.assume_type::<i64>())?;
            ptr.shift_mut(8);
            let y = ctx.mem_read(ptr.assume_type::<i64>())?;
            ptr.shift_mut(8);
            let z = ctx.mem_read(ptr.assume_type::<i64>())?;
            ptr.shift_mut(8);

            let block_id_len = ctx.mem_read(ptr.assume_type::<u32>())?;
            ptr.shift_mut(4);
            let block_id = ctx.mem_read_str(ptr, block_id_len)?.to_string();
            ptr.shift_mut(block_id_len);

            let states_count = ctx.mem_read(ptr.assume_type::<u8>())?;
            ptr.shift_mut(1);

            let mut states = Vec::with_capacity(states_count as usize);
            for _ in 0..states_count {

                let state_len = ctx.mem_read(ptr.assume_type::<u32>())?;
                ptr.shift_mut(4);
                let state = ctx.mem_read_str(ptr, state_len)?.to_string();
                ptr.shift_mut(state_len);

                let value_len = ctx.mem_read(ptr.assume_type::<u32>())?;
                ptr.shift_mut(4);
                let value = ctx.mem_read_str(ptr, value_len)?.to_string();
                ptr.shift_mut(value_len);

                states.push((state, value,));
            }
            blocks.push((Vec3::new(x, y, z), block_id, states,));
        }

        let _ = AsyncWorld.send_event(WorldChunkActionEvent {
            entity,
            action : WorldChunkAction::Set { blocks }
        });
    }
    Ok(())
}
