use crate::state;
use crate::types::{ WasmParamTyList, WasmResult, WasmReturnTy };
use flywheelmc_common::prelude::*;
use wasmtime as wt;


pub const MODULE : &str = "env";


pub struct ImportFuncs {
    funcs : HashMap<&'static str, ImportFunc>
}

impl ImportFuncs {

    pub(crate) fn new() -> Self { Self {
        funcs : HashMap::new()
    } }

    pub fn define<F, Fut, Params, Returns>(&mut self, name : &'static str, f : F) -> &mut Self
    where
        F       : (Fn<Params, Output = Fut>) + Clone + Send + Sync + 'static,
        Fut     : Future<Output = WasmResult<Returns>> + Send + 'static,
        Params  : WasmParamTyList,
        Returns : WasmReturnTy
    {
        if (self.funcs.try_insert(name, ImportFunc {
            register : Box::new(move |linker, name| {
                let f = f.clone();
                linker.func_wrap_async(MODULE, name, move |_, params : Params::Wasm| { // TODO: Caller will be needed
                    let f      = f.clone();
                    let params = Params::from_wasm(params);
                    Box::new(async move {
                        let result = f.call(params).await.unwrap(); // TODO: Kill plot on err
                        Returns::to_wasm(result)
                    })
                }).map(|_| ())
            })
        }).is_err()) {
            todo!() // TODO: Panic because name already taken
        }
        self
    }

    pub(crate) fn register(&self, linker : &mut wt::Linker<state::InstanceState>) -> wt::Result<()> {
        for (name, func,) in &self.funcs {
            (func.register)(linker, name)?;
        }
        Ok(())
    }

}

type ImportFuncRegister = Box<dyn (Fn(&mut wt::Linker<state::InstanceState>, &'static str) -> wt::Result<()>) + Send + Sync>;
struct ImportFunc {
    register : ImportFuncRegister
}



pub struct ExportFuncs {
    funcs : HashMap<&'static str, ExportFunc>
}

impl ExportFuncs {

    pub(crate) fn new() -> Self { Self {
        funcs : HashMap::new()
    } }

    pub fn define<Params, Returns>(&mut self, _name : &'static str, _required : bool) -> &mut Self
    where
        Params  : WasmParamTyList,
        Returns : WasmReturnTy
    {
        todo!(); // TODO: Define export func
    }

}

pub struct ExportFunc {}
