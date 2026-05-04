//! Compile-time trait-implementation assertions.
//!
//! Each `const _: fn() = || { ... };` block is type-checked at compile time but
//! never executed. Calling `check::<SomeType>()` forces the compiler to verify
//! that `SomeType` satisfies the named trait bound — if an impl is missing,
//! the crate fails to build.

#![allow(dead_code)]

use crate::*;

// ---------------------------------------------------------------------------
// Scalar traits
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check<T: FloatScalar>() {}
    check::<f32>();
    check::<f64>();
};

const _: fn() = || {
    fn check<T: BoolScalar>() {}
    check::<bool>();
};

const _: fn() = || {
    fn check<T: IntScalar>() {}
    check::<i32>();
    check::<i64>();
};

// ---------------------------------------------------------------------------
// BaseType (every glam type the crate cares about)
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check<T: BaseType>() {}
    check::<glam::Vec2>();
    check::<glam::Vec3>();
    check::<glam::Vec3A>();
    check::<glam::Vec4>();
    check::<glam::DVec2>();
    check::<glam::DVec3>();
    check::<glam::DVec4>();
    check::<glam::Mat2>();
    check::<glam::Mat3>();
    check::<glam::Mat3A>();
    check::<glam::Mat4>();
    check::<glam::DMat2>();
    check::<glam::DMat3>();
    check::<glam::DMat4>();
    check::<glam::Quat>();
    check::<glam::DQuat>();
    check::<glam::Affine2>();
    check::<glam::Affine3>();
    check::<glam::Affine3A>();
    check::<glam::DAffine2>();
    check::<glam::DAffine3>();
    check::<glam::BVec2>();
    check::<glam::BVec3>();
    check::<glam::BVec3A>();
    check::<glam::BVec4>();
    check::<glam::BVec4A>();
    check::<glam::IVec2>();
    check::<glam::IVec3>();
    check::<glam::IVec4>();
    check::<glam::UVec2>();
    check::<glam::UVec3>();
    check::<glam::UVec4>();
    check::<glam::I64Vec2>();
    check::<glam::I64Vec3>();
    check::<glam::I64Vec4>();
    check::<glam::U64Vec2>();
    check::<glam::U64Vec3>();
    check::<glam::U64Vec4>();
};

// ---------------------------------------------------------------------------
// Alignable / Unalignable pairs
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check<T: Alignable>() {}
    check::<glam::Vec3>();
    check::<glam::Mat3>();
    check::<glam::Affine3>();
    check::<glam::BVec3>();
    check::<glam::BVec4>();
};

const _: fn() = || {
    fn check<T: Unalignable>() {}
    check::<glam::Vec3A>();
    check::<glam::Mat3A>();
    check::<glam::Affine3A>();
    check::<glam::BVec3A>();
    check::<glam::BVec4A>();
};

const _: fn() = || {
    fn check<T: LossyAlignable>() {}
    check::<glam::Vec3>();
    check::<glam::Mat3>();
    check::<glam::Affine3>();
    check::<glam::BVec3>();
    check::<glam::BVec4>();
    check::<glam::DVec3>();
    check::<glam::DMat3>();
    check::<glam::DAffine3>();
    // Already-aligned types align to themselves.
    check::<glam::Vec3A>();
    check::<glam::Mat3A>();
    check::<glam::Affine3A>();
};

// ---------------------------------------------------------------------------
// ScalarCastable (floats cast to themselves and across f32 ↔ f64)
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check_self<T: ScalarCastable<f32>>() {}
    check_self::<glam::Vec2>();
    check_self::<glam::Vec3>();
    check_self::<glam::Vec4>();
    check_self::<glam::Quat>();
    check_self::<glam::Mat2>();
    check_self::<glam::Mat3>();
    check_self::<glam::Mat4>();
    check_self::<glam::Affine2>();
    check_self::<glam::Affine3>();

    fn check_dself<T: ScalarCastable<f64>>() {}
    check_dself::<glam::DVec2>();
    check_dself::<glam::DVec3>();
    check_dself::<glam::DVec4>();
    check_dself::<glam::DQuat>();
    check_dself::<glam::DMat2>();
    check_dself::<glam::DMat3>();
    check_dself::<glam::DMat4>();
    check_dself::<glam::DAffine2>();
    check_dself::<glam::DAffine3>();

    // f32 vec types also cast to f64 versions.
    check_dself::<glam::Vec2>();
    check_dself::<glam::Vec3>();
    check_dself::<glam::Vec4>();
    check_dself::<glam::Quat>();
    check_dself::<glam::Mat2>();
    check_dself::<glam::Mat3>();
    check_dself::<glam::Mat4>();
    check_dself::<glam::Affine2>();
    check_dself::<glam::Affine3>();

    // f64 vec types also cast to f32 versions.
    check_self::<glam::DVec2>();
    check_self::<glam::DVec3>();
    check_self::<glam::DVec4>();
    check_self::<glam::DQuat>();
    check_self::<glam::DMat2>();
    check_self::<glam::DMat3>();
    check_self::<glam::DMat4>();
    check_self::<glam::DAffine2>();
    check_self::<glam::DAffine3>();
};

// ---------------------------------------------------------------------------
// Float vector / matrix / affine / quat interfaces
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check_v2<T: TVec2<f32>>() {}
    fn check_v3<T: TVec3<f32>>() {}
    fn check_v4<T: TVec4<f32>>() {}
    fn check_q<T: TQuat<f32>>() {}
    fn check_m2<T: TMat2<f32>>() {}
    fn check_m3<T: TMat3<f32>>() {}
    fn check_m4<T: TMat4<f32>>() {}
    fn check_a2<T: TAffine2<f32>>() {}
    fn check_a3<T: TAffine3<f32>>() {}

    check_v2::<glam::Vec2>();
    check_v3::<glam::Vec3>();
    check_v3::<glam::Vec3A>();
    check_v4::<glam::Vec4>();
    check_q::<glam::Quat>();
    check_m2::<glam::Mat2>();
    check_m3::<glam::Mat3>();
    check_m3::<glam::Mat3A>();
    check_m4::<glam::Mat4>();
    check_a2::<glam::Affine2>();
    check_a3::<glam::Affine3>();
    check_a3::<glam::Affine3A>();
};

const _: fn() = || {
    fn check_v2<T: TVec2<f64>>() {}
    fn check_v3<T: TVec3<f64>>() {}
    fn check_v4<T: TVec4<f64>>() {}
    fn check_q<T: TQuat<f64>>() {}
    fn check_m2<T: TMat2<f64>>() {}
    fn check_m3<T: TMat3<f64>>() {}
    fn check_m4<T: TMat4<f64>>() {}
    fn check_a2<T: TAffine2<f64>>() {}
    fn check_a3<T: TAffine3<f64>>() {}

    check_v2::<glam::DVec2>();
    check_v3::<glam::DVec3>();
    check_v4::<glam::DVec4>();
    check_q::<glam::DQuat>();
    check_m2::<glam::DMat2>();
    check_m3::<glam::DMat3>();
    check_m4::<glam::DMat4>();
    check_a2::<glam::DAffine2>();
    check_a3::<glam::DAffine3>();
};

// Shared float-shape supertraits.
const _: fn() = || {
    fn check_fv<T: FloatVec<f32>>() {}
    check_fv::<glam::Vec2>();
    check_fv::<glam::Vec3>();
    check_fv::<glam::Vec3A>();
    check_fv::<glam::Vec4>();
    fn check_fv64<T: FloatVec<f64>>() {}
    check_fv64::<glam::DVec2>();
    check_fv64::<glam::DVec3>();
    check_fv64::<glam::DVec4>();

    fn check_fm<T: FloatMat<f32>>() {}
    check_fm::<glam::Mat2>();
    check_fm::<glam::Mat3>();
    check_fm::<glam::Mat3A>();
    check_fm::<glam::Mat4>();
    fn check_fm64<T: FloatMat<f64>>() {}
    check_fm64::<glam::DMat2>();
    check_fm64::<glam::DMat3>();
    check_fm64::<glam::DMat4>();

    fn check_fa<T: FloatAffine<f32>>() {}
    check_fa::<glam::Affine2>();
    check_fa::<glam::Affine3>();
    check_fa::<glam::Affine3A>();
    fn check_fa64<T: FloatAffine<f64>>() {}
    check_fa64::<glam::DAffine2>();
    check_fa64::<glam::DAffine3>();
};

// ---------------------------------------------------------------------------
// Bool / int interfaces
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check_bv<T: BoolVec>() {}
    check_bv::<glam::BVec2>();
    check_bv::<glam::BVec3>();
    check_bv::<glam::BVec4>();

    fn check_bv2<T: TBVec2>() {}
    fn check_bv3<T: TBVec3>() {}
    fn check_bv4<T: TBVec4>() {}
    check_bv2::<glam::BVec2>();
    check_bv3::<glam::BVec3>();
    check_bv4::<glam::BVec4>();
};

const _: fn() = || {
    fn check_iv<I: IntScalar, T: IntVec<I>>() {}
    check_iv::<i32, glam::IVec2>();
    check_iv::<i32, glam::IVec3>();
    check_iv::<i32, glam::IVec4>();
    check_iv::<i64, glam::I64Vec2>();
    check_iv::<i64, glam::I64Vec3>();
    check_iv::<i64, glam::I64Vec4>();

    fn check_iv2<I: IntScalar, T: TIVec2<I>>() {}
    fn check_iv3<I: IntScalar, T: TIVec3<I>>() {}
    fn check_iv4<I: IntScalar, T: TIVec4<I>>() {}
    check_iv2::<i32, glam::IVec2>();
    check_iv3::<i32, glam::IVec3>();
    check_iv4::<i32, glam::IVec4>();
    check_iv2::<i64, glam::I64Vec2>();
    check_iv3::<i64, glam::I64Vec3>();
    check_iv4::<i64, glam::I64Vec4>();
};

// ---------------------------------------------------------------------------
// Rotation / transformation supertraits
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check<F: FloatScalar, T: RotationLike3d<F>>() {}
    check::<f32, glam::Mat3>();
    check::<f32, glam::Mat3A>();
    check::<f32, glam::Quat>();
    check::<f64, glam::DMat3>();
    check::<f64, glam::DQuat>();

    fn check_t<F: FloatScalar, T: TransRotLike3d<F>>() {}
    check_t::<f32, glam::Mat4>();
    check_t::<f32, glam::Affine3>();
    check_t::<f32, glam::Affine3A>();
    check_t::<f64, glam::DMat4>();
    check_t::<f64, glam::DAffine3>();
};

// ---------------------------------------------------------------------------
// ArrayLike / ArrayLike2D
// ---------------------------------------------------------------------------

const _: fn() = || {
    fn check2<T: ArrayLike<2>>() {}
    fn check3<T: ArrayLike<3>>() {}
    fn check4<T: ArrayLike<4>>() {}
    fn check6<T: ArrayLike<6>>() {}
    fn check9<T: ArrayLike<9>>() {}
    fn check12<T: ArrayLike<12>>() {}
    fn check16<T: ArrayLike<16>>() {}

    check2::<glam::Vec2>();
    check3::<glam::Vec3>();
    check3::<glam::Vec3A>();
    check4::<glam::Vec4>();
    check9::<glam::Mat3A>();
    check2::<glam::DVec2>();
    check3::<glam::DVec3>();
    check4::<glam::DVec4>();
    check2::<glam::IVec2>();
    check3::<glam::IVec3>();
    check4::<glam::IVec4>();
    check2::<glam::BVec2>();
    check3::<glam::BVec3>();
    check4::<glam::BVec4>();
    check4::<glam::Quat>();
    check4::<glam::DQuat>();
    check4::<glam::Mat2>();
    check9::<glam::Mat3>();
    check16::<glam::Mat4>();
    check4::<glam::DMat2>();
    check9::<glam::DMat3>();
    check16::<glam::DMat4>();

    fn check2d<T: ArrayLike2D>() {}
    check2d::<glam::Mat2>();
    check2d::<glam::Mat3>();
    check2d::<glam::Mat3A>();
    check2d::<glam::Mat4>();
    check2d::<glam::DMat2>();
    check2d::<glam::DMat3>();
    check2d::<glam::DMat4>();
    check2d::<glam::Affine2>();
    check2d::<glam::Affine3>();
    check2d::<glam::Affine3A>();
    check2d::<glam::DAffine2>();
    check2d::<glam::DAffine3>();
};
