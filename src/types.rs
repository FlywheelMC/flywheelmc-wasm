use flywheelmc_common::prelude::*;
use core::marker::Tuple;
use wasmtime as wt;


pub trait WasmParamTyList : Tuple + Send + 'static {
    type Wasm : wt::WasmTyList + wt::WasmRet;
    fn from_wasm(wasm : Self::Wasm) -> Self;
}
variadic!{ impl_wasmtyconv_for_tuple }
macro impl_wasmtyconv_for_tuple( $( $generic:ident ),* $(,)? ) {
    impl< $( $generic : WasmParamTy , )* > WasmParamTyList for ( $( $generic , )* ) {
        type Wasm = ( $( <$generic as WasmParamTy>::Wasm , )* );
        #[allow(unused_variables)]
        fn from_wasm(wasm : Self::Wasm) -> Self {
            ( $( <$generic as WasmParamTy>::from_wasm(wasm.${index()}) , )* )
        }
    }
}


pub trait WasmParamTy : Send + 'static {
    type Wasm : wt::WasmTy;
    fn from_wasm(wasm : Self::Wasm) -> Self;
}
impl WasmParamTy for u32 {
    type Wasm = u32;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl WasmParamTy for u64 {
    type Wasm = u64;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl<T : WasmPtrable> WasmParamTy for WasmPtr<T> {
    type Wasm = u32;
    fn from_wasm(ptr : Self::Wasm) -> Self { Self { ptr, marker : PhantomData } }
}
impl WasmParamTy for WasmAnyPtr {
    type Wasm = u32;
    fn from_wasm(ptr : Self::Wasm) -> Self { Self { ptr } }
}


pub type WasmResult<T> = Result<T, Cow<'static, str>>;

pub trait WasmReturnTy {
    type Wasm : wt::WasmRet + 'static;
    fn to_wasm(self) -> Self::Wasm;
}
impl WasmReturnTy for () {
    type Wasm = ();
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for u32 {
    type Wasm = u32;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for u64 {
    type Wasm = u64;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl<T : WasmPtrable> WasmReturnTy for WasmPtr<T> {
    type Wasm = u32;
    fn to_wasm(self) -> Self::Wasm { self.ptr }
}
impl WasmReturnTy for WasmAnyPtr {
    type Wasm = u32;
    fn to_wasm(self) -> Self::Wasm { self.ptr }
}


pub unsafe trait WasmPtrable : 'static { }
unsafe impl WasmPtrable for u8 { }
unsafe impl WasmPtrable for i8 { }
unsafe impl WasmPtrable for u16 { }
unsafe impl WasmPtrable for i16 { }
unsafe impl WasmPtrable for u32 { }
unsafe impl WasmPtrable for i32 { }
unsafe impl WasmPtrable for u64 { }
unsafe impl WasmPtrable for i64 { }
unsafe impl WasmPtrable for u128 { }
unsafe impl WasmPtrable for i128 { }
unsafe impl<T : WasmPtrable> WasmPtrable for WasmPtr<T> { }
unsafe impl WasmPtrable for WasmAnyPtr { }


pub struct WasmPtr<T : WasmPtrable> {
    ptr    : u32,
    marker : PhantomData<*mut T>
}

unsafe impl<T : WasmPtrable> Send for WasmPtr<T> { }

impl <T : WasmPtrable> WasmPtr<T> {

    pub fn write(&self, value : &T) -> Result<(), OutOfBounds> {
        todo!();
    }

}


pub struct WasmAnyPtr {
    ptr : u32
}

impl WasmAnyPtr {

    pub unsafe fn write(&self, value : &[u8]) -> Result<(), OutOfBounds> {
        todo!();
    }

}


#[derive(Clone, Copy, Debug)]
pub struct OutOfBounds;
impl<'l> From<OutOfBounds> for Cow<'l, str> {
    fn from(_ : OutOfBounds) -> Self {
        Self::Borrowed("memory range out of bounds")
    }
}
