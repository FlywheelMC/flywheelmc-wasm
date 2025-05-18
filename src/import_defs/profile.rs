use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_profile_from_session,);
}


async fn flywheel_profile_from_session(
    _ctx          : WasmCallCtx<'_>,
    _session_id   : u64,
    _out_uuid     : WasmPtr<u128>,
    _out_name_ptr : WasmPtr<WasmAnyPtr>,
    _out_name_len : WasmPtr<u32>
) -> WasmResult<u32> {
    todo!()
}
