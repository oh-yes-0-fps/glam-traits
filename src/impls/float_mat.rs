use crate::{FloatMat, TMat2, TMat3, TMat4};

macro_rules! impl_float_mat {
    ($mat:ty, $f:ty, $col:ty) => {
        impl FloatMat<$f> for $mat {
            type Col = $col;

            const ZERO: Self = <$mat>::ZERO;
            const IDENTITY: Self = <$mat>::IDENTITY;
            const NAN: Self = <$mat>::NAN;

            #[inline]
            fn from_diagonal(diagonal: Self::Col) -> Self {
                <$mat>::from_diagonal(diagonal)
            }
            #[inline]
            fn from_cols_slice(slice: &[$f]) -> Self {
                <$mat>::from_cols_slice(slice)
            }
            #[inline]
            fn write_cols_to_slice(&self, slice: &mut [$f]) {
                <$mat>::write_cols_to_slice(self, slice)
            }
            #[inline]
            fn col(&self, index: usize) -> Self::Col {
                <$mat>::col(self, index)
            }
            #[inline]
            fn col_mut(&mut self, index: usize) -> &mut Self::Col {
                <$mat>::col_mut(self, index)
            }
            #[inline]
            fn row(&self, index: usize) -> Self::Col {
                <$mat>::row(self, index)
            }
            #[inline]
            fn is_finite(&self) -> bool {
                <$mat>::is_finite(self)
            }
            #[inline]
            fn is_nan(&self) -> bool {
                <$mat>::is_nan(self)
            }
            #[inline]
            fn transpose(&self) -> Self {
                <$mat>::transpose(self)
            }
            #[inline]
            fn diagonal(&self) -> Self::Col {
                <$mat>::diagonal(self)
            }
            #[inline]
            fn determinant(&self) -> $f {
                <$mat>::determinant(self)
            }
            #[inline]
            fn inverse(&self) -> Self {
                <$mat>::inverse(self)
            }
            #[inline]
            fn try_inverse(&self) -> Option<Self> {
                <$mat>::try_inverse(self)
            }
            #[inline]
            fn mul_diagonal_scale(&self, scale: Self::Col) -> Self {
                <$mat>::mul_diagonal_scale(self, scale)
            }
            #[inline]
            fn recip(&self) -> Self {
                <$mat>::recip(self)
            }
            #[inline]
            fn abs_diff_eq(&self, rhs: Self, max_abs_diff: $f) -> bool {
                <$mat>::abs_diff_eq(self, rhs, max_abs_diff)
            }
            #[inline]
            fn abs(&self) -> Self {
                <$mat>::abs(self)
            }
        }
    };
}

impl_float_mat!(glam::Mat2, f32, glam::Vec2);
impl_float_mat!(glam::Mat3, f32, glam::Vec3);
impl_float_mat!(glam::Mat4, f32, glam::Vec4);
impl_float_mat!(glam::DMat2, f64, glam::DVec2);
impl_float_mat!(glam::DMat3, f64, glam::DVec3);
impl_float_mat!(glam::DMat4, f64, glam::DVec4);

macro_rules! impl_interface_mat2 {
    ($mat:ty, $f:ty, $vec2:ty, $mat3:ty) => {
        impl TMat2<$f> for $mat {
            #[inline]
            fn from_cols(x_axis: $vec2, y_axis: $vec2) -> Self {
                <$mat>::from_cols(x_axis, y_axis)
            }
            #[inline]
            fn from_cols_array(m: &[$f; 4]) -> Self {
                <$mat>::from_cols_array(m)
            }
            #[inline]
            fn to_cols_array(&self) -> [$f; 4] {
                <$mat>::to_cols_array(self)
            }
            #[inline]
            fn from_cols_array_2d(m: &[[$f; 2]; 2]) -> Self {
                <$mat>::from_cols_array_2d(m)
            }
            #[inline]
            fn to_cols_array_2d(&self) -> [[$f; 2]; 2] {
                <$mat>::to_cols_array_2d(self)
            }
            #[inline]
            fn from_scale_angle(scale: $vec2, angle: $f) -> Self {
                <$mat>::from_scale_angle(scale, angle)
            }
            #[inline]
            fn from_mat3(m: $mat3) -> Self {
                <$mat>::from_mat3(m)
            }
            #[inline]
            fn from_mat3_minor(m: $mat3, i: usize, j: usize) -> Self {
                <$mat>::from_mat3_minor(m, i, j)
            }
        }
    };
}

impl_interface_mat2!(glam::Mat2, f32, glam::Vec2, glam::Mat3);
impl_interface_mat2!(glam::DMat2, f64, glam::DVec2, glam::DMat3);

macro_rules! impl_interface_mat3 {
    ($mat:ty, $f:ty, $vec2:ty, $vec3:ty, $mat2:ty, $mat4:ty) => {
        impl TMat3<$f> for $mat {
            #[inline]
            fn from_cols(x_axis: $vec3, y_axis: $vec3, z_axis: $vec3) -> Self {
                <$mat>::from_cols(x_axis, y_axis, z_axis)
            }
            #[inline]
            fn from_cols_array(m: &[$f; 9]) -> Self {
                <$mat>::from_cols_array(m)
            }
            #[inline]
            fn to_cols_array(&self) -> [$f; 9] {
                <$mat>::to_cols_array(self)
            }
            #[inline]
            fn from_cols_array_2d(m: &[[$f; 3]; 3]) -> Self {
                <$mat>::from_cols_array_2d(m)
            }
            #[inline]
            fn to_cols_array_2d(&self) -> [[$f; 3]; 3] {
                <$mat>::to_cols_array_2d(self)
            }
            #[inline]
            fn from_mat4(m: $mat4) -> Self {
                <$mat>::from_mat4(m)
            }
            #[inline]
            fn from_mat4_minor(m: $mat4, i: usize, j: usize) -> Self {
                <$mat>::from_mat4_minor(m, i, j)
            }
            #[inline]
            fn from_translation(translation: $vec2) -> Self {
                <$mat>::from_translation(translation)
            }
            #[inline]
            fn from_scale_angle_translation(scale: $vec2, angle: $f, translation: $vec2) -> Self {
                <$mat>::from_scale_angle_translation(scale, angle, translation)
            }
            #[inline]
            fn from_scale(scale: $vec2) -> Self {
                <$mat>::from_scale(scale)
            }
            #[inline]
            fn from_mat2(m: $mat2) -> Self {
                <$mat>::from_mat2(m)
            }
            #[inline]
            fn transform_point2(&self, rhs: $vec2) -> $vec2 {
                <$mat>::transform_point2(self, rhs)
            }
            #[inline]
            fn transform_vector2(&self, rhs: $vec2) -> $vec2 {
                <$mat>::transform_vector2(self, rhs)
            }
        }
    };
}

impl_interface_mat3!(glam::Mat3, f32, glam::Vec2, glam::Vec3, glam::Mat2, glam::Mat4);
impl_interface_mat3!(glam::DMat3, f64, glam::DVec2, glam::DVec3, glam::DMat2, glam::DMat4);

macro_rules! impl_interface_mat4 {
    ($mat:ty, $f:ty, $vec3:ty, $vec4:ty, $quat:ty, $mat3:ty) => {
        impl TMat4<$f> for $mat {
            #[inline]
            fn from_cols(x_axis: $vec4, y_axis: $vec4, z_axis: $vec4, w_axis: $vec4) -> Self {
                <$mat>::from_cols(x_axis, y_axis, z_axis, w_axis)
            }
            #[inline]
            fn from_cols_array(m: &[$f; 16]) -> Self {
                <$mat>::from_cols_array(m)
            }
            #[inline]
            fn to_cols_array(&self) -> [$f; 16] {
                <$mat>::to_cols_array(self)
            }
            #[inline]
            fn from_cols_array_2d(m: &[[$f; 4]; 4]) -> Self {
                <$mat>::from_cols_array_2d(m)
            }
            #[inline]
            fn to_cols_array_2d(&self) -> [[$f; 4]; 4] {
                <$mat>::to_cols_array_2d(self)
            }
            #[inline]
            fn from_scale_rotation_translation(
                scale: $vec3,
                rotation: $quat,
                translation: $vec3,
            ) -> Self {
                <$mat>::from_scale_rotation_translation(scale, rotation, translation)
            }
            #[inline]
            fn from_rotation_translation(rotation: $quat, translation: $vec3) -> Self {
                <$mat>::from_rotation_translation(rotation, translation)
            }
            #[inline]
            fn to_scale_rotation_translation(&self) -> ($vec3, $quat, $vec3) {
                <$mat>::to_scale_rotation_translation(self)
            }
            #[inline]
            fn from_quat(rotation: $quat) -> Self {
                <$mat>::from_quat(rotation)
            }
            #[inline]
            fn from_mat3(m: $mat3) -> Self {
                <$mat>::from_mat3(m)
            }
            #[inline]
            fn from_mat3_translation(mat3: $mat3, translation: $vec3) -> Self {
                <$mat>::from_mat3_translation(mat3, translation)
            }
            #[inline]
            fn from_translation(translation: $vec3) -> Self {
                <$mat>::from_translation(translation)
            }
            #[inline]
            fn from_axis_angle(axis: $vec3, angle: $f) -> Self {
                <$mat>::from_axis_angle(axis, angle)
            }
            #[inline]
            fn from_euler(order: glam::EulerRot, a: $f, b: $f, c: $f) -> Self {
                <$mat>::from_euler(order, a, b, c)
            }
            #[inline]
            fn to_euler(&self, order: glam::EulerRot) -> ($f, $f, $f) {
                <$mat>::to_euler(self, order)
            }
            #[inline]
            fn from_scale(scale: $vec3) -> Self {
                <$mat>::from_scale(scale)
            }
            #[inline]
            fn frustum_lh(left: $f, right: $f, bottom: $f, top: $f, near: $f, far: $f) -> Self {
                <$mat>::frustum_lh(left, right, bottom, top, near, far)
            }
            #[inline]
            fn frustum_rh(left: $f, right: $f, bottom: $f, top: $f, near: $f, far: $f) -> Self {
                <$mat>::frustum_rh(left, right, bottom, top, near, far)
            }
            #[inline]
            fn frustum_rh_gl(
                left: $f,
                right: $f,
                bottom: $f,
                top: $f,
                near: $f,
                far: $f,
            ) -> Self {
                <$mat>::frustum_rh_gl(left, right, bottom, top, near, far)
            }
            #[inline]
            fn perspective_lh(
                fov_y_radians: $f,
                aspect_ratio: $f,
                z_near: $f,
                z_far: $f,
            ) -> Self {
                <$mat>::perspective_lh(fov_y_radians, aspect_ratio, z_near, z_far)
            }
            #[inline]
            fn perspective_rh(
                fov_y_radians: $f,
                aspect_ratio: $f,
                z_near: $f,
                z_far: $f,
            ) -> Self {
                <$mat>::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)
            }
            #[inline]
            fn perspective_rh_gl(
                fov_y_radians: $f,
                aspect_ratio: $f,
                z_near: $f,
                z_far: $f,
            ) -> Self {
                <$mat>::perspective_rh_gl(fov_y_radians, aspect_ratio, z_near, z_far)
            }
            #[inline]
            fn perspective_infinite_lh(fov_y_radians: $f, aspect_ratio: $f, z_near: $f) -> Self {
                <$mat>::perspective_infinite_lh(fov_y_radians, aspect_ratio, z_near)
            }
            #[inline]
            fn perspective_infinite_reverse_lh(
                fov_y_radians: $f,
                aspect_ratio: $f,
                z_near: $f,
            ) -> Self {
                <$mat>::perspective_infinite_reverse_lh(fov_y_radians, aspect_ratio, z_near)
            }
            #[inline]
            fn perspective_infinite_rh(fov_y_radians: $f, aspect_ratio: $f, z_near: $f) -> Self {
                <$mat>::perspective_infinite_rh(fov_y_radians, aspect_ratio, z_near)
            }
            #[inline]
            fn perspective_infinite_reverse_rh(
                fov_y_radians: $f,
                aspect_ratio: $f,
                z_near: $f,
            ) -> Self {
                <$mat>::perspective_infinite_reverse_rh(fov_y_radians, aspect_ratio, z_near)
            }
            #[inline]
            fn orthographic_lh(
                left: $f,
                right: $f,
                bottom: $f,
                top: $f,
                near: $f,
                far: $f,
            ) -> Self {
                <$mat>::orthographic_lh(left, right, bottom, top, near, far)
            }
            #[inline]
            fn orthographic_rh(
                left: $f,
                right: $f,
                bottom: $f,
                top: $f,
                near: $f,
                far: $f,
            ) -> Self {
                <$mat>::orthographic_rh(left, right, bottom, top, near, far)
            }
            #[inline]
            fn orthographic_rh_gl(
                left: $f,
                right: $f,
                bottom: $f,
                top: $f,
                near: $f,
                far: $f,
            ) -> Self {
                <$mat>::orthographic_rh_gl(left, right, bottom, top, near, far)
            }
            #[inline]
            fn project_point3(&self, rhs: $vec3) -> $vec3 {
                <$mat>::project_point3(self, rhs)
            }
            #[inline]
            fn transform_point3(&self, rhs: $vec3) -> $vec3 {
                <$mat>::transform_point3(self, rhs)
            }
            #[inline]
            fn transform_vector3(&self, rhs: $vec3) -> $vec3 {
                <$mat>::transform_vector3(self, rhs)
            }
        }
    };
}

impl_interface_mat4!(glam::Mat4, f32, glam::Vec3, glam::Vec4, glam::Quat, glam::Mat3);
impl_interface_mat4!(glam::DMat4, f64, glam::DVec3, glam::DVec4, glam::DQuat, glam::DMat3);
