use crate::runner::{ MemoryOutOfBounds, MemoryDecodeError, WasmCallCtx };
use flywheelmc_common::prelude::*;
use core::marker::Tuple;


mod transaction;
pub use transaction::*;


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
            #[allow(clippy::unused_unit)]
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
impl WasmParamTy for i32 {
    type Wasm = i32;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl WasmParamTy for u64 {
    type Wasm = u64;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl WasmParamTy for i64 {
    type Wasm = i64;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl WasmParamTy for f32 {
    type Wasm = f32;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl WasmParamTy for f64 {
    type Wasm = f64;
    fn from_wasm(wasm : Self::Wasm) -> Self { wasm }
}
impl<T : WasmPtrable> WasmParamTy for WasmPtr<T> {
    type Wasm = u32;
    fn from_wasm(ptr : Self::Wasm) -> Self { Self::from_ptr(ptr) }
}
impl WasmParamTy for WasmAnyPtr {
    type Wasm = u32;
    fn from_wasm(ptr : Self::Wasm) -> Self { Self { ptr } }
}
impl WasmParamTy for TransactionId {
    type Wasm = u64;
    fn from_wasm(id : Self::Wasm) -> Self { Self { id } }
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
impl WasmReturnTy for i32 {
    type Wasm = i32;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for u64 {
    type Wasm = u64;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for i64 {
    type Wasm = i64;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for f32 {
    type Wasm = f32;
    fn to_wasm(self) -> Self::Wasm { self }
}
impl WasmReturnTy for f64 {
    type Wasm = f64;
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
impl WasmReturnTy for TransactionId {
    type Wasm = u64;
    fn to_wasm(self) -> Self::Wasm { self.id }
}


/// ### Safety
/// If implemented incorrectly, this could go horribly wrong.
pub unsafe trait WasmPtrable : Sized + 'static {
    const LEN   : usize;
    const ALIGN : usize;
    fn mem_read(ctx : &WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<Self, MemoryDecodeError>;
    fn mem_write(&self, ctx : &mut WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<(), MemoryOutOfBounds>;
}
macro impl_wasm_ptrable_for_num( $ty:ty ) {
    unsafe impl WasmPtrable for $ty {
        const LEN   : usize = mem::size_of::<$ty>();
        const ALIGN : usize = mem::align_of::<$ty>();
        fn mem_read(ctx : &WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<Self, MemoryDecodeError> {
            let mut buf = [0u8; mem::size_of::<Self>()];
            memory.read(ctx.store(), ptr.offset(), &mut buf)?;
            Ok(Self::from_le_bytes(buf))
        }
        fn mem_write(&self, ctx : &mut WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<(), MemoryOutOfBounds> {
            memory.write(ctx.store_mut(), ptr.offset() as usize, &self.to_le_bytes())?;
            Ok(())
        }
    }
}
impl_wasm_ptrable_for_num!(u8);
impl_wasm_ptrable_for_num!(i8);
impl_wasm_ptrable_for_num!(u16);
impl_wasm_ptrable_for_num!(i16);
impl_wasm_ptrable_for_num!(u32);
impl_wasm_ptrable_for_num!(i32);
impl_wasm_ptrable_for_num!(u64);
impl_wasm_ptrable_for_num!(i64);
impl_wasm_ptrable_for_num!(u128);
impl_wasm_ptrable_for_num!(i128);
impl_wasm_ptrable_for_num!(f32);
impl_wasm_ptrable_for_num!(f64);
unsafe impl<T : WasmPtrable> WasmPtrable for WasmPtr<T> {
    const LEN   : usize = mem::size_of::<u32>();
    const ALIGN : usize = mem::align_of::<u32>();
    fn mem_read(ctx : &WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<Self, MemoryDecodeError> {
        <u32 as WasmPtrable>::mem_read(ctx, memory, ptr.cast()).map(Self::from_ptr)
    }
    fn mem_write(&self, ctx : &mut WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<(), MemoryOutOfBounds> {
        <u32 as WasmPtrable>::mem_write(&self.ptr(), ctx, memory, ptr.cast())
    }
}
unsafe impl WasmPtrable for WasmAnyPtr {
    const LEN   : usize = mem::size_of::<u32>();
    const ALIGN : usize = mem::align_of::<u32>();
    fn mem_read(ctx : &WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<Self, MemoryDecodeError> {
        <u32 as WasmPtrable>::mem_read(ctx, memory, ptr.cast()).map(Self::from_ptr)
    }
    fn mem_write(&self, ctx : &mut WasmCallCtx<'_>, memory : &wt::Memory, ptr : WasmPtr<Self>) -> Result<(), MemoryOutOfBounds> {
        <u32 as WasmPtrable>::mem_write(&self.ptr(), ctx, memory, ptr.cast())
    }
}


pub struct WasmPtr<T : WasmPtrable> {
    pub(crate)  ptr     : u32,
                _marker : PhantomData<*mut T>
}
impl<T : WasmPtrable> Clone for WasmPtr<T> {
    fn clone(&self) -> Self { *self }
}
impl<T : WasmPtrable> Copy for WasmPtr<T> { }
impl<T : WasmPtrable> WasmPtr<T> {

    #[inline]
    pub fn from_ptr(ptr : u32) -> Self { Self { ptr, _marker : PhantomData } }

    #[inline]
    pub fn ptr(&self) -> u32 { self.ptr }

    #[inline]
    pub fn offset(&self) -> usize { self.ptr as usize }

    #[inline(always)]
    pub fn cast<U : WasmPtrable>(self) -> WasmPtr<U> {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    pub fn type_erase(self) -> WasmAnyPtr { WasmAnyPtr::from_ptr(self.ptr) }

}
unsafe impl<T : WasmPtrable> Send for WasmPtr<T> { }
unsafe impl<T : WasmPtrable> Sync for WasmPtr<T> { }


#[derive(Clone, Copy)]
pub struct WasmAnyPtr {
    pub(crate) ptr : u32
}

impl WasmAnyPtr {

    #[inline]
    pub fn from_ptr(ptr : u32) -> Self { Self { ptr } }

    #[inline]
    pub fn ptr(&self) -> u32 { self.ptr }

    #[inline]
    pub fn offset(&self) -> usize { self.ptr as usize }

    #[inline]
    pub fn assume_type<U : WasmPtrable>(self) -> WasmPtr<U> {
        WasmPtr::<U>::from_ptr(self.ptr)
    }

    #[inline]
    pub fn shift(self, offset : u32) -> Self {
        Self::from_ptr(self.ptr.checked_add(offset).unwrap())
    }
    #[inline]
    pub fn shift_mut(&mut self, offset : u32) -> &mut Self {
        *self = self.shift(offset);
        self
    }

    #[inline]
    pub fn shift_signed(self, offset : i32) -> Self {
        Self::from_ptr(self.ptr.checked_add_signed(offset).unwrap())
    }
    #[inline]
    pub fn shift_signed_mut(&mut self, offset : i32) -> &mut Self {
        *self = self.shift_signed(offset);
        self
    }

}


#[derive(Clone, Copy, Debug)]
pub struct OutOfBounds;
impl<'l> From<OutOfBounds> for Cow<'l, str> {
    fn from(_ : OutOfBounds) -> Self {
        Self::Borrowed("memory range out of bounds")
    }
}
