use crate::runner::WasmRunnerInstance;
use super::*;
use flywheelmc_players::player::Player;
use flywheelmc_players::world::{ WorldChunkActionEvent, WorldChunkAction };


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_world_set_blocks,);
}


async fn flywheel_world_set_blocks(
    mut ctx        : WasmCallCtx<'_>,
        session_id : u64,
        data_ptr   : WasmAnyPtr,
        data_len   : u32
) -> WasmResult<()> {
    if let Some(entity) = ctx.player_session_to_entity(session_id).await {
        let mut ptr = data_ptr;

        let mut count = ctx.mem_read(unsafe { ptr.assume_type::<u32>() })?;
        unsafe { ptr.shift_mut(4); }

        let mut blocks = Vec::with_capacity(count as usize);
        for _ in 0..count {

            let mut x = ctx.mem_read(unsafe { ptr.assume_type::<i64>() })?;
            unsafe { ptr.shift_mut(8); }
            let mut y = ctx.mem_read(unsafe { ptr.assume_type::<i64>() })?;
            unsafe { ptr.shift_mut(8); }
            let mut z = ctx.mem_read(unsafe { ptr.assume_type::<i64>() })?;
            unsafe { ptr.shift_mut(8); }

            let mut block_id_len = ctx.mem_read(unsafe { ptr.assume_type::<u32>() })?;
            unsafe { ptr.shift_mut(4); }
            let mut block_id = ctx.mem_read_str(ptr, block_id_len)?.to_string();
            unsafe { ptr.shift_mut(block_id_len); }

            let mut states_count = ctx.mem_read(unsafe { ptr.assume_type::<u8>() })?;
            unsafe { ptr.shift_mut(4); }

            let mut states = Vec::with_capacity(states_count as usize);
            for _ in 0..states_count {

                let mut state_len = ctx.mem_read(unsafe { ptr.assume_type::<u32>() })?;
                unsafe { ptr.shift_mut(4); }
                let mut state = ctx.mem_read_str(ptr, state_len)?.to_string();
                unsafe { ptr.shift_mut(state_len); }

                let mut value_len = ctx.mem_read(unsafe { ptr.assume_type::<u32>() })?;
                unsafe { ptr.shift_mut(4); }
                let mut value = ctx.mem_read_str(ptr, value_len)?.to_string();
                unsafe { ptr.shift_mut(value_len); }

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
