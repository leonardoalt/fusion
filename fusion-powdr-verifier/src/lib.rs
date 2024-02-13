#![no_std]

use powdr_riscv_runtime::{coprocessors::get_data_serde, print};

use ruint::aliases::U256;

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use fusion_api::Tx;
use fusion_state::{apply_tx, state};

#[no_mangle]
fn main() {
    let serialized_state: Vec<u8> = get_data_serde(42);
    let state: State = serde_cbor::from_slice(&serialized_state).unwrap();

    let serialized_tx: Vec<u8> = get_data_serde(43);
    let tx: Tx = serde_cbor::from_slice(&serialized_tx).unwrap();

    let serialized_post_root: Vec<u8> = get_data_serde(44);
    let post_root: U256 = serde_cbor::from_slice(&serialized_post_root).unwrap();

    let state = apply_tx(state, &tx);
    assert_eq!(post_root, state.root());
}
