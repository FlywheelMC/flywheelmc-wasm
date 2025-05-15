use crate::WasmGlobals;
use crate::state::InstanceState;
use flywheelmc_common::prelude::*;
use wasmtime as wt;


impl WasmGlobals {

    /// Can take WASM or WAT format.
    pub async fn compile_file<P : AsRef<Path>>(&self, fpath : P) -> wt::Result<()> {
        let module       = wt::Module::from_file(&self.engine, fpath)?;
        // TODO: Validate module imports and exports.
        let mut store    = wt::Store::new(&self.engine, InstanceState {

        });
        let     instance = self.linker.instantiate_async(&mut store, &module).await?;
        todo!("Success!");
    }

}
