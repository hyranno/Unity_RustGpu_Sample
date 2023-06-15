#![no_std]

// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
// #![deny(warnings)]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(
    output: &mut Vec4,
) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    in_position: Vec4,
    #[spirv(position)] out_position: &mut Vec4,
) {
    *out_position = in_position;
}