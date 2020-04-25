mod utils;

use wasm_bindgen::prelude::*;
use bip39::{Mnemonic, MnemonicType, Language};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let message = format!("Generated phrase: {}", phrase());
    alert(message.as_str());
}

#[wasm_bindgen]
pub fn phrase() -> String {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase: &str = mnemonic.phrase();
    return phrase.to_string();
}


