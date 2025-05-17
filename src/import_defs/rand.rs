use super::*;


pub fn define_all(import_funcs : &mut ImportFuncs) {
    define!(import_funcs, flywheel_rand_u64,);
}


async fn flywheel_rand_u64(
    _ctx : WasmCallCtx<'_>,
) -> WasmResult<u64> {
    Ok(random::<u64>())
}
