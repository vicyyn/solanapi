use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_pi_account(pi_id: u64) -> JsValue {
    let pi = pisolana::Pi::pda(pi_id);
    return serde_wasm_bindgen::to_value(&(pi.0.to_string(), pi.1)).unwrap();
}

#[wasm_bindgen]
pub fn get_hex_block_account(pi_id: u64, block_id: u64) -> JsValue {
    let hex_block = pisolana::HexBlock::pda(pi_id, block_id);
    return serde_wasm_bindgen::to_value(&(hex_block.0.to_string(), hex_block.1)).unwrap();
}
