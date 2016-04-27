use na::{self, Point2, Point4};
use math::{Scalar, Point, Vect};
use material::Material;

pub struct NormalMaterial;

impl Material for NormalMaterial {
    #[inline]
    fn ambiant(&self, _: &Point, normal: &Vect, _: &Option<Point2<Scalar>>) -> Point4<f32> {
        Point4::new((1.0f32 + na::cast::<Scalar, f32>(normal.x.clone())) / 2.0,
                  (1.0f32 + na::cast::<Scalar, f32>(normal.y.clone())) / 2.0,
                  (1.0f32 + na::cast::<Scalar, f32>(normal.z.clone())) / 2.0,
                  1.0)
    }
}

impl NormalMaterial {
    #[inline]
    pub fn new() -> NormalMaterial {
        NormalMaterial
    }
}
