use flywheelmc_common::prelude::*;
use wasmtime as wt;


mod sig;

mod state;


pub struct FlywheelMcWasmPlugin {
    /// Functions provided to WASM modules for import.
    import_funcs : sig::ImportFuncs
}

impl Plugin for FlywheelMcWasmPlugin {
    fn build(&self, app : &mut App) {
        let     engine = WasmEngine::default();
        let mut linker = WasmLinker(wt::Linker::new(&engine.0));
        self.import_funcs.register(&mut linker.0);
        app
            .insert_resource(engine)
            .insert_resource(linker);
    }
}


#[derive(Resource)]
struct WasmEngine(wt::Engine);
impl Default for WasmEngine {
    fn default() -> Self {
        let mut engine_config = wt::Config::new();
        engine_config
            .async_support(true)
            .wasm_backtrace(true)
            .wasm_backtrace_details(wt::WasmBacktraceDetails::Enable)
            .native_unwind_info(true)
            .consume_fuel(true)
            .epoch_interruption(false)
            .async_stack_zeroing(true);
        engine_config
            .wasm_tail_call(true)
            .wasm_custom_page_sizes(false)
            .wasm_threads(true)
            .wasm_reference_types(true)
            .wasm_function_references(false)
            .wasm_wide_arithmetic(false)
            //.wasm_gc(false)
            .wasm_simd(true)
            .wasm_relaxed_simd(true)
            .relaxed_simd_deterministic(false)
            .wasm_bulk_memory(true)
            .wasm_multi_value(true)
            .wasm_multi_memory(false)
            .wasm_memory64(false)
            .wasm_extended_const(true)
            .wasm_component_model(false);
            //.wasm_component_model_more_flags(false)
            //.wasm_component_model_multiple_returns(false);
            //.wasm_component_model_async(false);
        engine_config
            .strategy(wt::Strategy::Auto)
            .collector(wt::Collector::Auto)
            .profiler(wt::ProfilingStrategy::None)
            .cranelift_opt_level(wt::OptLevel::SpeedAndSize)
            .cranelift_regalloc_algorithm(wt::RegallocAlgorithm::Backtracking)
            .cranelift_nan_canonicalization(true);
        //    .cranelift_pcc(true);
        engine_config
            .cache_config_load_default().unwrap()
            .allocation_strategy(wt::InstanceAllocationStrategy::OnDemand);
        Self(wt::Engine::new(&engine_config).unwrap())
    }
}

#[derive(Resource)]
struct WasmLinker(wt::Linker<state::InstanceState>);
