use crate::runner::{ InstanceState, WasmCallCtx };
use crate::types::{ WasmParamTyList, WasmParamTy, WasmResult, WasmReturnTy };
use flywheelmc_common::prelude::*;


pub const MODULE : &str = "env";


type ImportFuncRegister = Box<dyn (Fn(&mut wt::Linker<InstanceState>, &'static str) -> wt::Result<()>) + Send + Sync>;

pub struct ImportFuncs {
    funcs : HashMap<&'static str, ImportFuncRegister>
}

impl ImportFuncs {

    pub(crate) fn new() -> Self { Self {
        funcs : HashMap::new()
    } }

    pub fn define<F, Params, Returns>(&mut self, name : &'static str, f : F) -> &mut Self
    where
        F       : for<'l> ImportFunc<'l, Params, Returns> + Clone,
        Params  : WasmParamTyList,
        Returns : WasmReturnTy
    {
        if (self.funcs.try_insert(name, Box::new(move |linker, name| {
            let f = f.clone();
            linker.func_wrap_async(MODULE, name, move |caller, params| {
                f.clone().call(WasmCallCtx { caller }, params)
            }).map(|_| ())
        })).is_err()) {
            panic!("Importable function {name:?} is already defined.");
        }
        self
    }

    pub(crate) fn register(&self, linker : &mut wt::Linker<InstanceState>) -> wt::Result<()> {
        for (name, register,) in &self.funcs {
            register(linker, name)?;
        }
        Ok(())
    }

}

pub trait ImportFunc<'l, Params, Returns>
where
    Self    : Send + Sync + 'static,
    Params  : WasmParamTyList,
    Returns : WasmReturnTy
{
    fn call(self, ctx : WasmCallCtx<'l>, params : Params::Wasm) -> Box<dyn Future<Output = Returns::Wasm> + Send + 'l>;
}
variadic!{ impl_import_func_for_fns }
macro impl_import_func_for_fns( $( $generic:ident ),* $(,)? ) {
    impl<'l,
        $( $generic : WasmParamTy , )*
        Returns : WasmReturnTy,
        Fut     : Future<Output = WasmResult<Returns>> + Send + 'l ,
        Func    : (Fn( WasmCallCtx<'l> , $( $generic , )* ) -> Fut) + Send + Sync + 'static ,
    > ImportFunc<'l,
        ( $( $generic , )* ),
        Returns
    > for Func
    {
        fn call(self,
            ctx : WasmCallCtx<'l>,
            #[allow(unused_variables)]
            params : ( $( $generic::Wasm , )* )
        ) -> Box<dyn Future<Output = Returns::Wasm> + Send + 'l> {
            Box::new(async move {
                let result = self( ctx, $( $generic::from_wasm(params.${index()}) , )* ).await.unwrap(); // TODO: Get rid of this unwrap.
                Returns::to_wasm(result)
            })
        }
    }
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
        // TODO: Define export func
        self
    }

}

pub struct ExportFunc {}
