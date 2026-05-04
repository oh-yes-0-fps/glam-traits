use crate::{FloatVec, TVec2, TVec3, TVec4};

macro_rules! impl_float_vec {
    ($vec:ty, $f:ty) => {
        impl FloatVec<$f> for $vec {
            const ZERO: Self = <$vec>::ZERO;
            const ONE: Self = <$vec>::ONE;
            const NEG_ONE: Self = <$vec>::NEG_ONE;
            const MIN: Self = <$vec>::MIN;
            const MAX: Self = <$vec>::MAX;
            const NAN: Self = <$vec>::NAN;
            const INFINITY: Self = <$vec>::INFINITY;
            const NEG_INFINITY: Self = <$vec>::NEG_INFINITY;

            #[inline]
            fn splat(value: $f) -> Self {
                <$vec>::splat(value)
            }
            #[inline]
            fn dot(self, rhs: Self) -> $f {
                <$vec>::dot(self, rhs)
            }
            #[inline]
            fn min(self, rhs: Self) -> Self {
                <$vec>::min(self, rhs)
            }
            #[inline]
            fn max(self, rhs: Self) -> Self {
                <$vec>::max(self, rhs)
            }
            #[inline]
            fn min_element(self) -> $f {
                <$vec>::min_element(self)
            }
            #[inline]
            fn max_element(self) -> $f {
                <$vec>::max_element(self)
            }
            #[inline]
            fn min_position(self) -> usize {
                <$vec>::min_position(self)
            }
            #[inline]
            fn max_position(self) -> usize {
                <$vec>::max_position(self)
            }
            #[inline]
            fn element_sum(self) -> $f {
                <$vec>::element_sum(self)
            }
            #[inline]
            fn element_product(self) -> $f {
                <$vec>::element_product(self)
            }
            #[inline]
            fn abs(self) -> Self {
                <$vec>::abs(self)
            }
            #[inline]
            fn signum(self) -> Self {
                <$vec>::signum(self)
            }
            #[inline]
            fn copysign(self, rhs: Self) -> Self {
                <$vec>::copysign(self, rhs)
            }
            #[inline]
            fn is_finite(self) -> bool {
                <$vec>::is_finite(self)
            }
            #[inline]
            fn is_nan(self) -> bool {
                <$vec>::is_nan(self)
            }
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                <$vec>::div_euclid(self, rhs)
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                <$vec>::rem_euclid(self, rhs)
            }
            #[inline]
            fn is_normalized(self) -> bool {
                <$vec>::is_normalized(self)
            }
            #[inline]
            fn round(self) -> Self {
                <$vec>::round(self)
            }
            #[inline]
            fn floor(self) -> Self {
                <$vec>::floor(self)
            }
            #[inline]
            fn ceil(self) -> Self {
                <$vec>::ceil(self)
            }
            #[inline]
            fn trunc(self) -> Self {
                <$vec>::trunc(self)
            }
            #[inline]
            fn fract(self) -> Self {
                <$vec>::fract(self)
            }
            #[inline]
            fn fract_gl(self) -> Self {
                <$vec>::fract_gl(self)
            }
            #[inline]
            fn step(self, rhs: Self) -> Self {
                <$vec>::step(self, rhs)
            }
            #[inline]
            fn exp(self) -> Self {
                <$vec>::exp(self)
            }
            #[inline]
            fn exp2(self) -> Self {
                <$vec>::exp2(self)
            }
            #[inline]
            fn ln(self) -> Self {
                <$vec>::ln(self)
            }
            #[inline]
            fn log2(self) -> Self {
                <$vec>::log2(self)
            }
            #[inline]
            fn powf(self, n: $f) -> Self {
                <$vec>::powf(self, n)
            }
            #[inline]
            fn sqrt(self) -> Self {
                <$vec>::sqrt(self)
            }
            #[inline]
            fn cos(self) -> Self {
                <$vec>::cos(self)
            }
            #[inline]
            fn sin(self) -> Self {
                <$vec>::sin(self)
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                <$vec>::sin_cos(self)
            }
            #[inline]
            fn recip(self) -> Self {
                <$vec>::recip(self)
            }
            #[inline]
            fn move_towards(self, rhs: Self, d: $f) -> Self {
                <$vec>::move_towards(self, rhs, d)
            }
            #[inline]
            fn midpoint(self, rhs: Self) -> Self {
                <$vec>::midpoint(self, rhs)
            }
            #[inline]
            fn abs_diff_eq(self, rhs: Self, max_abs_diff: $f) -> bool {
                <$vec>::abs_diff_eq(self, rhs, max_abs_diff)
            }
            #[inline]
            fn clamp_length(self, min: $f, max: $f) -> Self {
                <$vec>::clamp_length(self, min, max)
            }
            #[inline]
            fn clamp_length_max(self, max: $f) -> Self {
                <$vec>::clamp_length_max(self, max)
            }
            #[inline]
            fn clamp_length_min(self, min: $f) -> Self {
                <$vec>::clamp_length_min(self, min)
            }
            #[inline]
            fn reflect(self, normal: Self) -> Self {
                <$vec>::reflect(self, normal)
            }
            #[inline]
            fn refract(self, normal: Self, eta: $f) -> Self {
                <$vec>::refract(self, normal, eta)
            }
            #[inline]
            fn from_slice(slice: &[$f]) -> Self {
                <$vec>::from_slice(slice)
            }
            #[inline]
            fn write_to_slice(self, slice: &mut [$f]) {
                <$vec>::write_to_slice(self, slice)
            }
        }
    };
}

impl_float_vec!(glam::Vec2, f32);
impl_float_vec!(glam::Vec3, f32);
impl_float_vec!(glam::Vec4, f32);
impl_float_vec!(glam::DVec2, f64);
impl_float_vec!(glam::DVec3, f64);
impl_float_vec!(glam::DVec4, f64);

macro_rules! impl_interface_vec2 {
    ($vec:ty, $f:ty) => {
        impl TVec2<$f> for $vec {
            const X: Self = <$vec>::X;
            const Y: Self = <$vec>::Y;
            const NEG_X: Self = <$vec>::NEG_X;
            const NEG_Y: Self = <$vec>::NEG_Y;
            const AXES: [Self; 2] = <$vec>::AXES;

            #[inline]
            fn with_x(self, x: $f) -> Self {
                <$vec>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $f) -> Self {
                <$vec>::with_y(self, y)
            }
            #[inline]
            fn to_angle(self) -> $f {
                <$vec>::to_angle(self)
            }
            #[inline]
            fn angle_to(self, rhs: Self) -> $f {
                <$vec>::angle_to(self, rhs)
            }
            #[inline]
            fn perp(self) -> Self {
                <$vec>::perp(self)
            }
            #[inline]
            fn perp_dot(self, rhs: Self) -> $f {
                <$vec>::perp_dot(self, rhs)
            }
            #[inline]
            fn rotate(self, rhs: Self) -> Self {
                <$vec>::rotate(self, rhs)
            }
            #[inline]
            fn rotate_towards(self, rhs: Self, max_angle: $f) -> Self {
                <$vec>::rotate_towards(self, rhs, max_angle)
            }
        }
    };
}

impl_interface_vec2!(glam::Vec2, f32);
impl_interface_vec2!(glam::DVec2, f64);

macro_rules! impl_interface_vec3 {
    ($vec:ty, $f:ty, $vec4:ty) => {
        impl TVec3<$f> for $vec {
            const X: Self = <$vec>::X;
            const Y: Self = <$vec>::Y;
            const Z: Self = <$vec>::Z;
            const NEG_X: Self = <$vec>::NEG_X;
            const NEG_Y: Self = <$vec>::NEG_Y;
            const NEG_Z: Self = <$vec>::NEG_Z;
            const AXES: [Self; 3] = <$vec>::AXES;

            #[inline]
            fn with_x(self, x: $f) -> Self {
                <$vec>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $f) -> Self {
                <$vec>::with_y(self, y)
            }
            #[inline]
            fn with_z(self, z: $f) -> Self {
                <$vec>::with_z(self, z)
            }
            #[inline]
            fn cross(self, rhs: Self) -> Self {
                <$vec>::cross(self, rhs)
            }
            #[inline]
            fn angle_between(self, rhs: Self) -> $f {
                <$vec>::angle_between(self, rhs)
            }
            #[inline]
            fn any_orthogonal_vector(&self) -> Self {
                <$vec>::any_orthogonal_vector(*self)
            }
            #[inline]
            fn any_orthonormal_vector(&self) -> Self {
                <$vec>::any_orthonormal_vector(*self)
            }
            #[inline]
            fn any_orthonormal_pair(&self) -> (Self, Self) {
                <$vec>::any_orthonormal_pair(*self)
            }
            #[inline]
            fn from_homogeneous(v: $vec4) -> Option<Self> {
                Some(<$vec>::from_homogeneous(v))
            }
            #[inline]
            fn to_homogeneous(self) -> $vec4 {
                <$vec>::to_homogeneous(self)
            }
        }
    };
}

impl_interface_vec3!(glam::Vec3, f32, glam::Vec4);
impl_interface_vec3!(glam::DVec3, f64, glam::DVec4);

macro_rules! impl_interface_vec4 {
    ($vec:ty, $f:ty, $vec3:ty) => {
        impl TVec4<$f> for $vec {
            const X: Self = <$vec>::X;
            const Y: Self = <$vec>::Y;
            const Z: Self = <$vec>::Z;
            const W: Self = <$vec>::W;
            const NEG_X: Self = <$vec>::NEG_X;
            const NEG_Y: Self = <$vec>::NEG_Y;
            const NEG_Z: Self = <$vec>::NEG_Z;
            const NEG_W: Self = <$vec>::NEG_W;
            const AXES: [Self; 4] = <$vec>::AXES;

            #[inline]
            fn with_x(self, x: $f) -> Self {
                <$vec>::with_x(self, x)
            }
            #[inline]
            fn with_y(self, y: $f) -> Self {
                <$vec>::with_y(self, y)
            }
            #[inline]
            fn with_z(self, z: $f) -> Self {
                <$vec>::with_z(self, z)
            }
            #[inline]
            fn with_w(self, w: $f) -> Self {
                <$vec>::with_w(self, w)
            }
            #[inline]
            fn project(self) -> $vec3 {
                <$vec>::project(self)
            }
        }
    };
}

impl_interface_vec4!(glam::Vec4, f32, glam::Vec3);
impl_interface_vec4!(glam::DVec4, f64, glam::DVec3);
