use crate::{FloatAffine, TAffine2, TAffine3};

impl FloatAffine<f32> for glam::Affine2 {
    type Vec = glam::Vec2;
    type Mat = glam::Mat2;

    const ZERO: Self = glam::Affine2::ZERO;
    const IDENTITY: Self = glam::Affine2::IDENTITY;
    const NAN: Self = glam::Affine2::NAN;

    #[inline]
    fn from_scale(scale: Self::Vec) -> Self {
        glam::Affine2::from_scale(scale)
    }
    #[inline]
    fn from_translation(translation: Self::Vec) -> Self {
        glam::Affine2::from_translation(translation)
    }
    #[inline]
    fn from_cols_slice(slice: &[f32]) -> Self {
        glam::Affine2::from_cols_slice(slice)
    }
    #[inline]
    fn write_cols_to_slice(&self, slice: &mut [f32]) {
        glam::Affine2::write_cols_to_slice(self, slice)
    }
    #[inline]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_point2(rhs)
    }
    #[inline]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_vector2(rhs)
    }
    #[inline]
    fn is_finite(&self) -> bool {
        glam::Affine2::is_finite(self)
    }
    #[inline]
    fn is_nan(&self) -> bool {
        glam::Affine2::is_nan(self)
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        glam::Affine2::abs_diff_eq(self, rhs, max_abs_diff)
    }
    #[inline]
    fn inverse(&self) -> Self {
        glam::Affine2::inverse(self)
    }
}

impl FloatAffine<f64> for glam::DAffine2 {
    type Vec = glam::DVec2;
    type Mat = glam::DMat2;

    const ZERO: Self = glam::DAffine2::ZERO;
    const IDENTITY: Self = glam::DAffine2::IDENTITY;
    const NAN: Self = glam::DAffine2::NAN;

    #[inline]
    fn from_scale(scale: Self::Vec) -> Self {
        glam::DAffine2::from_scale(scale)
    }
    #[inline]
    fn from_translation(translation: Self::Vec) -> Self {
        glam::DAffine2::from_translation(translation)
    }
    #[inline]
    fn from_cols_slice(slice: &[f64]) -> Self {
        glam::DAffine2::from_cols_slice(slice)
    }
    #[inline]
    fn write_cols_to_slice(&self, slice: &mut [f64]) {
        glam::DAffine2::write_cols_to_slice(self, slice)
    }
    #[inline]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_point2(rhs)
    }
    #[inline]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_vector2(rhs)
    }
    #[inline]
    fn is_finite(&self) -> bool {
        glam::DAffine2::is_finite(self)
    }
    #[inline]
    fn is_nan(&self) -> bool {
        glam::DAffine2::is_nan(self)
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
        glam::DAffine2::abs_diff_eq(self, rhs, max_abs_diff)
    }
    #[inline]
    fn inverse(&self) -> Self {
        glam::DAffine2::inverse(self)
    }
}

impl FloatAffine<f32> for glam::Affine3 {
    type Vec = glam::Vec3;
    type Mat = glam::Mat3;

    const ZERO: Self = glam::Affine3::ZERO;
    const IDENTITY: Self = glam::Affine3::IDENTITY;
    const NAN: Self = glam::Affine3::NAN;

    #[inline]
    fn from_scale(scale: Self::Vec) -> Self {
        glam::Affine3::from_scale(scale)
    }
    #[inline]
    fn from_translation(translation: Self::Vec) -> Self {
        glam::Affine3::from_translation(translation)
    }
    #[inline]
    fn from_cols_slice(slice: &[f32]) -> Self {
        glam::Affine3::from_cols_slice(slice)
    }
    #[inline]
    fn write_cols_to_slice(&self, slice: &mut [f32]) {
        glam::Affine3::write_cols_to_slice(self, slice)
    }
    #[inline]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_point3(rhs)
    }
    #[inline]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_vector3(rhs)
    }
    #[inline]
    fn is_finite(&self) -> bool {
        glam::Affine3::is_finite(self)
    }
    #[inline]
    fn is_nan(&self) -> bool {
        glam::Affine3::is_nan(self)
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        glam::Affine3::abs_diff_eq(self, rhs, max_abs_diff)
    }
    #[inline]
    fn inverse(&self) -> Self {
        glam::Affine3::inverse(self)
    }
}

impl FloatAffine<f32> for glam::Affine3A {
    type Vec = glam::Vec3;
    type Mat = glam::Mat3;

    const ZERO: Self = glam::Affine3A::ZERO;
    const IDENTITY: Self = glam::Affine3A::IDENTITY;
    const NAN: Self = glam::Affine3A::NAN;

    #[inline]
    fn from_scale(scale: Self::Vec) -> Self {
        glam::Affine3A::from_scale(scale)
    }
    #[inline]
    fn from_translation(translation: Self::Vec) -> Self {
        glam::Affine3A::from_translation(translation)
    }
    #[inline]
    fn from_cols_slice(slice: &[f32]) -> Self {
        glam::Affine3A::from_cols_slice(slice)
    }
    #[inline]
    fn write_cols_to_slice(&self, slice: &mut [f32]) {
        glam::Affine3A::write_cols_to_slice(self, slice)
    }
    #[inline]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_point3(rhs)
    }
    #[inline]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_vector3(rhs)
    }
    #[inline]
    fn is_finite(&self) -> bool {
        glam::Affine3A::is_finite(self)
    }
    #[inline]
    fn is_nan(&self) -> bool {
        glam::Affine3A::is_nan(self)
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        glam::Affine3A::abs_diff_eq(self, rhs, max_abs_diff)
    }
    #[inline]
    fn inverse(&self) -> Self {
        glam::Affine3A::inverse(self)
    }
}

impl FloatAffine<f64> for glam::DAffine3 {
    type Vec = glam::DVec3;
    type Mat = glam::DMat3;

    const ZERO: Self = glam::DAffine3::ZERO;
    const IDENTITY: Self = glam::DAffine3::IDENTITY;
    const NAN: Self = glam::DAffine3::NAN;

    #[inline]
    fn from_scale(scale: Self::Vec) -> Self {
        glam::DAffine3::from_scale(scale)
    }
    #[inline]
    fn from_translation(translation: Self::Vec) -> Self {
        glam::DAffine3::from_translation(translation)
    }
    #[inline]
    fn from_cols_slice(slice: &[f64]) -> Self {
        glam::DAffine3::from_cols_slice(slice)
    }
    #[inline]
    fn write_cols_to_slice(&self, slice: &mut [f64]) {
        glam::DAffine3::write_cols_to_slice(self, slice)
    }
    #[inline]
    fn transform_point(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_point3(rhs)
    }
    #[inline]
    fn transform_vector(&self, rhs: Self::Vec) -> Self::Vec {
        self.transform_vector3(rhs)
    }
    #[inline]
    fn is_finite(&self) -> bool {
        glam::DAffine3::is_finite(self)
    }
    #[inline]
    fn is_nan(&self) -> bool {
        glam::DAffine3::is_nan(self)
    }
    #[inline]
    fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
        glam::DAffine3::abs_diff_eq(self, rhs, max_abs_diff)
    }
    #[inline]
    fn inverse(&self) -> Self {
        glam::DAffine3::inverse(self)
    }
}

macro_rules! impl_interface_affine2 {
    ($t:ty, $f:ty, $vec2:ty, $mat2:ty, $mat3:ty) => {
        impl TAffine2<$f> for $t {
            type MaybeAligned = $t;

            #[inline]
            fn maybe_align(&self) -> Self::MaybeAligned {
                *self
            }
            #[inline]
            fn matrix2(&self) -> $mat2 {
                self.matrix2
            }
            #[inline]
            fn translation(&self) -> $vec2 {
                self.translation
            }
            #[inline]
            fn from_cols(x_axis: $vec2, y_axis: $vec2, z_axis: $vec2) -> Self {
                <$t>::from_cols(x_axis, y_axis, z_axis)
            }
            #[inline]
            fn from_mat2_translation(matrix2: $mat2, translation: $vec2) -> Self {
                <$t>::from_mat2_translation(matrix2, translation)
            }
            #[inline]
            fn from_cols_array(m: &[$f; 6]) -> Self {
                <$t>::from_cols_array(m)
            }
            #[inline]
            fn to_cols_array(&self) -> [$f; 6] {
                <$t>::to_cols_array(self)
            }
            #[inline]
            fn from_cols_array_2d(m: &[[$f; 2]; 3]) -> Self {
                <$t>::from_cols_array_2d(m)
            }
            #[inline]
            fn to_cols_array_2d(&self) -> [[$f; 2]; 3] {
                <$t>::to_cols_array_2d(self)
            }
            #[inline]
            fn from_mat2(matrix2: $mat2) -> Self {
                <$t>::from_mat2(matrix2)
            }
            #[inline]
            fn from_scale_angle_translation(scale: $vec2, angle: $f, translation: $vec2) -> Self {
                <$t>::from_scale_angle_translation(scale, angle, translation)
            }
            #[inline]
            fn from_angle_translation(angle: $f, translation: $vec2) -> Self {
                <$t>::from_angle_translation(angle, translation)
            }
            #[inline]
            fn from_mat3(m: $mat3) -> Self {
                <$t>::from_mat3(m)
            }
            #[inline]
            fn to_scale_angle_translation(&self) -> ($vec2, $f, $vec2) {
                <$t>::to_scale_angle_translation(self)
            }
        }
    };
}

impl_interface_affine2!(glam::Affine2, f32, glam::Vec2, glam::Mat2, glam::Mat3);
impl_interface_affine2!(glam::DAffine2, f64, glam::DVec2, glam::DMat2, glam::DMat3);

macro_rules! impl_interface_affine3 {
    (
        $t:ty,
        $f:ty,
        $vec3:ty,
        $quat:ty,
        $mat3:ty,
        $mat4:ty,
        $aligned:ty,
        $align:expr,
        $from_cols:expr
    ) => {
        impl TAffine3<$f> for $t {
            type MaybeAligned = $aligned;

            #[inline]
            fn maybe_align(&self) -> Self::MaybeAligned {
                let f: fn(&$t) -> $aligned = $align;
                f(self)
            }
            #[inline]
            fn matrix3(&self) -> $mat3 {
                <$mat3>::from(self.matrix3)
            }
            #[inline]
            fn translation(&self) -> $vec3 {
                <$vec3>::from(self.translation)
            }
            #[inline]
            fn from_cols(
                x_axis: $vec3,
                y_axis: $vec3,
                z_axis: $vec3,
                w_axis: $vec3,
            ) -> Self {
                let f: fn($vec3, $vec3, $vec3, $vec3) -> $t = $from_cols;
                f(x_axis, y_axis, z_axis, w_axis)
            }
            #[inline]
            fn from_mat3_translation(matrix3: $mat3, translation: $vec3) -> Self {
                <$t>::from_mat3_translation(matrix3, translation)
            }
            #[inline]
            fn from_cols_array(m: &[$f; 12]) -> Self {
                <$t>::from_cols_array(m)
            }
            #[inline]
            fn to_cols_array(&self) -> [$f; 12] {
                <$t>::to_cols_array(self)
            }
            #[inline]
            fn from_cols_array_2d(m: &[[$f; 3]; 4]) -> Self {
                <$t>::from_cols_array_2d(m)
            }
            #[inline]
            fn to_cols_array_2d(&self) -> [[$f; 3]; 4] {
                <$t>::to_cols_array_2d(self)
            }
            #[inline]
            fn from_quat(rotation: $quat) -> Self {
                <$t>::from_quat(rotation)
            }
            #[inline]
            fn from_axis_angle(axis: $vec3, angle: $f) -> Self {
                <$t>::from_axis_angle(axis, angle)
            }
            #[inline]
            fn from_mat3(mat3: $mat3) -> Self {
                <$t>::from_mat3(mat3)
            }
            #[inline]
            fn from_scale_rotation_translation(
                scale: $vec3,
                rotation: $quat,
                translation: $vec3,
            ) -> Self {
                <$t>::from_scale_rotation_translation(scale, rotation, translation)
            }
            #[inline]
            fn from_rotation_translation(rotation: $quat, translation: $vec3) -> Self {
                <$t>::from_rotation_translation(rotation, translation)
            }
            #[inline]
            fn from_mat4(m: $mat4) -> Self {
                <$t>::from_mat4(m)
            }
            #[inline]
            fn to_scale_rotation_translation(&self) -> ($vec3, $quat, $vec3) {
                <$t>::to_scale_rotation_translation(self)
            }
        }
    };
}

impl_interface_affine3!(
    glam::Affine3,
    f32,
    glam::Vec3,
    glam::Quat,
    glam::Mat3,
    glam::Mat4,
    glam::Affine3A,
    |a| glam::Affine3A::from(*a),
    |x, y, z, w| glam::Affine3::from_cols(x, y, z, w)
);
impl_interface_affine3!(
    glam::Affine3A,
    f32,
    glam::Vec3,
    glam::Quat,
    glam::Mat3,
    glam::Mat4,
    glam::Affine3A,
    |a| *a,
    |x, y, z, w| glam::Affine3A::from_cols(
        glam::Vec3A::from(x),
        glam::Vec3A::from(y),
        glam::Vec3A::from(z),
        glam::Vec3A::from(w),
    )
);
impl_interface_affine3!(
    glam::DAffine3,
    f64,
    glam::DVec3,
    glam::DQuat,
    glam::DMat3,
    glam::DMat4,
    glam::DAffine3,
    |a| *a,
    |x, y, z, w| glam::DAffine3::from_cols(x, y, z, w)
);
