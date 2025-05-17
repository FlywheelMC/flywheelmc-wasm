use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_system_next_event,);
    define!(import_funcs, flywheel_system_dur_since_epoch,);
    define!(import_funcs, flywheel_system_players,);
    define!(import_funcs, flywheel_system_queue_stop,);
}


async fn flywheel_system_next_event(
    _ctx          : WasmCallCtx<'_>,
    _out_id_ptr   : WasmPtr<WasmAnyPtr>,
    _out_id_len   : WasmPtr<u32>,
    _out_args_ptr : WasmPtr<WasmAnyPtr>,
    _out_args_len : WasmPtr<u32>
) -> WasmResult<u32> {
    // TODO: Next event
    return Ok(0);
}


async fn flywheel_system_dur_since_epoch(
    mut ctx       : WasmCallCtx<'_>,
        out_secs  : WasmPtr<u64>,
        out_nanos : WasmPtr<u32>
) -> WasmResult<()> {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);
    ctx.mem_write(out_secs, dur.as_secs())?;
    ctx.mem_write(out_nanos, dur.subsec_nanos())?;
    Ok(())
}


async fn flywheel_system_players(
    _ctx             : WasmCallCtx<'_>,
    _out_session_ids : WasmPtr<WasmAnyPtr>
) -> WasmResult<u32> {
    todo!();
}


async fn flywheel_system_queue_stop(
    _ctx : WasmCallCtx<'_>
) -> WasmResult<()> {
    todo!();
}
