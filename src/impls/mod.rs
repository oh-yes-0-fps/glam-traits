mod support;
mod rotation;
mod float_vec;
mod float_mat;
mod float_affine;
mod quat;
mod bool_vec;
mod int_vec;
mod assertions;

use crate::{BoolScalar, FloatScalar, IntScalar};

impl FloatScalar for f32 {
    type Vec2 = glam::Vec2;
    type Vec3 = glam::Vec3;
    type Vec4 = glam::Vec4;
    type Quat = glam::Quat;
    type Mat2 = glam::Mat2;
    type Mat3 = glam::Mat3;
    type Mat4 = glam::Mat4;
    type Affine2 = glam::Affine2;
    type Affine3 = glam::Affine3;
}

impl FloatScalar for f64 {
    type Vec2 = glam::DVec2;
    type Vec3 = glam::DVec3;
    type Vec4 = glam::DVec4;
    type Quat = glam::DQuat;
    type Mat2 = glam::DMat2;
    type Mat3 = glam::DMat3;
    type Mat4 = glam::DMat4;
    type Affine2 = glam::DAffine2;
    type Affine3 = glam::DAffine3;
}

impl BoolScalar for bool {
    type Vec2 = glam::BVec2;
    type Vec3 = glam::BVec3;
    type Vec4 = glam::BVec4;
}

impl IntScalar for i32 {
    type Vec2 = glam::IVec2;
    type Vec3 = glam::IVec3;
    type Vec4 = glam::IVec4;
}

impl IntScalar for i64 {
    type Vec2 = glam::I64Vec2;
    type Vec3 = glam::I64Vec3;
    type Vec4 = glam::I64Vec4;
}
