use crate::{
    Alignable, ArrayLike, ArrayLike2D, BaseType, LossyAlignable, ScalarCastable, Unalignable,
};

macro_rules! base_type {
    ($($t:ty),* $(,)?) => {
        $(impl BaseType for $t {})*
    };
}

base_type! {
    glam::Vec2, glam::Vec3, glam::Vec3A, glam::Vec4,
    glam::DVec2, glam::DVec3, glam::DVec4,
    glam::Mat2, glam::Mat3, glam::Mat3A, glam::Mat4,
    glam::DMat2, glam::DMat3, glam::DMat4,
    glam::Quat, glam::DQuat,
    glam::Affine2, glam::Affine3, glam::Affine3A,
    glam::DAffine2, glam::DAffine3,
    glam::BVec2, glam::BVec3, glam::BVec3A, glam::BVec4, glam::BVec4A,
    glam::IVec2, glam::IVec3, glam::IVec4,
    glam::UVec2, glam::UVec3, glam::UVec4,
    glam::I64Vec2, glam::I64Vec3, glam::I64Vec4,
    glam::U64Vec2, glam::U64Vec3, glam::U64Vec4,
}

macro_rules! aligned_pair {
    ($unaligned:ty, $aligned:ty) => {
        impl Alignable for $unaligned {
            type Aligned = $aligned;
            #[inline]
            fn exact_align(&self) -> Self::Aligned {
                <$aligned>::from(*self)
            }
        }
        impl Unalignable for $aligned {
            type Unaligned = $unaligned;
            #[inline]
            fn unalign(&self) -> Self::Unaligned {
                <$unaligned>::from(*self)
            }
        }
    };
}

aligned_pair!(glam::Vec3, glam::Vec3A);
aligned_pair!(glam::Mat3, glam::Mat3A);
aligned_pair!(glam::Affine3, glam::Affine3A);

// BVec3 / BVec3A and BVec4 / BVec4A use array round-trip rather than direct From.
impl Alignable for glam::BVec3 {
    type Aligned = glam::BVec3A;
    #[inline]
    fn exact_align(&self) -> Self::Aligned {
        glam::BVec3A::from(self.to_array())
    }
}
impl Unalignable for glam::BVec3A {
    type Unaligned = glam::BVec3;
    #[inline]
    fn unalign(&self) -> Self::Unaligned {
        let mask = self.bitmask();
        glam::BVec3::new(mask & 1 != 0, mask & 2 != 0, mask & 4 != 0)
    }
}
impl Alignable for glam::BVec4 {
    type Aligned = glam::BVec4A;
    #[inline]
    fn exact_align(&self) -> Self::Aligned {
        glam::BVec4A::from(self.to_array())
    }
}
impl Unalignable for glam::BVec4A {
    type Unaligned = glam::BVec4;
    #[inline]
    fn unalign(&self) -> Self::Unaligned {
        let mask = self.bitmask();
        glam::BVec4::new(mask & 1 != 0, mask & 2 != 0, mask & 4 != 0, mask & 8 != 0)
    }
}

// Lossy alignment for f64 -> f32 SIMD-aligned types.
impl LossyAlignable for glam::DVec3 {
    type Aligned = glam::Vec3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        glam::Vec3A::from(self.as_vec3())
    }
}
impl LossyAlignable for glam::DMat3 {
    type Aligned = glam::Mat3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        glam::Mat3A::from(self.as_mat3())
    }
}
impl LossyAlignable for glam::DAffine3 {
    type Aligned = glam::Affine3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        glam::Affine3A::from(self.as_affine3())
    }
}

// Already-aligned f32 SIMD types align to themselves.
impl LossyAlignable for glam::Vec3A {
    type Aligned = glam::Vec3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        *self
    }
}
impl LossyAlignable for glam::Mat3A {
    type Aligned = glam::Mat3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        *self
    }
}
impl LossyAlignable for glam::Affine3A {
    type Aligned = glam::Affine3A;
    #[inline]
    fn align(&self) -> Self::Aligned {
        *self
    }
}

macro_rules! array_like {
    ($t:ty, $elem:ty, $n:literal) => {
        impl ArrayLike<$n> for $t {
            type Element = $elem;
            #[inline]
            fn from_array(arr: [$elem; $n]) -> Self {
                <$t>::from_array(arr)
            }
            #[inline]
            fn to_array(&self) -> [$elem; $n] {
                *<$t as AsRef<[$elem; $n]>>::as_ref(self)
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn to_ndarray(&self) -> ndarray::Array1<$elem> {
                ndarray::Array1::from_vec(<Self as ArrayLike<$n>>::to_array(self).to_vec())
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn from_ndarray(arr: ndarray::Array1<$elem>) -> Self {
                let slice = arr.as_slice().expect("contiguous ndarray");
                let mut out = [<$elem as Default>::default(); $n];
                out.copy_from_slice(&slice[..$n]);
                <$t>::from_array(out)
            }
        }
    };
}

array_like!(glam::Vec2, f32, 2);
array_like!(glam::Vec3, f32, 3);
array_like!(glam::Vec3A, f32, 3);
array_like!(glam::Vec4, f32, 4);
array_like!(glam::DVec2, f64, 2);
array_like!(glam::DVec3, f64, 3);
array_like!(glam::DVec4, f64, 4);
array_like!(glam::IVec2, i32, 2);
array_like!(glam::IVec3, i32, 3);
array_like!(glam::IVec4, i32, 4);
array_like!(glam::UVec2, u32, 2);
array_like!(glam::UVec3, u32, 3);
array_like!(glam::UVec4, u32, 4);
array_like!(glam::I64Vec2, i64, 2);
array_like!(glam::I64Vec3, i64, 3);
array_like!(glam::I64Vec4, i64, 4);
array_like!(glam::U64Vec2, u64, 2);
array_like!(glam::U64Vec3, u64, 3);
array_like!(glam::U64Vec4, u64, 4);
macro_rules! bvec_array_like {
    ($t:ty, $n:literal) => {
        impl ArrayLike<$n> for $t {
            type Element = bool;
            #[inline]
            fn from_array(arr: [bool; $n]) -> Self {
                <$t>::from_array(arr)
            }
            #[inline]
            fn to_array(&self) -> [bool; $n] {
                <[bool; $n]>::from(*self)
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn to_ndarray(&self) -> ndarray::Array1<bool> {
                ndarray::Array1::from_vec(<Self as ArrayLike<$n>>::to_array(self).to_vec())
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn from_ndarray(arr: ndarray::Array1<bool>) -> Self {
                let slice = arr.as_slice().expect("contiguous ndarray");
                let mut out = [false; $n];
                out.copy_from_slice(&slice[..$n]);
                <$t>::from_array(out)
            }
        }
    };
}

bvec_array_like!(glam::BVec2, 2);
bvec_array_like!(glam::BVec3, 3);
bvec_array_like!(glam::BVec4, 4);

// Quat: 4 floats, but glam uses from_array / to_array.
impl ArrayLike<4> for glam::Quat {
    type Element = f32;
    #[inline]
    fn from_array(arr: [f32; 4]) -> Self {
        glam::Quat::from_array(arr)
    }
    #[inline]
    fn to_array(&self) -> [f32; 4] {
        *<glam::Quat as AsRef<[f32; 4]>>::as_ref(self)
    }
    #[cfg(feature = "ndarray")]
    #[inline]
    fn to_ndarray(&self) -> ndarray::Array1<f32> {
        ndarray::Array1::from_vec(<Self as ArrayLike<4>>::to_array(self).to_vec())
    }
    #[cfg(feature = "ndarray")]
    #[inline]
    fn from_ndarray(arr: ndarray::Array1<f32>) -> Self {
        let s = arr.as_slice().expect("contiguous ndarray");
        glam::Quat::from_slice(s)
    }
}

impl ArrayLike<4> for glam::DQuat {
    type Element = f64;
    #[inline]
    fn from_array(arr: [f64; 4]) -> Self {
        glam::DQuat::from_array(arr)
    }
    #[inline]
    fn to_array(&self) -> [f64; 4] {
        *<glam::DQuat as AsRef<[f64; 4]>>::as_ref(self)
    }
    #[cfg(feature = "ndarray")]
    #[inline]
    fn to_ndarray(&self) -> ndarray::Array1<f64> {
        ndarray::Array1::from_vec(<Self as ArrayLike<4>>::to_array(self).to_vec())
    }
    #[cfg(feature = "ndarray")]
    #[inline]
    fn from_ndarray(arr: ndarray::Array1<f64>) -> Self {
        let s = arr.as_slice().expect("contiguous ndarray");
        glam::DQuat::from_slice(s)
    }
}

macro_rules! mat_array_like {
    ($t:ty, $elem:ty, $n:literal, $rows:literal, $cols:literal) => {
        impl ArrayLike<$n> for $t {
            type Element = $elem;
            #[inline]
            fn from_array(arr: [$elem; $n]) -> Self {
                <$t>::from_cols_array(&arr)
            }
            #[inline]
            fn to_array(&self) -> [$elem; $n] {
                self.to_cols_array()
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn to_ndarray(&self) -> ndarray::Array1<$elem> {
                ndarray::Array1::from_vec(self.to_cols_array().to_vec())
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn from_ndarray(arr: ndarray::Array1<$elem>) -> Self {
                let s = arr.as_slice().expect("contiguous ndarray");
                <$t>::from_cols_slice(s)
            }
        }
        impl ArrayLike2D for $t {
            type Element = $elem;
            const ROWS: usize = $rows;
            const COLS: usize = $cols;
            #[cfg(feature = "ndarray")]
            #[inline]
            fn from_ndarray2d(arr: ndarray::Array2<$elem>) -> Self {
                let mut flat = Vec::with_capacity($rows * $cols);
                for c in 0..$cols {
                    for r in 0..$rows {
                        flat.push(arr[(r, c)]);
                    }
                }
                <$t>::from_cols_slice(&flat)
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn to_ndarray2d(&self) -> ndarray::Array2<$elem> {
                let cols = self.to_cols_array();
                ndarray::Array2::from_shape_fn((Self::ROWS, Self::COLS), |(r, c)| cols[c * Self::ROWS + r])
            }
        }
    };
}

mat_array_like!(glam::Mat2, f32, 4, 2, 2);
mat_array_like!(glam::Mat3, f32, 9, 3, 3);
mat_array_like!(glam::Mat3A, f32, 9, 3, 3);
mat_array_like!(glam::Mat4, f32, 16, 4, 4);
mat_array_like!(glam::DMat2, f64, 4, 2, 2);
mat_array_like!(glam::DMat3, f64, 9, 3, 3);
mat_array_like!(glam::DMat4, f64, 16, 4, 4);

macro_rules! affine_array_like {
    ($t:ty, $elem:ty, $n:literal, $rows:literal, $cols:literal) => {
        impl ArrayLike2D for $t {
            type Element = $elem;
            const ROWS: usize = $rows;
            const COLS: usize = $cols;
            #[cfg(feature = "ndarray")]
            #[inline]
            fn from_ndarray2d(arr: ndarray::Array2<$elem>) -> Self {
                let mut flat = Vec::with_capacity($rows * $cols);
                for c in 0..$cols {
                    for r in 0..$rows {
                        flat.push(arr[(r, c)]);
                    }
                }
                <$t>::from_cols_slice(&flat)
            }
            #[cfg(feature = "ndarray")]
            #[inline]
            fn to_ndarray2d(&self) -> ndarray::Array2<$elem> {
                let mut out = ndarray::Array2::<$elem>::zeros((Self::ROWS, Self::COLS));
                let mut buf = [<$elem>::default(); $n];
                self.write_cols_to_slice(&mut buf);
                for c in 0..Self::COLS {
                    for r in 0..Self::ROWS {
                        out[(r, c)] = buf[c * Self::ROWS + r];
                    }
                }
                out
            }
        }
    };
}

affine_array_like!(glam::Affine2, f32, 6, 2, 3);
affine_array_like!(glam::Affine3, f32, 12, 3, 4);
affine_array_like!(glam::Affine3A, f32, 12, 3, 4);
affine_array_like!(glam::DAffine2, f64, 6, 2, 3);
affine_array_like!(glam::DAffine3, f64, 12, 3, 4);

// ScalarCastable: float-to-float casts.
macro_rules! scalar_castable_self {
    ($t:ty, $f:ty) => {
        impl ScalarCastable<$f> for $t {
            const LOSSY_CAST: bool = false;
            type Casted = $t;
            #[inline]
            fn cast(&self) -> Self::Casted {
                *self
            }
        }
    };
}

macro_rules! scalar_castable_via {
    ($from:ty, $to:ty, $f:ty, $method:ident, $lossy:expr) => {
        impl ScalarCastable<$f> for $from {
            const LOSSY_CAST: bool = $lossy;
            type Casted = $to;
            #[inline]
            fn cast(&self) -> Self::Casted {
                self.$method()
            }
        }
    };
}

scalar_castable_self!(glam::Vec2, f32);
scalar_castable_self!(glam::Vec3, f32);
scalar_castable_self!(glam::Vec4, f32);
scalar_castable_self!(glam::Quat, f32);
scalar_castable_self!(glam::Mat2, f32);
scalar_castable_self!(glam::Mat3, f32);
scalar_castable_self!(glam::Mat4, f32);
scalar_castable_self!(glam::Affine2, f32);
scalar_castable_self!(glam::Affine3, f32);

scalar_castable_self!(glam::DVec2, f64);
scalar_castable_self!(glam::DVec3, f64);
scalar_castable_self!(glam::DVec4, f64);
scalar_castable_self!(glam::DQuat, f64);
scalar_castable_self!(glam::DMat2, f64);
scalar_castable_self!(glam::DMat3, f64);
scalar_castable_self!(glam::DMat4, f64);
scalar_castable_self!(glam::DAffine2, f64);
scalar_castable_self!(glam::DAffine3, f64);

scalar_castable_via!(glam::Vec2, glam::DVec2, f64, as_dvec2, false);
scalar_castable_via!(glam::Vec3, glam::DVec3, f64, as_dvec3, false);
scalar_castable_via!(glam::Vec4, glam::DVec4, f64, as_dvec4, false);
scalar_castable_via!(glam::Quat, glam::DQuat, f64, as_dquat, false);
scalar_castable_via!(glam::Mat2, glam::DMat2, f64, as_dmat2, false);
scalar_castable_via!(glam::Mat3, glam::DMat3, f64, as_dmat3, false);
scalar_castable_via!(glam::Mat4, glam::DMat4, f64, as_dmat4, false);
scalar_castable_via!(glam::Affine2, glam::DAffine2, f64, as_daffine2, false);
scalar_castable_via!(glam::Affine3, glam::DAffine3, f64, as_daffine3, false);

scalar_castable_via!(glam::DVec2, glam::Vec2, f32, as_vec2, true);
scalar_castable_via!(glam::DVec3, glam::Vec3, f32, as_vec3, true);
scalar_castable_via!(glam::DVec4, glam::Vec4, f32, as_vec4, true);
scalar_castable_via!(glam::DQuat, glam::Quat, f32, as_quat, true);
scalar_castable_via!(glam::DMat2, glam::Mat2, f32, as_mat2, true);
scalar_castable_via!(glam::DMat3, glam::Mat3, f32, as_mat3, true);
scalar_castable_via!(glam::DMat4, glam::Mat4, f32, as_mat4, true);
scalar_castable_via!(glam::DAffine2, glam::Affine2, f32, as_affine2, true);
scalar_castable_via!(glam::DAffine3, glam::Affine3, f32, as_affine3, true);

// SIMD-aligned f32 types: cast to canonical Vec3/Mat3/Affine3 (unalign) for f32,
// and round-trip through the canonical types to reach f64 forms.
impl ScalarCastable<f32> for glam::Vec3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::Vec3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        glam::Vec3::from(*self)
    }
}
impl ScalarCastable<f64> for glam::Vec3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::DVec3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        glam::Vec3::from(*self).as_dvec3()
    }
}
impl ScalarCastable<f32> for glam::Mat3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::Mat3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        glam::Mat3::from(*self)
    }
}
impl ScalarCastable<f64> for glam::Mat3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::DMat3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        self.as_dmat3()
    }
}
impl ScalarCastable<f32> for glam::Affine3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::Affine3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        glam::Affine3::from(*self)
    }
}
impl ScalarCastable<f64> for glam::Affine3A {
    const LOSSY_CAST: bool = false;
    type Casted = glam::DAffine3;
    #[inline]
    fn cast(&self) -> Self::Casted {
        self.as_daffine3()
    }
}
