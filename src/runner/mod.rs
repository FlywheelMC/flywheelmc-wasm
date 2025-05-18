use crate::WasmGlobals;
use flywheelmc_common::prelude::*;


pub mod player;

pub mod event;


pub(crate) struct InstanceState {
    pub(crate) runner         : Entity,
    pub(crate) memory         : Option<wt::Memory>,
    pub(crate) fn_alloc       : Option<wt::TypedFunc<(u32, u32,), u32>>,
    pub(crate) event_receiver : mpsc::UnboundedReceiver<(&'static str, Vec<u8>,)>
}


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
    pub err : WasmError
}

#[derive(Debug)]
pub enum WasmError {
    Terminated,
    Wasmtime(wt::Error)
}
impl From<wt::Error> for WasmError {
    fn from(value : wt::Error) -> Self {
        Self::Wasmtime(value)
    }
}

#[derive(Event)]
pub struct WasmStartedEvent {
    pub id     : StartWasmId,
    pub entity : Entity
}

#[derive(Component)]
pub struct WasmRunnerInstance {
                id           : StartWasmId,
                #[allow(dead_code)]
                main_fn_task : Task<()>,
    pub(crate)  players      : BiBTreeMap<u64, Entity>,
                event_sender : mpsc::UnboundedSender<(&'static str, Vec<u8>,)>
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
                    let module = module?;
                    let (event_sender, event_receiver,) = mpsc::unbounded_channel();
                    // TODO: Validate module imports and exports.
                    let mut entity   = AsyncWorld.spawn_bundle(());
                    let mut store    = wt::Store::new(&engine, InstanceState {
                        runner         : entity.id(),
                        memory         : None,
                        fn_alloc       : None,
                        event_receiver
                    });
                    store.set_fuel(u64::MAX).unwrap();
                    store.fuel_async_yield_interval(Some(1024)).unwrap();
                    let     instance = linker.instantiate_async(&mut store, &module).await?;
                    store.data_mut().memory   = Some(instance.get_memory(&mut store, "memory").unwrap()); // TODO: Get rid of this unwrap.
                    store.data_mut().fn_alloc = Some(instance.get_typed_func(&mut store, "flywheel_alloc").unwrap()); // TODO: Get rid of this unwrap.
                    let     main_fn  = instance.get_typed_func::<(), ()>(&mut store, "flywheel_main")?;
                    Ok((entity, event_sender, AsyncWorld.spawn_task(async move {
                        let _ = task::poll_and_yield(main_fn.call_async(&mut store, ())).await;
                    })))

                }.await;
                match (result) {
                    Ok((entity, event_sender, main_fn_task,)) => {
                        info!("Started WASM runner {}...", id.0);
                        let _ = entity.insert(WasmRunnerInstance {
                            id,
                            main_fn_task,
                            players      : BiBTreeMap::new(),
                            event_sender
                        });
                        let _ = AsyncWorld.send_event(WasmStartedEvent { id, entity : entity.id() });
                    }
                    Err(err) => {
                        debug!("Failed to start WASM runner {}: {err:?}", id.0);
                        let _ = AsyncWorld.send_event(WasmErrorEvent { id, err });
                    }
                }
                Ok(())
            });
        }
    }
}
