// extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::u8;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn substring(string: String, start: usize, end: usize) -> String {
    return string.chars().skip(start).take(start - end).collect();
}

// #[wasm_bindgen] implied
#[derive(Deserialize)]
pub struct ColorCounter {
    image_data: Vec<u8>,
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

fn hexToRgb(hex: &str) -> (u8, u8, u8) {
    assert_eq!(hex.len(), 7);

    log!("here444");
    log!("{}", hex);

    let rs = substring(hex.to_string(), 1, 3);
    let gs = substring(hex.to_string(), 3, 5);
    let bs = substring(hex.to_string(), 5, 7);

    let r = u8::from_str_radix(&rs, 10);
    let g = u8::from_str_radix(&gs, 10);
    let b = u8::from_str_radix(&bs, 10);

    return (r.unwrap(), g.unwrap(), b.unwrap());
}

fn colorsAsRGB(colors: HashMap<String, String>) -> Vec<(String, (u8, u8, u8))> {
    return colors
        .iter()
        .map(|(k, v)| (k.to_string(), hexToRgb(v)))
        .collect::<Vec<(String, (u8, u8, u8))>>();
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
        // Why does colors have to be mut?
        let mut colors = self.colors.iter().map(|(_name, rgb)| (rgb));

        for chunk in self.image_data.chunks(4) {
            let rgb_chunk = utils::rgb(chunk);

            match colors.position(|rgb| rgb_chunk[0] == rgb[0]) {
                Some(index) => *count.entry(index).or_insert(0) += 1, //count.insert(index, 100),
                None => *count.entry(usize::MAX).or_insert(0) += 1, // count.insert(usize::MAX, 100),
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

        return ColorCounterResult { count: result };
    }
}
