use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_dur_since_epoch,);
    define!(import_funcs, flywheel_players,);
    define!(import_funcs, flywheel_queue_stop,);
}


async fn flywheel_dur_since_epoch(secs : WasmPtr<u64>, nanos : WasmPtr<u32>) -> WasmResult<()> {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);
    secs.write(&dur.as_secs())?;
    nanos.write(&dur.subsec_nanos())?;
    Ok(())
}


async fn flywheel_players(session_ids : WasmPtr<WasmAnyPtr>) -> WasmResult<u32> {
    todo!();
}


async fn flywheel_queue_stop() -> WasmResult<()> {
    todo!();
}
