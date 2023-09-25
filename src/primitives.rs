use glam::Vec2;

use crate::prelude::Vertex;

pub fn rect(p1: Vec2, p2: Vec2) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex {
            position: [p1.x,p1.y, 0.0],
            tex_coords: [0.4, 0.4],
        }, // A
        Vertex {
            position: [p1.x, p2.y, 0.0],
            tex_coords: [0.5, 0.5],
        }, // B
        Vertex {
            position: [p2.x, p1.y, 0.0],
            tex_coords: [0.6, 0.6],
        }, // C
        Vertex {
            position: [p2.x,p2.y, 0.0],
            tex_coords: [0.7, 0.7],
        }, // D
    ];

    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}