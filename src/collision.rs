use glam::Vec2;
use winit::dpi::PhysicalPosition;

pub struct Box2D {
    x_max: f32,
    x_min: f32,
    y_max: f32,
    y_min: f32,
}
impl Box2D {
    pub fn new(p1: Vec2, p2: Vec2) -> Self{
        Box2D {
            x_max: if p1.x > p2.x {p1.x} else {p2.x},
            x_min: if p1.x < p2.x {p1.x} else {p2.x},
            y_max: if p1.y > p2.y {p1.y} else {p2.y},
            y_min: if p1.y < p2.y {p1.y} else {p2.y}
        }
    }
    pub fn check_collision(&self, pos: &PhysicalPosition<f32>) -> bool {
        let x = pos.x;
        let y = pos.y;
        println!("x: {}, {}", self.x_max , self.x_min);
        println!("y: {}, {}", self.y_max, self.y_min);
        println!("coords: {}, {}", x, y);
        if x < self.x_max && x > self.x_min && y < self.y_max && y > self.y_min {
            println!("it collided");
            return true;
        }
        return false;
    }
}