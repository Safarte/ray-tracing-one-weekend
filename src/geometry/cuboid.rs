use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::{HitRecord, Material},
    ray::Ray,
    vec3::Point3,
};

use super::{
    aarect::{XYRect, XZRect, YZRect},
    Hittable, Hittables,
};

pub struct Cuboid {
    min: Point3,
    max: Point3,
    sides: Hittables,
}

impl Cuboid {
    pub fn new(min: Point3, max: Point3, mat: Arc<dyn Material>) -> Cuboid {
        let mut sides: Hittables = Vec::new();

        sides.push(Arc::new(XYRect::new(
            min[0],
            max[0],
            min[1],
            max[1],
            max[2],
            mat.clone(),
        )));
        sides.push(Arc::new(XYRect::new(
            min[0],
            max[0],
            min[1],
            max[1],
            min[2],
            mat.clone(),
        )));

        sides.push(Arc::new(XZRect::new(
            min[0],
            max[0],
            min[2],
            max[2],
            max[1],
            mat.clone(),
        )));
        sides.push(Arc::new(XZRect::new(
            min[0],
            max[0],
            min[2],
            max[2],
            min[1],
            mat.clone(),
        )));

        sides.push(Arc::new(YZRect::new(
            min[1],
            max[1],
            min[2],
            max[2],
            max[0],
            mat.clone(),
        )));
        sides.push(Arc::new(YZRect::new(
            min[1],
            max[1],
            min[2],
            max[2],
            min[0],
            mat.clone(),
        )));

        Cuboid { min, max, sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.min,
            max: self.max,
        })
    }
}
