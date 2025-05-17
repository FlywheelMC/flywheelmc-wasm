#![feature(
    // Language
    decl_macro,
    macro_metavar_expr,
    // Standard library
    map_try_insert,
    tuple_trait
)]


use flywheelmc_common::prelude::*;


mod sig;
pub use sig::{ ImportFuncs, WasmCallCtx };

mod state;

mod types;
pub use types::{ WasmPtr, WasmAnyPtr, WasmResult };

mod import_defs;

pub mod runner;


pub const PROTOCOL_VERSION : u64 = 0;


pub struct FlywheelMcWasmPlugin {
    /// Functions provided to WASM modules for import.
    pub import_funcs : sig::ImportFuncs,
    /// Functions that a WASM module can export.
    pub export_funcs : sig::ExportFuncs
}

impl Default for FlywheelMcWasmPlugin {
    fn default() -> Self { Self {

        import_funcs : {
            let mut import_funcs = sig::ImportFuncs::new();
            import_defs::define_all(&mut import_funcs);
            import_funcs
        },

        export_funcs : {
            let mut export_funcs = sig::ExportFuncs::new();
            export_funcs.define::<(), u64>("flywheel_protocol", true);
            export_funcs.define::<(), ()>("flywheel_init", true);
            export_funcs.define::<(u32, u32,), WasmAnyPtr>("flywheel_alloc", true);
            export_funcs
        }

    } }
}

impl Plugin for FlywheelMcWasmPlugin {
    fn build(&self, app : &mut App) {
        let     engine = WasmGlobals::new_engine();
        let mut linker = wt::Linker::new(&engine);
        self.import_funcs.register(&mut linker).unwrap(); // TODO: Handle Err case
        app
            .add_event::<runner::StartWasm>()
            .add_event::<runner::WasmStartedEvent>()
            .add_event::<runner::WasmErrorEvent>()
            .insert_resource(WasmGlobals {
                engine,
                linker : Arc::new(linker)
            })
            .add_systems(Update, runner::compile_wasms);
    }
}


#[derive(Resource)]
pub struct WasmGlobals {
    engine : wt::Engine,
    linker : Arc<wt::Linker<state::InstanceState>>
}
impl WasmGlobals {

    fn new_engine() -> wt::Engine {
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
        wt::Engine::new(&engine_config).unwrap()
    }

}
