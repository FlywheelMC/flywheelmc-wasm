use crate::WasmGlobals;
use crate::state::InstanceState;
use flywheelmc_common::prelude::*;
use wasmtime as wt;


impl WasmGlobals {

    /// Can take WASM or WAT format.
    pub fn new_from_file<P : AsRef<Path>>(&self, fpath : P) -> StartWasm {
        Self::new_from_maybe_module(wt::Module::from_file(&self.engine, fpath))
    }

    /// Can take WASM or WAT format.
    pub fn new_from_binary(&self, binary : &[u8]) -> StartWasm {
        Self::new_from_maybe_module(wt::Module::from_binary(&self.engine, binary))
    }

    #[inline]
    pub fn new_from_module(module : wt::Module) -> StartWasm {
        Self::new_from_maybe_module(Ok(module))
    }

    #[inline(always)]
    pub fn new_from_maybe_module(module : wt::Result<wt::Module>) -> StartWasm {
        StartWasm {
            id     : StartWasmId(START_WASM_ID.fetch_add(1, AtomicOrdering::Relaxed)),
            module : SMutex::new(Some(module))
        }
    }

}


static START_WASM_ID : AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StartWasmId(pub u64);

#[derive(Event)]
pub struct StartWasm {
    id     : StartWasmId,
    module : SMutex<Option<wt::Result<wt::Module>>>
}

#[derive(Event)]
pub struct WasmErrorEvent {
    pub id  : StartWasmId,
    pub err : wt::Error
}

#[derive(Event)]
pub struct WasmStartedEvent {
    pub id     : StartWasmId,
    pub entity : Entity
}

#[derive(Component)]
pub struct WasmRunnerInstance {
    main_fn_task : Task<()>
}


pub fn compile_wasms(
    mut cmds      : Commands,
        r_globals : Res<WasmGlobals>,
    mut ew_start  : EventReader<StartWasm>
) {
    for StartWasm { id, module } in ew_start.read() {
        debug!("Instantiating WASM runner {}...", id.0);
        if let Some(module) = module.lock().unwrap().take() {
            let id     = *id;
            let engine = r_globals.engine.clone();
            let linker = Arc::clone(&r_globals.linker);
            cmds.spawn_task(async move || {
                let result = async move {

                    let     module   = module?;
                    /// TODO: Validate module imports and exports.
                    let mut store    = wt::Store::new(&engine, InstanceState {});
                    store.set_fuel(u64::MAX).unwrap();
                    let     instance = linker.instantiate_async(&mut store, &module).await?;
                    let     main_fn  = instance.get_typed_func::<(), ()>(&mut store, "flywheel_main")?;
                    Ok(AsyncWorld.spawn_task(async move {
                        let _ = main_fn.call_async(&mut store, ()).await.unwrap(); // TODO: Get rid of this unwrap.
                    }))

                }.await;
                match (result) {
                    Ok(main_fn_task) => {
                        info!("Started WASM runner {}...", id.0);
                        let runner = AsyncWorld.spawn_bundle(WasmRunnerInstance { main_fn_task });
                        let _      = AsyncWorld.send_event(WasmStartedEvent { id, entity : runner.id() });
                    }
                    Err(err) => {
                        debug!("Failed to start WASM runner {}: {err}", id.0);
                        let _ = AsyncWorld.send_event(WasmErrorEvent { id, err });
                    }
                }
                Ok(())
            });
        }
    }
}

/*async fn compile_wasms(
    engine : wt::Engine,
    linker : Arc<wt::Linker<InstanceState>>
) {
    loop {
        let event = AsyncWorld.next_event::<StartWasm>().await;
        debug!("WASM {} compile requested", event.id.0);
        let engine = engine.clone();
        let linker = Arc::clone(&linker);
        AsyncWorld.spawn_bundle(CompilingWasm { task : AsyncWorld.spawn_task(async move {
            let result : Result<wt::Instance, Arc<wt::Error>> = async move {
                let module   = event.module?;
                /// TODO: Validate module imports and exports.
                let store    = wt::Store::new(&engine, InstanceState {});
                let instance = linker.instantiate_async(store, &module).await?;
                Ok(instance)
            }.await;
        }) });
        task::yield_now().await;
    }
}*/
