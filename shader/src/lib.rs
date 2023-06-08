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
    #[spirv(vertex_index)] vertex_id: i32,
    #[spirv(position)] out_position: &mut Vec4,
) {
    *out_position = vec4(
        vertex_id as f32 - 1.0,
        (vertex_id & 1) as f32 * 2.0 - 1.0,
        0.0,
        1.0,
    );
}