use bevy_ecs::{component::Component, system::Resource};
use glam::Vec3;

use crate::prelude::Instance;

use super::collision_fns_3d::{oriented_bounding_box_with_ray, sphere_with_ray_collision};

#[derive(Component, Resource)]
pub enum Collider3D {
    OBB(OBB),
    Sphere(Sphere),
    Ray(Ray),
}
impl Collider3D {
    pub fn check_collision(
        &self,
        parent_instance: Option<&Instance>,
        other: &Self,
        other_instance: Option<&Instance>,
    ) -> ColliderResult {
        match self {
            Collider3D::OBB(obb) => match other {
                Collider3D::OBB(_) => ColliderResult::NotImplemented,
                Collider3D::Sphere(_) => ColliderResult::NotImplemented,
                Collider3D::Ray(ray) => match oriented_bounding_box_with_ray(
                    ray.origin,
                    ray.direction,
                    obb.aabb_min,
                    obb.aabb_max,
                    if parent_instance.is_none() {
                        Instance {
                            ..Default::default()
                        }.to_raw().unwrap().model
                    } else {
                        parent_instance.unwrap().to_raw().unwrap().model
                    },
                ) {
                    Some(dist) => ColliderResult::Collision(dist),
                    None => ColliderResult::NoCollision,
                },
            },
            Collider3D::Sphere(sphere) => match other {
                Collider3D::OBB(_) => ColliderResult::NotImplemented,
                Collider3D::Sphere(_) => ColliderResult::NotImplemented,
                Collider3D::Ray(ray) => match sphere_with_ray_collision(
                    ray.origin,
                    ray.direction,
                    sphere.radius,
                    sphere.center + if parent_instance.is_none() {
                        Vec3::ZERO
                    } else {
                        parent_instance.unwrap().pos()
                    }
                ) {
                    Some(dist) => ColliderResult::Collision(dist),
                    None => ColliderResult::NoCollision,
                },
            },
            Collider3D::Ray(ray) => match other {
                Collider3D::OBB(obb) => match oriented_bounding_box_with_ray(
                    ray.origin,
                    ray.direction,
                    obb.aabb_min,
                    obb.aabb_max,
                    if parent_instance.is_none() {
                        Instance {
                            ..Default::default()
                        }.to_raw().unwrap().model
                    } else {
                        parent_instance.unwrap().to_raw().unwrap().model
                    },
                ) {
                    Some(dist) => ColliderResult::Collision(dist),
                    None => ColliderResult::NoCollision,
                },
                Collider3D::Sphere(sphere) => match sphere_with_ray_collision(
                    ray.origin,
                    ray.direction,
                    sphere.radius,
                    sphere.center + if parent_instance.is_none() {
                        Vec3::ZERO
                    } else {
                        parent_instance.unwrap().pos()
                    }
                ) {
                    Some(dist) => ColliderResult::Collision(dist),
                    None => ColliderResult::NoCollision,
                },
                Collider3D::Ray(_) => ColliderResult::NotImplemented,
            },
        }
    }
}
pub enum ColliderResult {
    NotImplemented,
    NoCollision,
    Collision(f32),
}
#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}
#[derive(Copy, Clone)]
pub struct OBB {
    pub aabb_min: Vec3,
    pub aabb_max: Vec3,
}
impl OBB {
    pub fn new(x_len: f32, y_len: f32, z_len: f32) -> Self {
        let x = x_len / 2.;
        let y = y_len / 2.;
        let z = z_len / 2.;
        Self {
            aabb_min: Vec3::new(-x, -y, -z),
            aabb_max: Vec3::new(x, y, z),
        }
    }
    pub fn check_collision_with_ray(&self, ray: Ray, instance: &Instance) -> Option<f32> {
        if !instance.enabled {
            return None;
        }
        let model_matrix = instance.to_raw().unwrap().model;
        oriented_bounding_box_with_ray(
            ray.origin,
            ray.direction,
            self.aabb_min,
            self.aabb_max,
            model_matrix,
        )
    }
}
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
