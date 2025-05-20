use crate::types::{ WasmPtr, WasmAnyPtr, WasmPtrable };
use crate::runner::{ WasmRunnerInstance, InstanceState };
use flywheelmc_common::prelude::*;


pub struct WasmCallCtx<'l> {
    pub(crate) caller : wt::Caller<'l, InstanceState>
}

impl<'l> WasmCallCtx<'l> {

    pub fn runner(&self) -> Entity {
        self.caller.data().runner
    }

    pub async fn player_session_to_entity(&self, session_id : u64) -> Option<Entity> {
        AsyncWorld.query::<(&WasmRunnerInstance,)>().entity(self.runner()).get(|(runner,)| runner.players.get_by_left(&session_id).cloned()).ok().flatten()
    }
    pub async fn player_entity_to_session(&self, entity : Entity) -> Option<u64> {
        AsyncWorld.query::<(&WasmRunnerInstance,)>().entity(self.runner()).get(|(runner,)| runner.players.get_by_right(&entity).cloned()).ok().flatten()
    }

    pub fn store(&self) -> impl wt::AsContext {
        &self.caller
    }
    pub fn store_mut(&mut self) -> impl wt::AsContextMut {
        &mut self.caller
    }

    pub async fn next_event(&mut self) -> Option<(&'static str, Vec<u8>,)> {
        self.caller.data_mut().event_receiver.try_recv().ok()
    }
    pub fn refuel(&mut self) {
        let _ = self.caller.set_fuel(u64::MAX);
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
    pub fn mem_write_any(&mut self, ptr : WasmAnyPtr, value : &[u8]) -> Result<(), MemoryOutOfBounds> {
        self.caller.data().memory.ok_or(MemoryOutOfBounds)?.write(&mut self.caller, ptr.offset(), value)?;
        Ok(())
    }
    pub fn mem_write_str(&mut self, ptr : WasmAnyPtr, value : &str) -> Result<(), MemoryOutOfBounds> {
        self.mem_write_any(ptr, value.as_bytes())
    }

    pub async fn mem_alloc<T : WasmPtrable>(&mut self) -> Result<WasmPtr<T>, MemoryAllocError> {
        Ok(self.mem_alloc_any(T::LEN, T::ALIGN).await?.assume_type())
    }
    pub async fn mem_alloc_any(&mut self, len : usize, align : usize) -> Result<WasmAnyPtr, MemoryAllocError> {
        let ptr = self.caller.data().fn_alloc.clone().ok_or(MemoryAllocError::NoAllocFn)?.call_async(&mut self.caller, (len as u32, align as u32,)).await?;
        Ok(WasmAnyPtr::from_ptr(ptr))
    }

    pub async fn mem_alloc_write<T : WasmPtrable>(&mut self, value : T) -> Result<WasmPtr<T>, MemoryAllocError> {
        let ptr = self.mem_alloc::<T>().await?;
        self.mem_write(ptr, value)?;
        Ok(ptr)
    }
    pub async fn mem_alloc_write_any(&mut self, value : &[u8]) -> Result<WasmAnyPtr, MemoryAllocError> {
        let ptr = self.mem_alloc_any(value.len(), 1).await?;
        self.mem_write_any(ptr, value)?;
        Ok(ptr)
    }
    pub async fn mem_alloc_write_str(&mut self, value : &str) -> Result<WasmAnyPtr, MemoryAllocError> {
        self.mem_alloc_write_any(value.as_bytes()).await
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
            MemoryDecodeError::InvalidData => Self::Borrowed("invalid utf8")
        }
    }
}

pub enum MemoryAllocError {
    NoAllocFn,
    OutOfBounds,
    Wasmtime(wt::Error)
}
impl From<MemoryOutOfBounds> for MemoryAllocError {
    fn from(_ : MemoryOutOfBounds) -> Self {
        Self::OutOfBounds
    }
}
impl From<MemoryAllocError> for Cow<'_, str> {
    fn from(value : MemoryAllocError) -> Self {
        match (value) {
            MemoryAllocError::NoAllocFn     => Self::Borrowed("no alloc function"),
            MemoryAllocError::OutOfBounds   => Self::Borrowed("memory index out of bounds"),
            MemoryAllocError::Wasmtime(err) => Self::Owned(err.to_string())
        }
    }
}
impl From<wt::Error> for MemoryAllocError {
    fn from(value : wt::Error) -> Self {
        Self::Wasmtime(value)
    }
}
