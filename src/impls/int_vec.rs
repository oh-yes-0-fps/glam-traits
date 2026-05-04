use crate::{TIVec2, TIVec3, TIVec4, IntVec};

macro_rules! impl_int_vec {
    ($t:ty, $i:ty, $mask:ty) => {
        impl IntVec<$i> for $t {
            type Mask = $mask;

            const ZERO: Self = <$t>::ZERO;
            const ONE: Self = <$t>::ONE;
            const MIN: Self = <$t>::MIN;
            const MAX: Self = <$t>::MAX;

            #[inline]
            fn splat(value: $i) -> Self {
                <$t>::splat(value)
            }
            #[inline]
            fn dot(self, rhs: Self) -> $i {
                <$t>::dot(self, rhs)
            }
            #[inline]
            fn min(self, rhs: Self) -> Self {
                <$t>::min(self, rhs)
            }
            #[inline]
            fn max(self, rhs: Self) -> Self {
                <$t>::max(self, rhs)
            }
            #[inline]
            fn min_element(self) -> $i {
                <$t>::min_element(self)
            }
            #[inline]
            fn max_element(self) -> $i {
                <$t>::max_element(self)
            }
            #[inline]
            fn min_position(self) -> usize {
                <$t>::min_position(self)
            }
            #[inline]
            fn max_position(self) -> usize {
                <$t>::max_position(self)
            }
            #[inline]
            fn element_sum(self) -> $i {
                <$t>::element_sum(self)
            }
            #[inline]
            fn element_product(self) -> $i {
                <$t>::element_product(self)
            }
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                <$t>::div_euclid(self, rhs)
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                <$t>::rem_euclid(self, rhs)
            }
            #[inline]
            fn cmpeq(self, rhs: Self) -> Self::Mask {
                <$t>::cmpeq(self, rhs)
            }
            #[inline]
            fn cmpne(self, rhs: Self) -> Self::Mask {
                <$t>::cmpne(self, rhs)
            }
            #[inline]
            fn cmpge(self, rhs: Self) -> Self::Mask {
                <$t>::cmpge(self, rhs)
            }
            #[inline]
            fn cmpgt(self, rhs: Self) -> Self::Mask {
                <$t>::cmpgt(self, rhs)
            }
            #[inline]
            fn cmple(self, rhs: Self) -> Self::Mask {
                <$t>::cmple(self, rhs)
            }
            #[inline]
            fn cmplt(self, rhs: Self) -> Self::Mask {
                <$t>::cmplt(self, rhs)
            }
            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$t>::checked_add(self, rhs)
            }
            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$t>::checked_sub(self, rhs)
            }
            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                <$t>::checked_mul(self, rhs)
            }
            #[inline]
            fn checked_div(self, rhs: Self) -> Option<Self> {
                <$t>::checked_div(self, rhs)
            }
            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }
            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self {
                <$t>::wrapping_sub(self, rhs)
            }
            #[inline]
            fn wrapping_mul(self, rhs: Self) -> Self {
                <$t>::wrapping_mul(self, rhs)
            }
            #[inline]
            fn wrapping_div(self, rhs: Self) -> Self {
                <$t>::wrapping_div(self, rhs)
            }
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                <$t>::saturating_add(self, rhs)
            }
            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self {
                <$t>::saturating_sub(self, rhs)
            }
            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self {
                <$t>::saturating_mul(self, rhs)
            }
            #[inline]
            fn saturating_div(self, rhs: Self) -> Self {
                <$t>::saturating_div(self, rhs)
            }
            #[inline]
            fn from_slice(slice: &[$i]) -> Self {
                <$t>::from_slice(slice)
            }
            #[inline]
            fn write_to_slice(self, slice: &mut [$i]) {
                <$t>::write_to_slice(self, slice)
            }
        }
    };
}

impl_int_vec!(glam::IVec2, i32, glam::BVec2);
impl_int_vec!(glam::IVec3, i32, glam::BVec3);
impl_int_vec!(glam::IVec4, i32, glam::BVec4);
impl_int_vec!(glam::I64Vec2, i64, glam::BVec2);
impl_int_vec!(glam::I64Vec3, i64, glam::BVec3);
impl_int_vec!(glam::I64Vec4, i64, glam::BVec4);

macro_rules! impl_interface_ivec2 {
    ($t:ty, $i:ty, $vec3:ty) => {
        impl TIVec2<$i> for $t {
            const X: Self = <$t>::X;
            const Y: Self = <$t>::Y;
            const AXES: [Self; 2] = <$t>::AXES;

            #[inline]
            fn with_x(self, x: $i) -> Self {
                <$t>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $i) -> Self {
                <$t>::with_y(self, y)
            }
            #[inline]
            fn extend(self, z: $i) -> $vec3 {
                <$t>::extend(self, z)
            }
            #[inline]
            fn perp(self) -> Self {
                <$t>::perp(self)
            }
            #[inline]
            fn perp_dot(self, rhs: Self) -> $i {
                <$t>::perp_dot(self, rhs)
            }
            #[inline]
            fn rotate(self, rhs: Self) -> Self {
                <$t>::rotate(self, rhs)
            }
        }
    };
}

impl_interface_ivec2!(glam::IVec2, i32, glam::IVec3);
impl_interface_ivec2!(glam::I64Vec2, i64, glam::I64Vec3);

// UVec2 doesn't impl perp/perp_dot/rotate (those need signed). Skip.
// We'd need a separate trait for unsigned to express that cleanly.
// For now, signed-only:

macro_rules! impl_interface_ivec3 {
    ($t:ty, $i:ty, $vec2:ty, $vec4:ty) => {
        impl TIVec3<$i> for $t {
            const X: Self = <$t>::X;
            const Y: Self = <$t>::Y;
            const Z: Self = <$t>::Z;
            const AXES: [Self; 3] = <$t>::AXES;

            #[inline]
            fn with_x(self, x: $i) -> Self {
                <$t>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $i) -> Self {
                <$t>::with_y(self, y)
            }
            #[inline]
            fn with_z(self, z: $i) -> Self {
                <$t>::with_z(self, z)
            }
            #[inline]
            fn extend(self, w: $i) -> $vec4 {
                <$t>::extend(self, w)
            }
            #[inline]
            fn truncate(self) -> $vec2 {
                <$t>::truncate(self)
            }
            #[inline]
            fn cross(self, rhs: Self) -> Self {
                <$t>::cross(self, rhs)
            }
        }
    };
}

impl_interface_ivec3!(glam::IVec3, i32, glam::IVec2, glam::IVec4);
impl_interface_ivec3!(glam::I64Vec3, i64, glam::I64Vec2, glam::I64Vec4);

macro_rules! impl_interface_ivec4 {
    ($t:ty, $i:ty, $vec3:ty) => {
        impl TIVec4<$i> for $t {
            const X: Self = <$t>::X;
            const Y: Self = <$t>::Y;
            const Z: Self = <$t>::Z;
            const W: Self = <$t>::W;
            const AXES: [Self; 4] = <$t>::AXES;

            #[inline]
            fn with_x(self, x: $i) -> Self {
                <$t>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $i) -> Self {
                <$t>::with_y(self, y)
            }
            #[inline]
            fn with_z(self, z: $i) -> Self {
                <$t>::with_z(self, z)
            }
            #[inline]
            fn with_w(self, w: $i) -> Self {
                <$t>::with_w(self, w)
            }
            #[inline]
            fn truncate(self) -> $vec3 {
                <$t>::truncate(self)
            }
        }
    };
}

impl_interface_ivec4!(glam::IVec4, i32, glam::IVec3);
impl_interface_ivec4!(glam::I64Vec4, i64, glam::I64Vec3);
