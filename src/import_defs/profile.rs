use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_profile_by_session,);
    define!(import_funcs, flywheel_profile_by_uuid,);
    define!(import_funcs, flywheel_profile_by_username,);
}


async fn flywheel_profile_by_session(
    _ctx              : WasmCallCtx<'_>,
    _session_id       : u64,
    _out_uuid         : WasmPtr<u128>,
    _out_username_ptr : WasmPtr<WasmAnyPtr>,
    _out_username_len : WasmPtr<u32>
) -> WasmResult<u32> {
    todo!()
}

async fn flywheel_profile_by_uuid(
    _ctx     : WasmCallCtx<'_>,
    _in_uuid : WasmPtr<u128>
) -> WasmResult<TransactionId> {
    todo!()
}

async fn flywheel_profile_by_username(
    _ctx          : WasmCallCtx<'_>,
    _in_username  : WasmAnyPtr,
    _username_len : u32
) -> WasmResult<TransactionId> {
    todo!()
}
