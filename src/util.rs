use std::ops::{Add, Mul};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Vector2i {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

impl Add for Vector2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vector2i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2i {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Vector2i {
    pub const ZERO: Self = Vector2i { x: 0, y: 0 };
    pub const ONE: Self = Vector2i { x: 1, y: 1 };

    pub const X_AXIS: Self = Vector2i { x: 1, y: 0 };
    pub const Y_AXIS: Self = Vector2i { x: 0, y: 1 };
}
