use bevy_ecs::{system::Resource, component::Component};
use glam::Vec3;

use crate::prelude::Instance;

#[derive(Component,Resource)]
pub struct OrientedBoundingBox {
    pub aabb_min: Vec3,
    pub aabb_max: Vec3,
}
impl OrientedBoundingBox {
    pub fn new(x_len: f32, y_len: f32, z_len: f32) -> Self {
        let x = x_len/2.;
        let y = y_len/2.;
        let z = z_len/2.;   
        Self {
            aabb_min: Vec3::new(-x,-y,-z),
            aabb_max: Vec3::new(x,y,z),
        }
    }
    pub fn check_collision_with_ray(&self, ray_origin: Vec3, ray_direction: Vec3, instance: &Instance) -> (bool,f32) {
        if !instance.enabled {
            return (false,0.0);
        }
        let model_matrix = instance.to_raw().unwrap().model;
        if sphere_with_ray_collision(ray_origin, ray_direction, 2., Vec3::new(0., 0., 0.)){
            return (true,-1.)
        }
        (false,0.)
        //oriented_bounding_box_with_ray(ray_origin, ray_direction, self.aabb_min, self.aabb_max, model_matrix)
    }
}
#[derive(Component,Resource)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}
