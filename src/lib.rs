extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::u8;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen] implied
#[derive(Deserialize)]
pub struct ColorCounter {
    data: Vec<u8>,
    colors: HashMap<String, [u8; 3]>,
}

#[derive(Serialize)]
pub struct ColorCounterResult {
    count: HashMap<String, u32>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
pub fn calculate(data: &JsValue) -> JsValue {
    let cc: ColorCounter = ColorCounter::new(data);
    let result = cc.calculate();

    JsValue::from_serde(&result).unwrap()
}

impl ColorCounter {
    pub fn new(data: &JsValue) -> ColorCounter {
        utils::set_panic_hook();
        let cc: ColorCounter = data.into_serde().unwrap();

        return cc;
    }

    pub fn calculate(self) -> ColorCounterResult {
        let mut count: HashMap<usize, u32> = HashMap::new();
        let mut result: HashMap<String, u32> = HashMap::new();

        // utils::time("calculate");

        for chunk in self.data.chunks_exact(4) {
            let rgb_chunk = utils::rgb(chunk);

            match self
                .colors
                .iter()
                .position(|(_name, color)| (*color as [u8; 3]) == (rgb_chunk as [u8; 3]))
            {
                Some(index) => *count.entry(index).or_insert(0) += 1,
                None => *count.entry(usize::MAX).or_insert(0) += 1,
            };
        }

        for (i, (name, _rgb)) in self.colors.iter().enumerate() {
            match count.get(&i) {
                Some(value) => result.insert(name.to_string(), *value),
                None => result.insert(name.to_string(), 0),
            };
        }
        match count.get(&usize::MAX) {
            Some(value) => result.insert("unknown".to_string(), *value),
            None => result.insert("unknown".to_string(), 0),
        };

        // utils::timeEnd("calculate");

        return ColorCounterResult { count: result };
    }
}
