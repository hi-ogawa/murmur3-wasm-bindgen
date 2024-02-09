#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use murmur3;
use std::io::Cursor;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
#[allow(dead_code)]
pub fn murmur3_32(source: &[u8], seed: u32) -> Result<u32, JsError> {
    let mut cursor = Cursor::new(source);
    Ok(murmur3::murmur3_32(&mut cursor, seed)?)
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_murmur3_32(
        arg0: <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
        arg1: <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi,
    ) -> <Result<u32, JsError> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0)
            };
            let arg0 = &*arg0;
            let arg1 = unsafe {
                <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
            };
            let _ret = murmur3_32(arg0, arg1);
            _ret
        };
        <Result<u32, JsError> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[allow(dead_code)]
pub fn murmur3_x64_128(
    source: &[u8],
    seed: u32,
    dest: &mut [u32],
) -> Result<(), JsError> {
    let mut cursor = Cursor::new(source);
    let h = murmur3::murmur3_x64_128(&mut cursor, seed)?;
    dest[0] = ((h >> 32 * 0) & 0xffffffff) as u32;
    dest[1] = ((h >> 32 * 1) & 0xffffffff) as u32;
    dest[2] = ((h >> 32 * 2) & 0xffffffff) as u32;
    dest[3] = ((h >> 32 * 3) & 0xffffffff) as u32;
    Ok(())
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_murmur3_x64_128(
        arg0: <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
        arg1: <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi,
        arg2: <[u32] as wasm_bindgen::convert::RefMutFromWasmAbi>::Abi,
    ) -> <Result<(), JsError> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0)
            };
            let arg0 = &*arg0;
            let arg1 = unsafe {
                <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
            };
            let mut arg2 = unsafe {
                <[u32] as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                    arg2,
                )
            };
            let arg2 = &mut *arg2;
            let _ret = murmur3_x64_128(arg0, arg1, arg2);
            _ret
        };
        <Result<(), JsError> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[allow(dead_code)]
pub fn murmur3_x86_128(
    source: &[u8],
    seed: u32,
    dest: &mut [u32],
) -> Result<(), JsError> {
    let mut cursor = Cursor::new(source);
    let h = murmur3::murmur3_x86_128(&mut cursor, seed)?;
    dest[0] = ((h >> 32 * 0) & 0xffffffff) as u32;
    dest[1] = ((h >> 32 * 1) & 0xffffffff) as u32;
    dest[2] = ((h >> 32 * 2) & 0xffffffff) as u32;
    dest[3] = ((h >> 32 * 3) & 0xffffffff) as u32;
    Ok(())
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_murmur3_x86_128(
        arg0: <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
        arg1: <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi,
        arg2: <[u32] as wasm_bindgen::convert::RefMutFromWasmAbi>::Abi,
    ) -> <Result<(), JsError> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <[u8] as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0)
            };
            let arg0 = &*arg0;
            let arg1 = unsafe {
                <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
            };
            let mut arg2 = unsafe {
                <[u32] as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                    arg2,
                )
            };
            let arg2 = &mut *arg2;
            let _ret = murmur3_x86_128(arg0, arg1, arg2);
            _ret
        };
        <Result<(), JsError> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
