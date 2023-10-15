use glam::Vec2;
use winit::dpi::PhysicalPosition;

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Box2D {
    x_max: f32,
    x_min: f32,
    y_max: f32,
    y_min: f32,
    enabled: bool
}
impl Box2D {
    pub fn new(p1: Vec2, p2: Vec2) -> Self{
        Box2D {
            x_max: if p1.x > p2.x {p1.x} else {p2.x},
            x_min: if p1.x < p2.x {p1.x} else {p2.x},
            y_max: if p1.y > p2.y {p1.y} else {p2.y},
            y_min: if p1.y < p2.y {p1.y} else {p2.y},
            enabled: true
        }
    }
    pub fn check_collision(&self, pos: &PhysicalPosition<f32>) -> bool {
        let x = pos.x;
        let y = pos.y;
        if self.enabled{
            if x < self.x_max && x > self.x_min && y < self.y_max && y > self.y_min {
                println!("collision");
                return true;
            }
        }
        return false;
    }
}
#[derive(Component)]
pub struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
    enabled: bool,
}
impl Circle {
    pub fn new(center: Vec2, radius: f32, enabled: bool) -> Self{
        Circle {
            center_x: center.x,
            center_y: center.y,
            radius,
            enabled
        }
    }
    pub fn check_collision(&self, pos: &PhysicalPosition<f32>) -> bool {
        let x = pos.x;
        let y = pos.y;
        //find distance betweenn two points
        let dist_x = (x - self.center_x).powi(2);
        let dist_y = (y - self.center_y).powi(2);
        let dist = (dist_x + dist_y).sqrt();
        if self.enabled && dist < self.radius {
            println!("collision");
            return true;
        }
        return false;
    }
}