use crate::state::InstanceState;
use crate::types::{ WasmParamTyList, WasmParamTy, WasmResult, WasmReturnTy };
use crate::types::{ WasmPtr, WasmAnyPtr, WasmPtrable };
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
            panic!("Importable function {:?} is already defined.", name);
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


pub struct WasmCallCtx<'l> {
    caller : wt::Caller<'l, InstanceState>
}

impl<'l> WasmCallCtx<'l> {

    pub fn store(&self) -> impl wt::AsContext {
        &self.caller
    }

    pub fn store_mut(&mut self) -> impl wt::AsContextMut {
        &mut self.caller
    }

    pub fn mem_read<T : WasmPtrable>(&self, ptr : WasmPtr<T>) -> Result<T, MemoryDecodeError> {
        self.caller.data().memory.ok_or(MemoryDecodeError::OutOfBounds).and_then(|memory| T::mem_read(self, &memory, ptr))
    }
    pub fn mem_read_any(&self, ptr : WasmAnyPtr, len : u32,) -> Result<&[u8], MemoryOutOfBounds> {
        let a = ptr.ptr as usize;
        let b = a + (len as usize);
        self.caller.data().memory.and_then(|memory| memory.data(&self.caller).get(a..b)).ok_or(MemoryOutOfBounds)
    }
    pub fn mem_read_str(&self, ptr : WasmAnyPtr, len : u32,) -> Result<&str, MemoryDecodeError> {
        let mem = self.mem_read_any(ptr, len)?;
        Ok(str::from_utf8(mem)?)
    }

    pub fn mem_write<T : WasmPtrable>(&mut self, ptr : WasmPtr<T>, value : T) -> Result<(), MemoryOutOfBounds> {
        self.caller.data().memory.ok_or(MemoryOutOfBounds).and_then(|memory| value.mem_write(self, &memory, ptr))
    }

}

pub struct MemoryOutOfBounds;
impl From<wt::MemoryAccessError> for MemoryOutOfBounds {
    fn from(_ : wt::MemoryAccessError) -> Self { Self }
}
impl From<MemoryOutOfBounds> for Cow<'_, str> {
    fn from(_ : MemoryOutOfBounds) -> Self {
        Self::Borrowed("memory index out of bounds")
    }
}

pub enum MemoryDecodeError {
    OutOfBounds,
    InvalidData
}
impl From<MemoryOutOfBounds> for MemoryDecodeError {
    fn from(_ : MemoryOutOfBounds) -> Self {
        Self::OutOfBounds
    }
}
impl From<wt::MemoryAccessError> for MemoryDecodeError {
    fn from(_ : wt::MemoryAccessError) -> Self { Self::OutOfBounds }
}
impl From<str::Utf8Error> for MemoryDecodeError {
    fn from(_ : str::Utf8Error) -> Self {
        Self::InvalidData
    }
}
impl From<MemoryDecodeError> for Cow<'_, str> {
    fn from(value : MemoryDecodeError) -> Self {
        match (value) {
            MemoryDecodeError::OutOfBounds => MemoryOutOfBounds.into(),
            MemoryDecodeError::InvalidData => Cow::Borrowed("invalid utf8")
        }
    }
}
