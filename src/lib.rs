/**
 * Ledger-rs-lib Wasm interfaces
 * For use in JavaScript
 */

use wasm_bindgen::{prelude::*, convert::IntoWasmAbi};

use ledger_rs_lib::journal::Journal;

/// Parse given text in Ledger journal format.
/// 
#[wasm_bindgen]
pub fn parse(text: &str) -> Journal {
    ledger_rs_lib::parse(text)
}

// impl IntoWasmAbi for Journal {
//     type Abi;

//     fn into_abi(self) -> Self::Abi {
//         todo!()
//     }
// }