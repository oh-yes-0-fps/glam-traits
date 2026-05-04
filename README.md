# glam_trait

Traits over [`glam`](https://docs.rs/glam) types — write code generic over `f32`/`f64`, vector arity, matrix size, etc.

## What it gives you

- Scalar traits: `FloatScalar` (f32, f64), `IntScalar` (i32, i64), `BoolScalar` (bool) — each exposes associated `Vec2/3/4`, `Mat2/3/4`, `Quat`, `Affine2/3` types.
- Per-shape interfaces: `TVec2/3/4<F>`, `TQuat<F>`, `TMat2/3/4<F>`, `TAffine2/3<F>`, `TBVec2/3/4`, `TIVec2/3/4<I>`.
- Shared supertraits: `FloatVec<F>`, `FloatMat<F>`, `FloatAffine<F>`, `BoolVec`, `IntVec<I>` — collapse common arithmetic/methods across dimensions.
- Rotation/transform: `RotationLike3d<F>` (Quat, Mat3), `TransRotLike3d<F>` (Mat4, Affine3).
- Layout: `Alignable`/`Unalignable` for `*A` aligned variants, `LossyAlignable` for f64 → f32 aligned, `ArrayLike<N>` and `ArrayLike2D`.
- Casts: `ScalarCastable<F>` for f32 ↔ f64 conversions.

All impls land on the matching glam types. Compile-time `const _: fn()` assertions in `impls::assertions` keep the bound surface honest.

## Usage

```rust
use glam_trait::{FloatScalar, FloatVec, TVec3};

fn centroid<F: FloatScalar>(points: &[F::Vec3]) -> F::Vec3 {
    let n = F::from(points.len()).unwrap();
    points.iter().copied().sum::<F::Vec3>() / n
}

let p32 = centroid::<f32>(&[glam::Vec3::ONE, glam::Vec3::ZERO]);
let p64 = centroid::<f64>(&[glam::DVec3::ONE, glam::DVec3::ZERO]);
```

## Features

| feature     | enables                                              |
|-------------|------------------------------------------------------|
| `approx`    | `AbsDiffEq + RelativeEq + UlpsEq` bound on float types |
| `serde`     | `Serialize + DeserializeOwned`                       |
| `bytemuck`  | `Pod + Zeroable` (vecs/mats/quats only)              |
| `zerocopy`  | `IntoBytes + FromBytes + Immutable + KnownLayout`    |
| `arbitrary` | `for<'a> Arbitrary<'a>`                              |
| `ndarray`   | `to_ndarray`/`from_ndarray` on `ArrayLike`/`ArrayLike2D` |

Each forwards to glam's matching feature when applicable. Bounds use empty private trait shims so the public surface is identical across feature flags.

## Notes

- Default methods (length, normalize, distance, lerp, mul_add, etc.) are provided wherever derivable from primitives — implementers only need the foundational ops.
- `IntScalar` only impls signed types (i32, i64); unsigned glam vecs lack `perp`/`rotate`.
- Methods returning values are `#[must_use]`; defaulted methods are `#[inline]`.
