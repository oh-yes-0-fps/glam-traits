#![cfg(feature = "Valuable")]

use crate::GlamValuable;
use ::valuable::{Listable, Slice, Valuable, Value, Visit};

macro_rules! impl_asref_primitive {
    ($ty:ty, $elem:ty, $n:expr, $variant:ident) => {
        impl GlamValuable for $ty {
            #[inline]
            fn as_value(&self) -> Value<'_> {
                let arr: &[$elem; $n] = self.as_ref();
                Value::Listable(arr)
            }
            #[inline]
            fn visit(&self, visit: &mut dyn Visit) {
                let arr: &[$elem; $n] = self.as_ref();
                visit.visit_primitive_slice(Slice::$variant(arr));
            }
        }
    };
}

impl_asref_primitive!(glam::Vec2, f32, 2, F32);
impl_asref_primitive!(glam::Vec3, f32, 3, F32);
impl_asref_primitive!(glam::Vec3A, f32, 3, F32);
impl_asref_primitive!(glam::Vec4, f32, 4, F32);
impl_asref_primitive!(glam::Mat2, f32, 4, F32);
impl_asref_primitive!(glam::Mat3, f32, 9, F32);
impl_asref_primitive!(glam::Mat4, f32, 16, F32);
impl_asref_primitive!(glam::Quat, f32, 4, F32);

impl_asref_primitive!(glam::DVec2, f64, 2, F64);
impl_asref_primitive!(glam::DVec3, f64, 3, F64);
impl_asref_primitive!(glam::DVec4, f64, 4, F64);
impl_asref_primitive!(glam::DMat2, f64, 4, F64);
impl_asref_primitive!(glam::DMat3, f64, 9, F64);
impl_asref_primitive!(glam::DMat4, f64, 16, F64);
impl_asref_primitive!(glam::DQuat, f64, 4, F64);

impl_asref_primitive!(glam::IVec2, i32, 2, I32);
impl_asref_primitive!(glam::IVec3, i32, 3, I32);
impl_asref_primitive!(glam::IVec4, i32, 4, I32);
impl_asref_primitive!(glam::I64Vec2, i64, 2, I64);
impl_asref_primitive!(glam::I64Vec3, i64, 3, I64);
impl_asref_primitive!(glam::I64Vec4, i64, 4, I64);

// Types without `AsRef<[T; N]>` are bridged through a private
// `#[repr(transparent)]` wrapper so that `as_value` can return
// `Value::Listable(&wrapper)` with a borrow tied to `&self`.
macro_rules! impl_wrapped {
    ($wrap:ident, $ty:ty, $elem:ty, $n:expr, $variant:ident, $to_array:expr) => {
        #[repr(transparent)]
        struct $wrap($ty);

        impl Valuable for $wrap {
            #[inline]
            fn as_value(&self) -> Value<'_> {
                Value::Listable(self)
            }
            #[inline]
            fn visit(&self, visit: &mut dyn Visit) {
                let arr: [$elem; $n] = $to_array(&self.0);
                visit.visit_primitive_slice(Slice::$variant(&arr));
            }
        }

        impl Listable for $wrap {
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                ($n, Some($n))
            }
        }

        impl GlamValuable for $ty {
            #[inline]
            fn as_value(&self) -> Value<'_> {
                // SAFETY: `$wrap` is `#[repr(transparent)]` over `$ty`, so a
                // `&$ty` and a `&$wrap` have the same memory layout.
                let wrapped: &$wrap = unsafe { &*(self as *const $ty as *const $wrap) };
                Value::Listable(wrapped)
            }
            #[inline]
            fn visit(&self, visit: &mut dyn Visit) {
                let arr: [$elem; $n] = $to_array(self);
                visit.visit_primitive_slice(Slice::$variant(&arr));
            }
        }
    };
}

impl_wrapped!(Mat3AVal, glam::Mat3A, f32, 9, F32, |m: &glam::Mat3A| m.to_cols_array());
impl_wrapped!(Affine2Val, glam::Affine2, f32, 6, F32, |a: &glam::Affine2| a.to_cols_array());
impl_wrapped!(Affine3Val, glam::Affine3, f32, 12, F32, |a: &glam::Affine3| a.to_cols_array());
impl_wrapped!(Affine3AVal, glam::Affine3A, f32, 12, F32, |a: &glam::Affine3A| a.to_cols_array());
impl_wrapped!(DAffine2Val, glam::DAffine2, f64, 6, F64, |a: &glam::DAffine2| a.to_cols_array());
impl_wrapped!(DAffine3Val, glam::DAffine3, f64, 12, F64, |a: &glam::DAffine3| a.to_cols_array());

impl_wrapped!(BVec2Val, glam::BVec2, bool, 2, Bool, |b: &glam::BVec2| <[bool; 2]>::from(*b));
impl_wrapped!(BVec3Val, glam::BVec3, bool, 3, Bool, |b: &glam::BVec3| <[bool; 3]>::from(*b));
impl_wrapped!(BVec4Val, glam::BVec4, bool, 4, Bool, |b: &glam::BVec4| <[bool; 4]>::from(*b));
