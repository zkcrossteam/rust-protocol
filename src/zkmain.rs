use zkwasm_rust_sdk::jubjub::BabyJubjubPoint;
use zkwasm_rust_sdk::jubjub::JubjubSignature;
use zkwasm_rust_sdk::merkle::Merkle;
use zkwasm_rust_sdk::wasm_dbg;
use zkwasm_rust_sdk::wasm_output;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::poseidon::PoseidonHasher;
use primitive_types::U256;


use crate::DepositInfo;
use sha2::{Sha256, Digest};

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn zkmain() -> i64 {
    let nounce = unsafe {
        wasm_input(0)
    };
    let account =
        unsafe {
            [
                wasm_input(0), //1
                wasm_input(0), //2
                wasm_input(0), //3
                wasm_input(0), //4
            ]
        };

    let amount =
        unsafe {
            [
                wasm_input(0), //1
                wasm_input(0), //2
                wasm_input(0), //3
                wasm_input(0), //4
            ]
        };

    let receiver =
        unsafe {
            [
                wasm_input(0), //1
                wasm_input(0), //2
                wasm_input(0), //3
                wasm_input(0), //4
            ]
        };


    let mut merkle = Merkle::load(
        unsafe {
            [
                // old root
                wasm_input(1),
                wasm_input(1),
                wasm_input(1),
                wasm_input(1),
            ]
        }
    );

    let mut hasher = PoseidonHasher::new();
    for d in account {
        hasher.update(d);
    }
    let sender = hasher.finalize();

    let mut sub_merkle_root = [0u64; 4];

    merkle.get(sender[0], &mut sub_merkle_root, false);

    let mut sub_merkle = Merkle::load(sub_merkle_root.clone());

    for d in receiver {
        hasher.update(d);
    }
    let receiver = hasher.finalize();

    let mut current_amount = [0u64; 4];
    sub_merkle.get(receiver[0], &mut current_amount, true);

    current_amount = (U256(amount) + U256(current_amount)).0;

    sub_merkle.set(receiver[0], &current_amount, true);

    merkle.set(sender[0], &sub_merkle.root, false);

    let tx_data = DepositInfo::new(nounce, 0, 0, amount, account);

    let mut hasher = Sha256::new();
    for data in tx_data.to_bytes() {
        hasher.update(data.to_le_bytes());
    }
    let result = hasher.finalize();
    let sha_u64:[u64;4] = unsafe { std::mem::transmute(result) };
    unsafe {
        wasm_dbg(sha_u64[0]);
        wasm_dbg(sha_u64[1]);
        wasm_dbg(sha_u64[2]);
        wasm_dbg(sha_u64[3]);
    };

    unsafe {
        // new_root
        wasm_output(merkle.root[0]);
        wasm_output(merkle.root[1]);
        wasm_output(merkle.root[2]);
        wasm_output(merkle.root[3]);
        // sha
        wasm_output(sha_u64[0]);
        wasm_output(sha_u64[1]);
        wasm_output(sha_u64[2]);
        wasm_output(sha_u64[3]);
    };

    //super::dbg!("abc{}\n", a);
    0
}
