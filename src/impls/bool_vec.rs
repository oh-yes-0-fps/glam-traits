use crate::{BoolVec, TBVec2, TBVec3, TBVec4};

macro_rules! impl_bool_vec {
    ($t:ty, $false_const:expr, $true_const:expr) => {
        impl BoolVec for $t {
            const FALSE: Self = $false_const;
            const TRUE: Self = $true_const;

            #[inline]
            fn splat(value: bool) -> Self {
                <$t>::splat(value)
            }
            #[inline]
            fn bitmask(self) -> u32 {
                <$t>::bitmask(self)
            }
            #[inline]
            fn any(self) -> bool {
                <$t>::any(self)
            }
            #[inline]
            fn all(self) -> bool {
                <$t>::all(self)
            }
            #[inline]
            fn test(self, index: usize) -> bool {
                <$t>::test(self, index)
            }
            #[inline]
            fn set(&mut self, index: usize, value: bool) {
                <$t>::set(self, index, value)
            }
        }
    };
}

impl_bool_vec!(glam::BVec2, glam::BVec2::new(false, false), glam::BVec2::new(true, true));
impl_bool_vec!(
    glam::BVec3,
    glam::BVec3::new(false, false, false),
    glam::BVec3::new(true, true, true)
);
impl_bool_vec!(
    glam::BVec4,
    glam::BVec4::new(false, false, false, false),
    glam::BVec4::new(true, true, true, true)
);

impl TBVec2 for glam::BVec2 {}
impl TBVec3 for glam::BVec3 {}
impl TBVec4 for glam::BVec4 {}
