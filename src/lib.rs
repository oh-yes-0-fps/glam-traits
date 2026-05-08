#![allow(private_bounds)]

mod impls;

use std::hash::Hash;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
};

use num_traits::{Float, PrimInt};


pub trait Alignable {
    type Aligned: Copy + Sized + Unalignable<Unaligned = Self>;

    #[must_use]
    fn exact_align(&self) -> Self::Aligned;
}

pub trait LossyAlignable {
    type Aligned: Copy + Sized;

    #[must_use]
    fn align(&self) -> Self::Aligned;
}

impl<T: Alignable> LossyAlignable for T {
    type Aligned = T::Aligned;

    fn align(&self) -> Self::Aligned {
        self.exact_align()
    }
}

pub trait Unalignable {
    type Unaligned: Copy + Sized;

    #[must_use]
    fn unalign(&self) -> Self::Unaligned;
}

pub trait RotationLike3d<F: FloatScalar>: Sized {
    #[must_use]
    fn to_mat3(&self) -> F::Mat3;
    #[must_use]
    fn to_quat(&self) -> F::Quat;
    #[must_use]
    fn to_euler(&self, order: glam::EulerRot) -> F::Vec3;
    #[must_use]
    fn to_axis_angle(&self) -> (F::Vec3, F);
    #[must_use]
    fn to_rodrigues(&self) -> F::Vec3;
    #[must_use]
    fn to_rot_vec(&self) -> F::Vec3;
    #[must_use]
    fn from_mat3(mat: F::Mat3) -> Self;
    #[must_use]
    fn from_quat(quat: F::Quat) -> Self;
    #[must_use]
    fn from_euler(euler: F::Vec3, order: glam::EulerRot) -> Self;
    #[must_use]
    fn from_axis_angle(axis: F::Vec3, angle: F) -> Self;
    #[must_use]
    fn from_rodrigues(rodrigues: F::Vec3) -> Self;
    #[must_use]
    fn from_rot_vec(rot_vec: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_x(angle: F) -> Self;
    #[must_use]
    fn from_rotation_y(angle: F) -> Self;
    #[must_use]
    fn from_rotation_z(angle: F) -> Self;
    #[must_use]
    fn from_angle(angle: F) -> Self;
    #[must_use]
    fn look_to_lh(dir: F::Vec3, up: F::Vec3) -> Self;
    #[must_use]
    fn look_to_rh(dir: F::Vec3, up: F::Vec3) -> Self;
    #[must_use]
    fn look_at_lh(eye: F::Vec3, center: F::Vec3, up: F::Vec3) -> Self;
    #[must_use]
    fn look_at_rh(eye: F::Vec3, center: F::Vec3, up: F::Vec3) -> Self;
}

pub trait TransRotLike3d<F: FloatScalar>: Sized {
    #[must_use]
    fn to_affine3(&self) -> F::Affine3;
    #[must_use]
    fn to_mat4(&self) -> F::Mat4;
    #[must_use]
    fn from_affine3(affine: F::Affine3) -> Self;
    #[must_use]
    fn from_mat4(mat: F::Mat4) -> Self;
}

pub trait ScalarCastable<F: FloatScalar>: Sized {
    const LOSSY_CAST: bool;
    type Casted;

    #[must_use]
    fn cast(&self) -> Self::Casted;
}

pub trait IntScalarCastable<F: IntScalar>: Sized {
    const LOSSY_CAST: bool;
    type Casted;

    #[must_use]
    fn cast(&self) -> Self::Casted;
}

pub trait ArrayLike<const N: usize>: Sized {
    type Element;
    const LENGTH: usize = N;

    #[must_use]
    fn from_array(arr: [Self::Element; N]) -> Self;
    #[must_use]
    fn to_array(&self) -> [Self::Element; N];
    #[cfg(feature = "ndarray")]
    #[must_use]
    fn to_ndarray(&self) -> ndarray::Array1<Self::Element>;
    #[cfg(feature = "ndarray")]
    #[must_use]
    fn from_ndarray(arr: ndarray::Array1<Self::Element>) -> Self;
}

pub trait ArrayLike2D {
    type Element;
    const ROWS: usize;
    const COLS: usize;

    #[cfg(feature = "ndarray")]
    #[must_use]
    fn from_ndarray2d(arr: ndarray::Array2<Self::Element>) -> Self;
    #[cfg(feature = "ndarray")]
    #[must_use]
    fn to_ndarray2d(&self) -> ndarray::Array2<Self::Element>;
}

pub trait FloatScalar: Float {
    type Vec2: TVec2<Self>;
    type Vec3: TVec3<Self>;
    type Vec4: TVec4<Self>;
    type Quat: TQuat<Self>;
    type Mat2: TMat2<Self>;
    type Mat3: TMat3<Self>;
    type Mat4: TMat4<Self>;
    type Affine2: TAffine2<Self>;
    type Affine3: TAffine3<Self>;
}

pub trait BoolScalar: Sized + Eq + Send + Sync + 'static {
    type Vec2: TBVec2;
    type Vec3: TBVec3;
    type Vec4: TBVec4;
}

pub trait IntScalar: PrimInt {
    type Vec2: TIVec2<Self>;
    type Vec3: TIVec3<Self>;
    type Vec4: TIVec4<Self>;
}

pub(crate) trait BaseType:
    std::fmt::Debug + Copy + Sized + Send + Sync + 'static + SerdeShim + ArbitraryShim + Into<Self> + From<Self>
{
}

#[cfg(feature = "approx")]
trait ApproxFloat<F>:
    approx::AbsDiffEq<Epsilon = F> + approx::RelativeEq + approx::UlpsEq
{
}

#[cfg(feature = "approx")]
impl<T, F> ApproxFloat<F> for T where
    T: approx::AbsDiffEq<Epsilon = F> + approx::RelativeEq + approx::UlpsEq
{
}

#[cfg(not(feature = "approx"))]
trait ApproxFloat<F> {}

#[cfg(not(feature = "approx"))]
impl<T, F> ApproxFloat<F> for T {}

#[cfg(feature = "serde")]
trait SerdeShim: serde::Serialize + serde::de::DeserializeOwned {}
#[cfg(feature = "serde")]
impl<T: serde::Serialize + serde::de::DeserializeOwned> SerdeShim for T {}
#[cfg(not(feature = "serde"))]
trait SerdeShim {}
#[cfg(not(feature = "serde"))]
impl<T> SerdeShim for T {}

#[cfg(feature = "bytemuck")]
trait BytemuckShim: bytemuck::Pod + bytemuck::Zeroable {}
#[cfg(feature = "bytemuck")]
impl<T: bytemuck::Pod + bytemuck::Zeroable> BytemuckShim for T {}
#[cfg(not(feature = "bytemuck"))]
trait BytemuckShim {}
#[cfg(not(feature = "bytemuck"))]
impl<T> BytemuckShim for T {}

#[cfg(feature = "zerocopy")]
trait ZerocopyShim:
    zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::Immutable + zerocopy::KnownLayout
{
}
#[cfg(feature = "zerocopy")]
impl<T> ZerocopyShim for T where
    T: zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::Immutable + zerocopy::KnownLayout
{
}
#[cfg(not(feature = "zerocopy"))]
trait ZerocopyShim {}
#[cfg(not(feature = "zerocopy"))]
impl<T> ZerocopyShim for T {}

#[cfg(feature = "arbitrary")]
trait ArbitraryShim: for<'a> arbitrary::Arbitrary<'a> {}
#[cfg(feature = "arbitrary")]
impl<T: for<'a> arbitrary::Arbitrary<'a>> ArbitraryShim for T {}
#[cfg(not(feature = "arbitrary"))]
trait ArbitraryShim {}
#[cfg(not(feature = "arbitrary"))]
impl<T> ArbitraryShim for T {}

/// Mirror of [`valuable::Valuable`] for `glam` types. Available behind the
/// `Valuable` feature.
#[cfg(feature = "Valuable")]
pub trait GlamValuable {
    fn as_value(&self) -> valuable::Value<'_>;
    fn visit(&self, visit: &mut dyn valuable::Visit);
    fn visit_slice(slice: &[Self], visit: &mut dyn valuable::Visit)
    where
        Self: Sized,
    {
        for item in slice {
            visit.visit_value(item.as_value());
        }
    }
}

pub trait FloatVec<F: FloatScalar>:
    BaseType
    + ApproxFloat<F>
    + BytemuckShim
    + ZerocopyShim
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + Add<F, Output = Self>
    + AddAssign<Self>
    + AddAssign<F>
    + Sub<Self, Output = Self>
    + Sub<F, Output = Self>
    + SubAssign<Self>
    + SubAssign<F>
    + Mul<Self, Output = Self>
    + Mul<F, Output = Self>
    + MulAssign<Self>
    + MulAssign<F>
    + Div<Self, Output = Self>
    + Div<F, Output = Self>
    + DivAssign<Self>
    + DivAssign<F>
    + Rem<Self, Output = Self>
    + Rem<F, Output = Self>
    + RemAssign<Self>
    + RemAssign<F>
    + Neg<Output = Self>
    + Index<usize, Output = F>
    + IndexMut<usize>
    + Sum<Self>
    + Product<Self>
    + std::fmt::Display
{
    const ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;
    const MIN: Self;
    const MAX: Self;
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    #[must_use]
    fn splat(value: F) -> Self;
    #[must_use]
    fn dot(self, rhs: Self) -> F;
    #[must_use]
    #[inline]
    fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }
    #[must_use]
    fn min(self, rhs: Self) -> Self;
    #[must_use]
    fn max(self, rhs: Self) -> Self;
    #[must_use]
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
    #[must_use]
    fn min_element(self) -> F;
    #[must_use]
    fn max_element(self) -> F;
    #[must_use]
    fn min_position(self) -> usize;
    #[must_use]
    fn max_position(self) -> usize;
    #[must_use]
    fn element_sum(self) -> F;
    #[must_use]
    fn element_product(self) -> F;
    #[must_use]
    fn abs(self) -> Self;
    #[must_use]
    fn signum(self) -> Self;
    #[must_use]
    fn copysign(self, rhs: Self) -> Self;
    #[must_use]
    fn is_finite(self) -> bool;
    #[must_use]
    fn is_nan(self) -> bool;
    #[must_use]
    #[inline]
    fn length(self) -> F {
        Float::sqrt(self.length_squared())
    }
    #[must_use]
    #[inline]
    fn length_squared(self) -> F {
        self.dot(self)
    }
    #[must_use]
    #[inline]
    fn length_recip(self) -> F {
        Float::recip(self.length())
    }
    #[must_use]
    #[inline]
    fn distance(self, rhs: Self) -> F {
        (self - rhs).length()
    }
    #[must_use]
    #[inline]
    fn distance_squared(self, rhs: Self) -> F {
        (self - rhs).length_squared()
    }
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;
    #[must_use]
    #[inline]
    fn normalize(self) -> Self {
        self * self.length_recip()
    }
    #[must_use]
    #[inline]
    fn try_normalize(self) -> Option<Self> {
        let recip = self.length_recip();
        if Float::is_finite(recip) {
            Some(self * recip)
        } else {
            None
        }
    }
    #[must_use]
    #[inline]
    fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }
    #[must_use]
    #[inline]
    fn normalize_or_zero(self) -> Self {
        self.normalize_or(Self::ZERO)
    }
    #[must_use]
    #[inline]
    fn normalize_and_length(self) -> (Self, F) {
        let len = self.length();
        (self * Float::recip(len), len)
    }
    #[must_use]
    fn is_normalized(self) -> bool;
    #[must_use]
    #[inline]
    fn project_onto(self, rhs: Self) -> Self {
        rhs * (self.dot(rhs) / rhs.length_squared())
    }
    #[must_use]
    #[inline]
    fn reject_from(self, rhs: Self) -> Self {
        self - self.project_onto(rhs)
    }
    #[must_use]
    #[inline]
    fn project_onto_normalized(self, rhs: Self) -> Self {
        rhs * self.dot(rhs)
    }
    #[must_use]
    #[inline]
    fn reject_from_normalized(self, rhs: Self) -> Self {
        self - self.project_onto_normalized(rhs)
    }
    #[must_use]
    fn round(self) -> Self;
    #[must_use]
    fn floor(self) -> Self;
    #[must_use]
    fn ceil(self) -> Self;
    #[must_use]
    fn trunc(self) -> Self;
    #[must_use]
    fn fract(self) -> Self;
    #[must_use]
    fn fract_gl(self) -> Self;
    #[must_use]
    fn step(self, rhs: Self) -> Self;
    #[must_use]
    #[inline]
    fn saturate(self) -> Self {
        self.clamp(Self::ZERO, Self::ONE)
    }
    #[must_use]
    fn exp(self) -> Self;
    #[must_use]
    fn exp2(self) -> Self;
    #[must_use]
    fn ln(self) -> Self;
    #[must_use]
    fn log2(self) -> Self;
    #[must_use]
    fn powf(self, n: F) -> Self;
    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn sin_cos(self) -> (Self, Self);
    #[must_use]
    fn recip(self) -> Self;
    #[must_use]
    #[inline]
    fn lerp(self, rhs: Self, s: F) -> Self {
        self + (rhs - self) * s
    }
    #[must_use]
    fn move_towards(self, rhs: Self, d: F) -> Self;
    #[must_use]
    fn midpoint(self, rhs: Self) -> Self;
    #[must_use]
    fn abs_diff_eq(self, rhs: Self, max_abs_diff: F) -> bool;
    #[must_use]
    fn clamp_length(self, min: F, max: F) -> Self;
    #[must_use]
    fn clamp_length_max(self, max: F) -> Self;
    #[must_use]
    fn clamp_length_min(self, min: F) -> Self;
    #[must_use]
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }
    #[must_use]
    fn reflect(self, normal: Self) -> Self;
    #[must_use]
    fn refract(self, normal: Self, eta: F) -> Self;
    #[must_use]
    fn from_slice(slice: &[F]) -> Self;
    fn write_to_slice(self, slice: &mut [F]);
}

pub trait TVec2<F: FloatScalar>:
    FloatVec<F>
    + ScalarCastable<f32, Casted = glam::Vec2>
    + ScalarCastable<f64, Casted = glam::DVec2>
    + ArrayLike<2, Element = F>
    + AsRef<[F; 2]>
    + AsMut<[F; 2]>
    + From<[F; 2]>
    + From<(F, F)>
    + Into<[F; 2]>
    + Into<(F, F)>
{
    type MaybeAligned: TVec2<F>;

    const X: Self;
    const Y: Self;
    const NEG_X: Self;
    const NEG_Y: Self;
    const AXES: [Self; 2];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> F {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> F {
        self[1]
    }

    #[must_use]
    fn new(x: F, y: F) -> Self {
        Self::from_array([x, y])
    }

    #[must_use]
    fn map<FN>(self, f: FN) -> Self
    where
        FN: Fn(F) -> F,
    {
        Self::new(f(self[0]), f(self[1]))
    }

    #[must_use]
    fn extend(self, z: F) -> F::Vec3 {
        F::Vec3::new(self[0], self[1], z)
    }

    #[must_use]
    fn with_x(self, x: F) -> Self;
    #[must_use]
    fn with_y(self, y: F) -> Self;
    #[must_use]
    fn to_angle(self) -> F;
    #[must_use]
    fn angle_to(self, rhs: Self) -> F;
    #[must_use]
    fn perp(self) -> Self;
    #[must_use]
    fn perp_dot(self, rhs: Self) -> F;
    #[must_use]
    fn rotate(self, rhs: Self) -> Self;
    #[must_use]
    fn rotate_towards(self, rhs: Self, max_angle: F) -> Self;
}

pub trait TVec3<F: FloatScalar>:
    FloatVec<F>
    + ScalarCastable<f32, Casted = glam::Vec3>
    + ScalarCastable<f64, Casted = glam::DVec3>
    + LossyAlignable<Aligned = glam::Vec3A>
    + ArrayLike<3, Element = F>
    + AsRef<[F; 3]>
    + AsMut<[F; 3]>
    + From<[F; 3]>
    + From<(F, F, F)>
    + Into<[F; 3]>
    + Into<(F, F, F)>
{
    type MaybeAligned: TVec3<F>;

    const X: Self;
    const Y: Self;
    const Z: Self;
    const NEG_X: Self;
    const NEG_Y: Self;
    const NEG_Z: Self;
    const AXES: [Self; 3];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> F {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> F {
        self[1]
    }
    #[must_use]
    #[inline]
    fn z(&self) -> F {
        self[2]
    }

    #[must_use]
    fn new(x: F, y: F, z: F) -> Self {
        Self::from_array([x, y, z])
    }

    #[must_use]
    fn map<FN>(self, f: FN) -> Self
    where
        FN: Fn(F) -> F,
    {
        Self::new(f(self[0]), f(self[1]), f(self[2]))
    }

    #[must_use]
    fn extend(self, w: F) -> F::Vec4 {
        F::Vec4::new(self[0], self[1], self[2], w)
    }

    #[must_use]
    fn truncate(self) -> F::Vec2 {
        F::Vec2::new(self[0], self[1])
    }

    #[must_use]
    fn with_x(self, x: F) -> Self;
    #[must_use]
    fn with_y(self, y: F) -> Self;
    #[must_use]
    fn with_z(self, z: F) -> Self;
    #[must_use]
    fn cross(self, rhs: Self) -> Self;
    #[must_use]
    fn angle_between(self, rhs: Self) -> F;
    #[must_use]
    fn any_orthogonal_vector(&self) -> Self;
    #[must_use]
    fn any_orthonormal_vector(&self) -> Self;
    #[must_use]
    fn any_orthonormal_pair(&self) -> (Self, Self);
    #[must_use]
    fn from_homogeneous(v: F::Vec4) -> Option<Self>;
    #[must_use]
    fn to_homogeneous(self) -> F::Vec4;
}

pub trait TVec4<F: FloatScalar>:
    FloatVec<F>
    + ScalarCastable<f32, Casted = glam::Vec4>
    + ScalarCastable<f64, Casted = glam::DVec4>
    + ArrayLike<4, Element = F>
    + AsRef<[F; 4]>
    + AsMut<[F; 4]>
    + From<[F; 4]>
    + From<(F, F, F, F)>
    + Into<[F; 4]>
    + Into<(F, F, F, F)>
{
    type MaybeAligned: TVec4<F>;

    const X: Self;
    const Y: Self;
    const Z: Self;
    const W: Self;
    const NEG_X: Self;
    const NEG_Y: Self;
    const NEG_Z: Self;
    const NEG_W: Self;
    const AXES: [Self; 4];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> F {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> F {
        self[1]
    }
    #[must_use]
    #[inline]
    fn z(&self) -> F {
        self[2]
    }
    #[must_use]
    #[inline]
    fn w(&self) -> F {
        self[3]
    }

    #[must_use]
    fn new(x: F, y: F, z: F, w: F) -> Self {
        Self::from_array([x, y, z, w])
    }

    #[must_use]
    fn map<FN>(self, f: FN) -> Self
    where
        FN: Fn(F) -> F,
    {
        Self::new(f(self[0]), f(self[1]), f(self[2]), f(self[3]))
    }

    #[must_use]
    fn truncate(self) -> F::Vec3 {
        F::Vec3::new(self[0], self[1], self[2])
    }

    #[must_use]
    fn with_x(self, x: F) -> Self;
    #[must_use]
    fn with_y(self, y: F) -> Self;
    #[must_use]
    fn with_z(self, z: F) -> Self;
    #[must_use]
    fn with_w(self, w: F) -> Self;
    #[must_use]
    fn project(self) -> F::Vec3;
}

pub trait TQuat<F: FloatScalar>:
    BaseType
    + ApproxFloat<F>
    + BytemuckShim
    + ZerocopyShim
    + RotationLike3d<F>
    + ScalarCastable<f32, Casted = glam::Quat>
    + ScalarCastable<f64, Casted = glam::DQuat>
    + ArrayLike<4, Element = F>
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + Mul<F, Output = Self>
    + MulAssign<Self>
    + MulAssign<F>
    + Div<F, Output = Self>
    + DivAssign<F>
    + Neg<Output = Self>
    + Sum<Self>
    + Product<Self>
    + AsRef<[F; 4]>
    + Into<[F; 4]>
    + Into<(F, F, F, F)>
    + std::fmt::Display
{
    type MaybeAligned: TQuat<F>;

    const IDENTITY: Self;
    const NAN: Self;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> F {
        <Self as AsRef<[F; 4]>>::as_ref(self)[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> F {
        <Self as AsRef<[F; 4]>>::as_ref(self)[1]
    }
    #[must_use]
    #[inline]
    fn z(&self) -> F {
        <Self as AsRef<[F; 4]>>::as_ref(self)[2]
    }
    #[must_use]
    #[inline]
    fn w(&self) -> F {
        <Self as AsRef<[F; 4]>>::as_ref(self)[3]
    }

    #[must_use]
    fn from_xyzw(x: F, y: F, z: F, w: F) -> Self {
        Self::from_array([x, y, z, w])
    }

    #[must_use]
    fn from_vec4(v: F::Vec4) -> Self;
    #[must_use]
    fn from_scaled_axis(v: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_axes(x_axis: F::Vec3, y_axis: F::Vec3, z_axis: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_arc(from: F::Vec3, to: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_arc_colinear(from: F::Vec3, to: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_arc_2d(from: F::Vec2, to: F::Vec2) -> Self;
    #[must_use]
    fn to_scaled_axis(self) -> F::Vec3;
    #[must_use]
    fn xyz(self) -> F::Vec3;
    #[must_use]
    fn conjugate(self) -> Self;
    #[must_use]
    fn inverse(self) -> Self;
    #[must_use]
    fn dot(self, rhs: Self) -> F;
    #[must_use]
    fn length(self) -> F;
    #[must_use]
    fn length_squared(self) -> F;
    #[must_use]
    fn length_recip(self) -> F;
    #[must_use]
    fn normalize(self) -> Self;
    #[must_use]
    fn is_finite(self) -> bool;
    #[must_use]
    fn is_nan(self) -> bool;
    #[must_use]
    fn is_normalized(self) -> bool;
    #[must_use]
    fn is_near_identity(self) -> bool;
    #[must_use]
    fn angle_between(self, rhs: Self) -> F;
    #[must_use]
    fn rotate_towards(self, rhs: Self, max_angle: F) -> Self;
    #[must_use]
    fn abs_diff_eq(self, rhs: Self, max_abs_diff: F) -> bool;
    #[must_use]
    fn lerp(self, end: Self, s: F) -> Self;
    #[must_use]
    fn slerp(self, end: Self, s: F) -> Self;
    #[must_use]
    fn mul_vec3(self, rhs: F::Vec3) -> F::Vec3;
    #[must_use]
    fn mul_quat(self, rhs: Self) -> Self;
    #[must_use]
    fn from_slice(slice: &[F]) -> Self;
    fn write_to_slice(self, slice: &mut [F]);
}

pub trait FloatMat<F: FloatScalar>:
    BaseType
    + ApproxFloat<F>
    + BytemuckShim
    + ZerocopyShim
    + ArrayLike2D<Element = F>
    + Default
    + PartialEq
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + Mul<F, Output = Self>
    + Mul<<Self as FloatMat<F>>::Col, Output = <Self as FloatMat<F>>::Col>
    + MulAssign<Self>
    + MulAssign<F>
    + Div<F, Output = Self>
    + DivAssign<F>
    + Neg<Output = Self>
    + Sum<Self>
    + Product<Self>
    + std::fmt::Display
{
    type Col: FloatVec<F>;

    const ZERO: Self;
    const IDENTITY: Self;
    const NAN: Self;

    #[must_use]
    fn from_diagonal(diagonal: Self::Col) -> Self;
    #[must_use]
    fn from_cols_slice(slice: &[F]) -> Self;
    fn write_cols_to_slice(&self, slice: &mut [F]);
    #[must_use]
    fn col(&self, index: usize) -> Self::Col;
    fn col_mut(&mut self, index: usize) -> &mut Self::Col;
    #[must_use]
    fn row(&self, index: usize) -> Self::Col;
    #[must_use]
    fn is_finite(&self) -> bool;
    #[must_use]
    fn is_nan(&self) -> bool;
    #[must_use]
    fn transpose(&self) -> Self;
    #[must_use]
    fn diagonal(&self) -> Self::Col;
    #[must_use]
    fn determinant(&self) -> F;
    #[must_use]
    fn inverse(&self) -> Self;
    #[must_use]
    fn try_inverse(&self) -> Option<Self>;
    #[must_use]
    #[inline]
    fn inverse_or_zero(&self) -> Self {
        self.try_inverse().unwrap_or(Self::ZERO)
    }
    #[must_use]
    #[inline]
    fn mul_vec(&self, rhs: Self::Col) -> Self::Col {
        *self * rhs
    }
    #[must_use]
    #[inline]
    fn mul_transpose_vec(&self, rhs: Self::Col) -> Self::Col {
        self.transpose().mul_vec(rhs)
    }
    #[must_use]
    #[inline]
    fn mul_mat(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
    #[must_use]
    #[inline]
    fn add_mat(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
    #[must_use]
    #[inline]
    fn sub_mat(&self, rhs: &Self) -> Self {
        *self - *rhs
    }
    #[must_use]
    #[inline]
    fn mul_scalar(&self, rhs: F) -> Self {
        *self * rhs
    }
    #[must_use]
    fn mul_diagonal_scale(&self, scale: Self::Col) -> Self;
    #[must_use]
    #[inline]
    fn div_scalar(&self, rhs: F) -> Self {
        *self / rhs
    }
    #[must_use]
    fn recip(&self) -> Self;
    #[must_use]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: F) -> bool;
    #[must_use]
    fn abs(&self) -> Self;
}

pub trait TMat2<F: FloatScalar>:
    FloatMat<F, Col = F::Vec2>
    + ScalarCastable<f32, Casted = glam::Mat2>
    + ScalarCastable<f64, Casted = glam::DMat2>
    + ArrayLike<4, Element = F>
    + Mul<F::Vec2, Output = F::Vec2>
    + AsRef<[F; 4]>
    + AsMut<[F; 4]>
{
    type MaybeAligned: TMat2<F>;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x_axis(&self) -> F::Vec2 {
        self.col(0)
    }
    #[must_use]
    #[inline]
    fn y_axis(&self) -> F::Vec2 {
        self.col(1)
    }

    #[must_use]
    fn from_cols(x_axis: F::Vec2, y_axis: F::Vec2) -> Self;
    #[must_use]
    fn from_cols_array(m: &[F; 4]) -> Self;
    #[must_use]
    fn to_cols_array(&self) -> [F; 4];
    #[must_use]
    fn from_cols_array_2d(m: &[[F; 2]; 2]) -> Self;
    #[must_use]
    fn to_cols_array_2d(&self) -> [[F; 2]; 2];
    #[must_use]
    fn from_scale_angle(scale: F::Vec2, angle: F) -> Self;
    #[must_use]
    fn from_mat3(m: F::Mat3) -> Self;
    #[must_use]
    fn from_mat3_minor(m: F::Mat3, i: usize, j: usize) -> Self;
}

pub trait TMat3<F: FloatScalar>:
    FloatMat<F, Col: TVec3<F>>
    + RotationLike3d<F>
    + LossyAlignable<Aligned = glam::Mat3A>
    + ScalarCastable<f32, Casted = glam::Mat3>
    + ScalarCastable<f64, Casted = glam::DMat3>
    + ArrayLike<9, Element = F>
    + Mul<F::Vec3, Output = F::Vec3>
{
    type MaybeAligned: TMat3<F>;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x_axis(&self) -> <Self as FloatMat<F>>::Col {
        self.col(0)
    }
    #[must_use]
    #[inline]
    fn y_axis(&self) -> <Self as FloatMat<F>>::Col {
        self.col(1)
    }
    #[must_use]
    #[inline]
    fn z_axis(&self) -> <Self as FloatMat<F>>::Col {
        self.col(2)
    }

    #[must_use]
    fn from_cols(
        x_axis: <Self as FloatMat<F>>::Col,
        y_axis: <Self as FloatMat<F>>::Col,
        z_axis: <Self as FloatMat<F>>::Col,
    ) -> Self;
    #[must_use]
    fn from_cols_array(m: &[F; 9]) -> Self;
    #[must_use]
    fn to_cols_array(&self) -> [F; 9];
    #[must_use]
    fn from_cols_array_2d(m: &[[F; 3]; 3]) -> Self;
    #[must_use]
    fn to_cols_array_2d(&self) -> [[F; 3]; 3];
    #[must_use]
    fn from_mat4(m: F::Mat4) -> Self;
    #[must_use]
    fn from_mat4_minor(m: F::Mat4, i: usize, j: usize) -> Self;
    #[must_use]
    fn from_translation(translation: F::Vec2) -> Self;
    #[must_use]
    fn from_scale_angle_translation(scale: F::Vec2, angle: F, translation: F::Vec2) -> Self;
    #[must_use]
    fn from_scale(scale: F::Vec2) -> Self;
    #[must_use]
    fn from_mat2(m: F::Mat2) -> Self;
    #[must_use]
    fn transform_point2(&self, rhs: F::Vec2) -> F::Vec2;
    #[must_use]
    fn transform_vector2(&self, rhs: F::Vec2) -> F::Vec2;
}

pub trait TMat4<F: FloatScalar>:
    FloatMat<F, Col = F::Vec4>
    + TransRotLike3d<F>
    + ScalarCastable<f32, Casted = glam::Mat4>
    + ScalarCastable<f64, Casted = glam::DMat4>
    + ArrayLike<16, Element = F>
    + Mul<F::Vec4, Output = F::Vec4>
    + AsRef<[F; 16]>
    + AsMut<[F; 16]>
{
    type MaybeAligned: TMat4<F>;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x_axis(&self) -> F::Vec4 {
        self.col(0)
    }
    #[must_use]
    #[inline]
    fn y_axis(&self) -> F::Vec4 {
        self.col(1)
    }
    #[must_use]
    #[inline]
    fn z_axis(&self) -> F::Vec4 {
        self.col(2)
    }
    #[must_use]
    #[inline]
    fn w_axis(&self) -> F::Vec4 {
        self.col(3)
    }

    #[must_use]
    fn from_cols(x_axis: F::Vec4, y_axis: F::Vec4, z_axis: F::Vec4, w_axis: F::Vec4) -> Self;
    #[must_use]
    fn from_cols_array(m: &[F; 16]) -> Self;
    #[must_use]
    fn to_cols_array(&self) -> [F; 16];
    #[must_use]
    fn from_cols_array_2d(m: &[[F; 4]; 4]) -> Self;
    #[must_use]
    fn to_cols_array_2d(&self) -> [[F; 4]; 4];
    #[must_use]
    fn from_scale_rotation_translation(scale: F::Vec3, rotation: F::Quat, translation: F::Vec3) -> Self;
    #[must_use]
    fn from_rotation_translation(rotation: F::Quat, translation: F::Vec3) -> Self;
    #[must_use]
    fn to_scale_rotation_translation(&self) -> (F::Vec3, F::Quat, F::Vec3);
    #[must_use]
    fn from_quat(rotation: F::Quat) -> Self;
    #[must_use]
    fn from_mat3(m: F::Mat3) -> Self;
    #[must_use]
    fn from_mat3_translation(mat3: F::Mat3, translation: F::Vec3) -> Self;
    #[must_use]
    fn from_translation(translation: F::Vec3) -> Self;
    #[must_use]
    fn from_axis_angle(axis: F::Vec3, angle: F) -> Self;
    #[must_use]
    fn from_euler(order: glam::EulerRot, a: F, b: F, c: F) -> Self;
    #[must_use]
    fn to_euler(&self, order: glam::EulerRot) -> (F, F, F);
    #[must_use]
    fn from_scale(scale: F::Vec3) -> Self;
    #[must_use]
    fn frustum_lh(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn frustum_rh(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn frustum_rh_gl(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn perspective_lh(fov_y_radians: F, aspect_ratio: F, z_near: F, z_far: F) -> Self;
    #[must_use]
    fn perspective_rh(fov_y_radians: F, aspect_ratio: F, z_near: F, z_far: F) -> Self;
    #[must_use]
    fn perspective_rh_gl(fov_y_radians: F, aspect_ratio: F, z_near: F, z_far: F) -> Self;
    #[must_use]
    fn perspective_infinite_lh(fov_y_radians: F, aspect_ratio: F, z_near: F) -> Self;
    #[must_use]
    fn perspective_infinite_reverse_lh(fov_y_radians: F, aspect_ratio: F, z_near: F) -> Self;
    #[must_use]
    fn perspective_infinite_rh(fov_y_radians: F, aspect_ratio: F, z_near: F) -> Self;
    #[must_use]
    fn perspective_infinite_reverse_rh(fov_y_radians: F, aspect_ratio: F, z_near: F) -> Self;
    #[must_use]
    fn orthographic_lh(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn orthographic_rh(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn orthographic_rh_gl(left: F, right: F, bottom: F, top: F, near: F, far: F) -> Self;
    #[must_use]
    fn project_point3(&self, rhs: F::Vec3) -> F::Vec3;
    #[must_use]
    fn transform_point3(&self, rhs: F::Vec3) -> F::Vec3;
    #[must_use]
    fn transform_vector3(&self, rhs: F::Vec3) -> F::Vec3;
}

pub trait FloatAffine<F: FloatScalar>:
    BaseType
    + ApproxFloat<F>
    + Default
    + PartialEq
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + std::fmt::Display
{
    type Vec: FloatVec<F>;
    type Mat: FloatMat<F, Col = Self::Vec>;

    const ZERO: Self;
    const IDENTITY: Self;
    const NAN: Self;

    #[must_use]
    fn from_scale(scale: Self::Vec) -> Self;
    #[must_use]
    fn from_translation(translation: Self::Vec) -> Self;
    #[must_use]
    fn from_cols_slice(slice: &[F]) -> Self;
    fn write_cols_to_slice(&self, slice: &mut [F]);
    #[must_use]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec;
    #[must_use]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec;
    #[must_use]
    fn is_finite(&self) -> bool;
    #[must_use]
    fn is_nan(&self) -> bool;
    #[must_use]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: F) -> bool;
    #[must_use]
    fn inverse(&self) -> Self;
}

pub trait TAffine2<F: FloatScalar>:
    FloatAffine<F, Vec = F::Vec2, Mat = F::Mat2>
    + ScalarCastable<f32, Casted = glam::Affine2>
    + ScalarCastable<f64, Casted = glam::DAffine2>
    + Mul<F::Mat3, Output = F::Mat3>
{
    type MaybeAligned: TAffine2<F>;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    fn matrix2(&self) -> F::Mat2;
    #[must_use]
    fn translation(&self) -> F::Vec2;

    #[must_use]
    fn from_cols(x_axis: F::Vec2, y_axis: F::Vec2, z_axis: F::Vec2) -> Self;
    #[must_use]
    fn from_mat2_translation(matrix2: F::Mat2, translation: F::Vec2) -> Self;
    #[must_use]
    fn from_cols_array(m: &[F; 6]) -> Self;
    #[must_use]
    fn to_cols_array(&self) -> [F; 6];
    #[must_use]
    fn from_cols_array_2d(m: &[[F; 2]; 3]) -> Self;
    #[must_use]
    fn to_cols_array_2d(&self) -> [[F; 2]; 3];
    #[must_use]
    fn from_mat2(matrix2: F::Mat2) -> Self;
    #[must_use]
    fn from_scale_angle_translation(scale: F::Vec2, angle: F, translation: F::Vec2) -> Self;
    #[must_use]
    fn from_angle_translation(angle: F, translation: F::Vec2) -> Self;
    #[must_use]
    fn from_mat3(m: F::Mat3) -> Self;
    #[must_use]
    fn to_scale_angle_translation(&self) -> (F::Vec2, F, F::Vec2);
}

pub trait TAffine3<F: FloatScalar>:
    FloatAffine<F, Vec: TVec3<F>, Mat: TMat3<F>>
    + TransRotLike3d<F>
    + LossyAlignable<Aligned = glam::Affine3A>
    + ScalarCastable<f32, Casted = glam::Affine3>
    + ScalarCastable<f64, Casted = glam::DAffine3>
    + Mul<F::Mat4, Output = F::Mat4>
{
    type MaybeAligned: TAffine3<F>;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    fn matrix3(&self) -> <Self as FloatAffine<F>>::Mat;
    #[must_use]
    fn translation(&self) -> <Self as FloatAffine<F>>::Vec;

    #[must_use]
    fn from_cols(
        x_axis: <Self as FloatAffine<F>>::Vec,
        y_axis: <Self as FloatAffine<F>>::Vec,
        z_axis: <Self as FloatAffine<F>>::Vec,
        w_axis: <Self as FloatAffine<F>>::Vec,
    ) -> Self;
    #[must_use]
    fn from_mat3_translation(
        matrix3: <Self as FloatAffine<F>>::Mat,
        translation: <Self as FloatAffine<F>>::Vec,
    ) -> Self;
    #[must_use]
    fn from_cols_array(m: &[F; 12]) -> Self;
    #[must_use]
    fn to_cols_array(&self) -> [F; 12];
    #[must_use]
    fn from_cols_array_2d(m: &[[F; 3]; 4]) -> Self;
    #[must_use]
    fn to_cols_array_2d(&self) -> [[F; 3]; 4];
    #[must_use]
    fn from_quat(rotation: F::Quat) -> Self;
    #[must_use]
    fn from_axis_angle(axis: <Self as FloatAffine<F>>::Vec, angle: F) -> Self;
    #[must_use]
    fn from_mat3(mat3: <Self as FloatAffine<F>>::Mat) -> Self;
    #[must_use]
    fn from_scale_rotation_translation(
        scale: <Self as FloatAffine<F>>::Vec,
        rotation: F::Quat,
        translation: <Self as FloatAffine<F>>::Vec,
    ) -> Self;
    #[must_use]
    fn from_rotation_translation(
        rotation: F::Quat,
        translation: <Self as FloatAffine<F>>::Vec,
    ) -> Self;
    #[must_use]
    fn from_mat4(m: F::Mat4) -> Self;
    #[must_use]
    fn to_scale_rotation_translation(
        &self,
    ) -> (<Self as FloatAffine<F>>::Vec, F::Quat, <Self as FloatAffine<F>>::Vec);
}

// ============================================================================
// Bool vectors
// ============================================================================

pub trait BoolVec:
    BaseType
    + Default
    + PartialEq
    + Eq
    + Hash
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + Not<Output = Self>
    + std::fmt::Display
{
    const FALSE: Self;
    const TRUE: Self;

    #[must_use]
    fn splat(value: bool) -> Self;
    #[must_use]
    fn bitmask(self) -> u32;
    #[must_use]
    fn any(self) -> bool;
    #[must_use]
    fn all(self) -> bool;
    #[must_use]
    fn test(self, index: usize) -> bool;
    fn set(&mut self, index: usize, value: bool);
}

pub trait TBVec2:
    BoolVec
    + ArrayLike<2, Element = bool>
    + From<[bool; 2]>
    + Into<[bool; 2]>
{
    type MaybeAligned: TBVec2;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> bool {
        self.test(0)
    }
    #[must_use]
    #[inline]
    fn y(&self) -> bool {
        self.test(1)
    }

    #[must_use]
    fn new(x: bool, y: bool) -> Self {
        Self::from_array([x, y])
    }
}

pub trait TBVec3:
    BoolVec
    + Alignable<Aligned = glam::BVec3A>
    + ArrayLike<3, Element = bool>
    + From<[bool; 3]>
    + Into<[bool; 3]>
{
    type MaybeAligned: TBVec3;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> bool {
        self.test(0)
    }
    #[must_use]
    #[inline]
    fn y(&self) -> bool {
        self.test(1)
    }
    #[must_use]
    #[inline]
    fn z(&self) -> bool {
        self.test(2)
    }

    #[must_use]
    fn new(x: bool, y: bool, z: bool) -> Self {
        Self::from_array([x, y, z])
    }
}

pub trait TBVec4:
    BoolVec
    + Alignable<Aligned = glam::BVec4A>
    + ArrayLike<4, Element = bool>
    + From<[bool; 4]>
    + Into<[bool; 4]>
{
    type MaybeAligned: TBVec4;

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> bool {
        self.test(0)
    }
    #[must_use]
    #[inline]
    fn y(&self) -> bool {
        self.test(1)
    }
    #[must_use]
    #[inline]
    fn z(&self) -> bool {
        self.test(2)
    }
    #[must_use]
    #[inline]
    fn w(&self) -> bool {
        self.test(3)
    }

    #[must_use]
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self::from_array([x, y, z, w])
    }
}

pub trait IntVec<I: IntScalar>:
    BaseType
    + BytemuckShim
    + ZerocopyShim
    + Default
    + PartialEq
    + Eq
    + Hash
    + Add<Self, Output = Self>
    + Add<I, Output = Self>
    + AddAssign<Self>
    + AddAssign<I>
    + Sub<Self, Output = Self>
    + Sub<I, Output = Self>
    + SubAssign<Self>
    + SubAssign<I>
    + Mul<Self, Output = Self>
    + Mul<I, Output = Self>
    + MulAssign<Self>
    + MulAssign<I>
    + Div<Self, Output = Self>
    + Div<I, Output = Self>
    + DivAssign<Self>
    + DivAssign<I>
    + Rem<Self, Output = Self>
    + Rem<I, Output = Self>
    + RemAssign<Self>
    + RemAssign<I>
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + Index<usize, Output = I>
    + IndexMut<usize>
    + Sum<Self>
    + Product<Self>
    + std::fmt::Display
{
    type Mask: BoolVec;

    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;

    #[must_use]
    fn splat(value: I) -> Self;
    #[must_use]
    fn dot(self, rhs: Self) -> I;
    #[must_use]
    #[inline]
    fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }
    #[must_use]
    fn min(self, rhs: Self) -> Self;
    #[must_use]
    fn max(self, rhs: Self) -> Self;
    #[must_use]
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
    #[must_use]
    fn min_element(self) -> I;
    #[must_use]
    fn max_element(self) -> I;
    #[must_use]
    fn min_position(self) -> usize;
    #[must_use]
    fn max_position(self) -> usize;
    #[must_use]
    fn element_sum(self) -> I;
    #[must_use]
    fn element_product(self) -> I;
    #[must_use]
    #[inline]
    fn length_squared(self) -> I {
        self.dot(self)
    }
    #[must_use]
    #[inline]
    fn distance_squared(self, rhs: Self) -> I {
        (self - rhs).length_squared()
    }
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;
    #[must_use]
    fn cmpeq(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn cmpne(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn cmpge(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn cmpgt(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn cmple(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn cmplt(self, rhs: Self) -> Self::Mask;
    #[must_use]
    fn checked_add(self, rhs: Self) -> Option<Self>;
    #[must_use]
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    #[must_use]
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    #[must_use]
    fn checked_div(self, rhs: Self) -> Option<Self>;
    #[must_use]
    fn wrapping_add(self, rhs: Self) -> Self;
    #[must_use]
    fn wrapping_sub(self, rhs: Self) -> Self;
    #[must_use]
    fn wrapping_mul(self, rhs: Self) -> Self;
    #[must_use]
    fn wrapping_div(self, rhs: Self) -> Self;
    #[must_use]
    fn saturating_add(self, rhs: Self) -> Self;
    #[must_use]
    fn saturating_sub(self, rhs: Self) -> Self;
    #[must_use]
    fn saturating_mul(self, rhs: Self) -> Self;
    #[must_use]
    fn saturating_div(self, rhs: Self) -> Self;
    #[must_use]
    fn from_slice(slice: &[I]) -> Self;
    fn write_to_slice(self, slice: &mut [I]);
}

pub trait TIVec2<I: IntScalar>:
    IntVec<I, Mask: TBVec2>
    + ArrayLike<2, Element = I>
    + AsRef<[I; 2]>
    + AsMut<[I; 2]>
    + From<[I; 2]>
    + From<(I, I)>
    + Into<[I; 2]>
    + Into<(I, I)>
{
    type MaybeAligned: TIVec2<I>;

    const X: Self;
    const Y: Self;
    const AXES: [Self; 2];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> I {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> I {
        self[1]
    }

    #[must_use]
    fn new(x: I, y: I) -> Self {
        Self::from_array([x, y])
    }

    #[must_use]
    fn with_x(self, x: I) -> Self;
    #[must_use]
    fn with_y(self, y: I) -> Self;
    #[must_use]
    fn extend(self, z: I) -> I::Vec3;
    #[must_use]
    fn perp(self) -> Self;
    #[must_use]
    fn perp_dot(self, rhs: Self) -> I;
    #[must_use]
    fn rotate(self, rhs: Self) -> Self;
}

pub trait TIVec3<I: IntScalar>:
    IntVec<I, Mask: TBVec3>
    + ArrayLike<3, Element = I>
    + AsRef<[I; 3]>
    + AsMut<[I; 3]>
    + From<[I; 3]>
    + From<(I, I, I)>
    + Into<[I; 3]>
    + Into<(I, I, I)>
{
    type MaybeAligned: TIVec3<I>;

    const X: Self;
    const Y: Self;
    const Z: Self;
    const AXES: [Self; 3];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> I {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> I {
        self[1]
    }
    #[must_use]
    #[inline]
    fn z(&self) -> I {
        self[2]
    }

    #[must_use]
    fn new(x: I, y: I, z: I) -> Self {
        Self::from_array([x, y, z])
    }

    #[must_use]
    fn with_x(self, x: I) -> Self;
    #[must_use]
    fn with_y(self, y: I) -> Self;
    #[must_use]
    fn with_z(self, z: I) -> Self;
    #[must_use]
    fn extend(self, w: I) -> I::Vec4;
    #[must_use]
    fn truncate(self) -> I::Vec2;
    #[must_use]
    fn cross(self, rhs: Self) -> Self;
}

pub trait TIVec4<I: IntScalar>:
    IntVec<I, Mask: TBVec4>
    + ArrayLike<4, Element = I>
    + AsRef<[I; 4]>
    + AsMut<[I; 4]>
    + From<[I; 4]>
    + From<(I, I, I, I)>
    + Into<[I; 4]>
    + Into<(I, I, I, I)>
{
    type MaybeAligned: TIVec4<I>;

    const X: Self;
    const Y: Self;
    const Z: Self;
    const W: Self;
    const AXES: [Self; 4];

    #[must_use]
    fn maybe_align(&self) -> Self::MaybeAligned;

    #[must_use]
    #[inline]
    fn x(&self) -> I {
        self[0]
    }
    #[must_use]
    #[inline]
    fn y(&self) -> I {
        self[1]
    }
    #[must_use]
    #[inline]
    fn z(&self) -> I {
        self[2]
    }
    #[must_use]
    #[inline]
    fn w(&self) -> I {
        self[3]
    }

    #[must_use]
    fn new(x: I, y: I, z: I, w: I) -> Self {
        Self::from_array([x, y, z, w])
    }

    #[must_use]
    fn with_x(self, x: I) -> Self;
    #[must_use]
    fn with_y(self, y: I) -> Self;
    #[must_use]
    fn with_z(self, z: I) -> Self;
    #[must_use]
    fn with_w(self, w: I) -> Self;
    #[must_use]
    fn truncate(self) -> I::Vec3;
}