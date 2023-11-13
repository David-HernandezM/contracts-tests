#![no_std]

#[cfg(not(feature = "binary-vendor"))]
mod contract;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

//target/wasm32-unknown-unknown/release/program_state.meta.wasm