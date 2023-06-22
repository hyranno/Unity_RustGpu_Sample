#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
// #![deny(warnings)]

use spirv_std::glam::{vec4, Vec4, Mat4};
use spirv_std::spirv;

#[derive(Copy, Clone)]
// #[spirv(matrix)]
#[repr(C)]
pub struct Mat4SPIRV {
    pub value: [Vec4; 4],
}
impl From<Mat4> for Mat4SPIRV {
    fn from(value: Mat4) -> Self {
        Self{
            value: [value.x_axis, value.y_axis, value.z_axis, value.w_axis],
        }
    }
}
impl Into<Mat4> for Mat4SPIRV {
    fn into(self) -> Mat4 {
        Mat4::from_cols(self.value[0], self.value[1], self.value[2], self.value[3],)
    }
}

#[repr(C)]
pub struct UnityPerFrameBuffer {
    unity_MatrixVP: Mat4SPIRV,
}

#[repr(C)]
pub struct UnityPerDrawBuffer {
    unity_ObjectToWorld: Mat4SPIRV,
    unity_WorldToObject: Mat4SPIRV,
}

fn UnityObjectToClipPos(mat_vp: &Mat4, obj_to_world: &Mat4, pos: Vec4) -> Vec4 {
    mat_vp.mul_vec4(obj_to_world.mul_vec4(pos))
}

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
    #[spirv(uniform)] UnityPerFrame: &UnityPerFrameBuffer,
    #[spirv(uniform)] UnityPerDraw: &UnityPerDrawBuffer,
) {
    *out_position = UnityObjectToClipPos(
        &UnityPerFrame.unity_MatrixVP.into(),
        &UnityPerDraw.unity_ObjectToWorld.into(),
        in_position
    );
}
