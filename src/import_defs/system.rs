use crate::sig::ImportFuncs;
use crate::runner::WasmCallCtx;
use crate::types::{ WasmPtr, WasmAnyPtr, WasmResult };
use super::define;
use flywheelmc_common::prelude::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_system_dur_since_epoch,);
    define!(import_funcs, flywheel_system_players,);
    define!(import_funcs, flywheel_system_queue_stop,);
}


static LAST_TIME : Mutex<SystemTime> = Mutex::new(SystemTime::UNIX_EPOCH);

async fn flywheel_system_dur_since_epoch(
    mut ctx       : WasmCallCtx<'_>,
        out_secs  : WasmPtr<u64>,
        out_nanos : WasmPtr<u32>
) -> WasmResult<()> {
    let     now  = SystemTime::now();
    let mut last = LAST_TIME.lock().await;
    let monotonic_now = if (now > *last) {
        *last = now;
        now
    } else {
        *last
    };
    drop(last);
    let dur = monotonic_now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::ZERO);
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
