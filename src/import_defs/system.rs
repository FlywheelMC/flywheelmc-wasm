use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_system_next_event,);
    define!(import_funcs, flywheel_system_dur_since_epoch,);
    define!(import_funcs, flywheel_system_players,);
    define!(import_funcs, flywheel_system_queue_stop,);
}


async fn flywheel_system_next_event(
    out_id_ptr   : WasmPtr<WasmAnyPtr>,
    out_id_len   : WasmPtr<u32>,
    out_args_ptr : WasmPtr<WasmAnyPtr>,
    out_args_len : WasmPtr<u32>
) -> WasmResult<u32> {
    // TODO: Next event
    return Ok(0);
}


async fn flywheel_system_dur_since_epoch(out_secs : WasmPtr<u64>, out_nanos : WasmPtr<u32>) -> WasmResult<()> {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);
    out_secs.write(&dur.as_secs())?;
    out_nanos.write(&dur.subsec_nanos())?;
    Ok(())
}


async fn flywheel_system_players(_out_session_ids : WasmPtr<WasmAnyPtr>) -> WasmResult<u32> {
    todo!();
}


async fn flywheel_system_queue_stop() -> WasmResult<()> {
    todo!();
}
