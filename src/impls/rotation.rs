use crate::{RotationLike3d, TransRotLike3d};

macro_rules! impl_rotation_like_mat3 {
    ($t:ty, $f:ty, $vec3:ty, $mat3:ty, $quat:ty) => {
        impl RotationLike3d<$f> for $t {
            #[inline]
            fn to_mat3(&self) -> $mat3 {
                *self
            }
            #[inline]
            fn to_quat(&self) -> $quat {
                <$quat>::from_mat3(self)
            }
            #[inline]
            fn to_euler(&self, order: glam::EulerRot) -> $vec3 {
                let (a, b, c) = self.to_euler(order);
                <$vec3>::new(a, b, c)
            }
            #[inline]
            fn to_axis_angle(&self) -> ($vec3, $f) {
                <$quat>::from_mat3(self).to_axis_angle()
            }
            #[inline]
            fn to_rodrigues(&self) -> $vec3 {
                let (axis, angle) = self.to_axis_angle();
                axis * (angle * 0.5 as $f).tan() * 2.0 as $f
            }
            #[inline]
            fn to_rot_vec(&self) -> $vec3 {
                <$quat>::from_mat3(self).to_scaled_axis()
            }
            #[inline]
            fn from_mat3(mat: $mat3) -> Self {
                mat
            }
            #[inline]
            fn from_quat(quat: $quat) -> Self {
                <$mat3>::from_quat(quat)
            }
            #[inline]
            fn from_euler(euler: $vec3, order: glam::EulerRot) -> Self {
                <$mat3>::from_euler(order, euler.x, euler.y, euler.z)
            }
            #[inline]
            fn from_axis_angle(axis: $vec3, angle: $f) -> Self {
                <$mat3>::from_axis_angle(axis, angle)
            }
            #[inline]
            fn from_rodrigues(rodrigues: $vec3) -> Self {
                let len = rodrigues.length();
                if len > 0.0 as $f {
                    let angle = (len * 0.5 as $f).atan() * 2.0 as $f;
                    <$mat3>::from_axis_angle(rodrigues / len, angle)
                } else {
                    <$mat3>::IDENTITY
                }
            }
            #[inline]
            fn from_rot_vec(rot_vec: $vec3) -> Self {
                <$mat3>::from_quat(<$quat>::from_scaled_axis(rot_vec))
            }
            #[inline]
            fn from_rotation_x(angle: $f) -> Self {
                <$mat3>::from_rotation_x(angle)
            }
            #[inline]
            fn from_rotation_y(angle: $f) -> Self {
                <$mat3>::from_rotation_y(angle)
            }
            #[inline]
            fn from_rotation_z(angle: $f) -> Self {
                <$mat3>::from_rotation_z(angle)
            }
            #[inline]
            fn from_angle(angle: $f) -> Self {
                <$mat3>::from_angle(angle)
            }
            #[inline]
            fn look_to_lh(dir: $vec3, up: $vec3) -> Self {
                <$mat3>::look_to_lh(dir, up)
            }
            #[inline]
            fn look_to_rh(dir: $vec3, up: $vec3) -> Self {
                <$mat3>::look_to_rh(dir, up)
            }
            #[inline]
            fn look_at_lh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                <$mat3>::look_at_lh(eye, center, up)
            }
            #[inline]
            fn look_at_rh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                <$mat3>::look_at_rh(eye, center, up)
            }
        }
    };
}

impl_rotation_like_mat3!(glam::Mat3, f32, glam::Vec3, glam::Mat3, glam::Quat);
impl_rotation_like_mat3!(glam::DMat3, f64, glam::DVec3, glam::DMat3, glam::DQuat);

macro_rules! impl_rotation_like_quat {
    ($t:ty, $f:ty, $vec3:ty, $mat3:ty) => {
        impl RotationLike3d<$f> for $t {
            #[inline]
            fn to_mat3(&self) -> $mat3 {
                <$mat3>::from_quat(*self)
            }
            #[inline]
            fn to_quat(&self) -> Self {
                *self
            }
            #[inline]
            fn to_euler(&self, order: glam::EulerRot) -> $vec3 {
                let (a, b, c) = <$t>::to_euler(*self, order);
                <$vec3>::new(a, b, c)
            }
            #[inline]
            fn to_axis_angle(&self) -> ($vec3, $f) {
                <$t>::to_axis_angle(*self)
            }
            #[inline]
            fn to_rodrigues(&self) -> $vec3 {
                let (axis, angle) = self.to_axis_angle();
                axis * (angle * 0.5 as $f).tan() * 2.0 as $f
            }
            #[inline]
            fn to_rot_vec(&self) -> $vec3 {
                <$t>::to_scaled_axis(*self)
            }
            #[inline]
            fn from_mat3(mat: $mat3) -> Self {
                <$t>::from_mat3(&mat)
            }
            #[inline]
            fn from_quat(quat: Self) -> Self {
                quat
            }
            #[inline]
            fn from_euler(euler: $vec3, order: glam::EulerRot) -> Self {
                <$t>::from_euler(order, euler.x, euler.y, euler.z)
            }
            #[inline]
            fn from_axis_angle(axis: $vec3, angle: $f) -> Self {
                <$t>::from_axis_angle(axis, angle)
            }
            #[inline]
            fn from_rodrigues(rodrigues: $vec3) -> Self {
                let len = rodrigues.length();
                if len > 0.0 as $f {
                    let angle = (len * 0.5 as $f).atan() * 2.0 as $f;
                    <$t>::from_axis_angle(rodrigues / len, angle)
                } else {
                    <$t>::IDENTITY
                }
            }
            #[inline]
            fn from_rot_vec(rot_vec: $vec3) -> Self {
                <$t>::from_scaled_axis(rot_vec)
            }
            #[inline]
            fn from_rotation_x(angle: $f) -> Self {
                <$t>::from_rotation_x(angle)
            }
            #[inline]
            fn from_rotation_y(angle: $f) -> Self {
                <$t>::from_rotation_y(angle)
            }
            #[inline]
            fn from_rotation_z(angle: $f) -> Self {
                <$t>::from_rotation_z(angle)
            }
            #[inline]
            fn from_angle(angle: $f) -> Self {
                <$t>::from_rotation_z(angle)
            }
            #[inline]
            fn look_to_lh(dir: $vec3, up: $vec3) -> Self {
                <$t>::look_to_lh(dir, up)
            }
            #[inline]
            fn look_to_rh(dir: $vec3, up: $vec3) -> Self {
                <$t>::look_to_rh(dir, up)
            }
            #[inline]
            fn look_at_lh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                <$t>::look_at_lh(eye, center, up)
            }
            #[inline]
            fn look_at_rh(eye: $vec3, center: $vec3, up: $vec3) -> Self {
                <$t>::look_at_rh(eye, center, up)
            }
        }
    };
}

impl_rotation_like_quat!(glam::Quat, f32, glam::Vec3, glam::Mat3);
impl_rotation_like_quat!(glam::DQuat, f64, glam::DVec3, glam::DMat3);

// TransRotLike3d
impl TransRotLike3d<f32> for glam::Mat4 {
    #[inline]
    fn to_affine3(&self) -> glam::Affine3 {
        glam::Affine3::from_mat4(*self)
    }
    #[inline]
    fn to_mat4(&self) -> glam::Mat4 {
        *self
    }
    #[inline]
    fn from_affine3(affine: glam::Affine3) -> Self {
        glam::Mat4::from(affine)
    }
    #[inline]
    fn from_mat4(mat: glam::Mat4) -> Self {
        mat
    }
}

impl TransRotLike3d<f64> for glam::DMat4 {
    #[inline]
    fn to_affine3(&self) -> glam::DAffine3 {
        glam::DAffine3::from_mat4(*self)
    }
    #[inline]
    fn to_mat4(&self) -> glam::DMat4 {
        *self
    }
    #[inline]
    fn from_affine3(affine: glam::DAffine3) -> Self {
        glam::DMat4::from(affine)
    }
    #[inline]
    fn from_mat4(mat: glam::DMat4) -> Self {
        mat
    }
}

impl TransRotLike3d<f32> for glam::Affine3 {
    #[inline]
    fn to_affine3(&self) -> glam::Affine3 {
        *self
    }
    #[inline]
    fn to_mat4(&self) -> glam::Mat4 {
        glam::Mat4::from(*self)
    }
    #[inline]
    fn from_affine3(affine: glam::Affine3) -> Self {
        affine
    }
    #[inline]
    fn from_mat4(mat: glam::Mat4) -> Self {
        glam::Affine3::from_mat4(mat)
    }
}

impl TransRotLike3d<f64> for glam::DAffine3 {
    #[inline]
    fn to_affine3(&self) -> glam::DAffine3 {
        *self
    }
    #[inline]
    fn to_mat4(&self) -> glam::DMat4 {
        glam::DMat4::from(*self)
    }
    #[inline]
    fn from_affine3(affine: glam::DAffine3) -> Self {
        affine
    }
    #[inline]
    fn from_mat4(mat: glam::DMat4) -> Self {
        glam::DAffine3::from_mat4(mat)
    }
}
