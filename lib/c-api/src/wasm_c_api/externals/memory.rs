use super::super::store::wasm_store_t;
use super::super::types::wasm_memorytype_t;
use std::mem;
use wasmer::{Memory, Pages};

#[allow(non_camel_case_types)]
pub struct wasm_memory_t {
    // maybe needs to hold onto instance
    pub(crate) inner: Memory,
}

#[no_mangle]
pub unsafe extern "C" fn wasm_memory_new(
    store: &wasm_store_t,
    mt: &wasm_memorytype_t,
) -> Option<Box<wasm_memory_t>> {
    let md = mt.as_memorytype().clone();
    let memory = c_try!(Memory::new(&store.inner, md));

    Some(Box::new(wasm_memory_t { inner: memory }))
}

#[no_mangle]
pub unsafe extern "C" fn wasm_memory_delete(_memory: Option<Box<wasm_memory_t>>) {}

// TODO: figure out if these should be deep or shallow copies
#[no_mangle]
pub unsafe extern "C" fn wasm_memory_copy(wasm_memory: &wasm_memory_t) -> Box<wasm_memory_t> {
    // do shallow copy
    Box::new(wasm_memory_t {
        inner: wasm_memory.inner.clone(),
    })
}

#[no_mangle]
pub unsafe extern "C" fn wasm_memory_type(_memory_ptr: &wasm_memory_t) -> *mut wasm_memorytype_t {
    todo!("wasm_memory_type")
}

// get a raw pointer into bytes
#[no_mangle]
pub unsafe extern "C" fn wasm_memory_data(memory: &mut wasm_memory_t) -> *mut u8 {
    mem::transmute::<&[std::cell::Cell<u8>], &[u8]>(&memory.inner.view()[..]) as *const [u8]
        as *const u8 as *mut u8
}

// size in bytes
#[no_mangle]
pub unsafe extern "C" fn wasm_memory_data_size(memory: &wasm_memory_t) -> usize {
    memory.inner.size().bytes().0
}

// size in pages
#[no_mangle]
pub unsafe extern "C" fn wasm_memory_size(memory: &wasm_memory_t) -> u32 {
    memory.inner.size().0 as _
}

// delta is in pages
#[no_mangle]
pub unsafe extern "C" fn wasm_memory_grow(memory: &mut wasm_memory_t, delta: u32) -> bool {
    memory.inner.grow(Pages(delta)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn wasm_memory_same(
    wasm_memory1: &wasm_memory_t,
    wasm_memory2: &wasm_memory_t,
) -> bool {
    wasm_memory1.inner.same(&wasm_memory2.inner)
}
