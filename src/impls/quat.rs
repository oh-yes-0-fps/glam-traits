use crate::TQuat;

macro_rules! impl_interface_quat {
    ($t:ty, $f:ty, $vec2:ty, $vec3:ty, $vec4:ty) => {
        impl TQuat<$f> for $t {
            type MaybeAligned = $t;

            const IDENTITY: Self = <$t>::IDENTITY;
            const NAN: Self = <$t>::NAN;

            #[inline]
            fn maybe_align(&self) -> Self::MaybeAligned {
                *self
            }

            #[inline]
            fn from_vec4(v: $vec4) -> Self {
                <$t>::from_vec4(v)
            }
            #[inline]
            fn from_scaled_axis(v: $vec3) -> Self {
                <$t>::from_scaled_axis(v)
            }
            #[inline]
            fn from_rotation_axes(x_axis: $vec3, y_axis: $vec3, z_axis: $vec3) -> Self {
                <$t>::from_rotation_axes(x_axis, y_axis, z_axis)
            }
            #[inline]
            fn from_rotation_arc(from: $vec3, to: $vec3) -> Self {
                <$t>::from_rotation_arc(from, to)
            }
            #[inline]
            fn from_rotation_arc_colinear(from: $vec3, to: $vec3) -> Self {
                <$t>::from_rotation_arc_colinear(from, to)
            }
            #[inline]
            fn from_rotation_arc_2d(from: $vec2, to: $vec2) -> Self {
                <$t>::from_rotation_arc_2d(from, to)
            }
            #[inline]
            fn to_scaled_axis(self) -> $vec3 {
                <$t>::to_scaled_axis(self)
            }
            #[inline]
            fn xyz(self) -> $vec3 {
                <$t>::xyz(self)
            }
            #[inline]
            fn conjugate(self) -> Self {
                <$t>::conjugate(self)
            }
            #[inline]
            fn inverse(self) -> Self {
                <$t>::inverse(self)
            }
            #[inline]
            fn dot(self, rhs: Self) -> $f {
                <$t>::dot(self, rhs)
            }
            #[inline]
            fn length(self) -> $f {
                <$t>::length(self)
            }
            #[inline]
            fn length_squared(self) -> $f {
                <$t>::length_squared(self)
            }
            #[inline]
            fn length_recip(self) -> $f {
                <$t>::length_recip(self)
            }
            #[inline]
            fn normalize(self) -> Self {
                <$t>::normalize(self)
            }
            #[inline]
            fn is_finite(self) -> bool {
                <$t>::is_finite(self)
            }
            #[inline]
            fn is_nan(self) -> bool {
                <$t>::is_nan(self)
            }
            #[inline]
            fn is_normalized(self) -> bool {
                <$t>::is_normalized(self)
            }
            #[inline]
            fn is_near_identity(self) -> bool {
                <$t>::is_near_identity(self)
            }
            #[inline]
            fn angle_between(self, rhs: Self) -> $f {
                <$t>::angle_between(self, rhs)
            }
            #[inline]
            fn rotate_towards(self, rhs: Self, max_angle: $f) -> Self {
                <$t>::rotate_towards(self, rhs, max_angle)
            }
            #[inline]
            fn abs_diff_eq(self, rhs: Self, max_abs_diff: $f) -> bool {
                <$t>::abs_diff_eq(self, rhs, max_abs_diff)
            }
            #[inline]
            fn lerp(self, end: Self, s: $f) -> Self {
                <$t>::lerp(self, end, s)
            }
            #[inline]
            fn slerp(self, end: Self, s: $f) -> Self {
                <$t>::slerp(self, end, s)
            }
            #[inline]
            fn mul_vec3(self, rhs: $vec3) -> $vec3 {
                <$t>::mul_vec3(self, rhs)
            }
            #[inline]
            fn mul_quat(self, rhs: Self) -> Self {
                <$t>::mul_quat(self, rhs)
            }
            #[inline]
            fn from_slice(slice: &[$f]) -> Self {
                <$t>::from_slice(slice)
            }
            #[inline]
            fn write_to_slice(self, slice: &mut [$f]) {
                <$t>::write_to_slice(self, slice)
            }
        }
    };
}

impl_interface_quat!(glam::Quat, f32, glam::Vec2, glam::Vec3, glam::Vec4);
impl_interface_quat!(glam::DQuat, f64, glam::DVec2, glam::DVec3, glam::DVec4);
