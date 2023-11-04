use glam::Vec2;

use crate::prelude::Vertex;

pub fn rect(p1: Vec2, p2: Vec2) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex::new([p1.x,p1.y, 0.0], [0.0,1.0]),
        Vertex::new([p1.x, p2.y, 0.0],[0.0,0.0]),
        Vertex::new([p2.x, p1.y, 0.0],[1.0,1.0]),
        Vertex::new([p2.x,p2.y, 0.0],[1.0, 0.0]),
    ];

    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}
pub fn rect_with_tex_coords(p1: Vec2, p2: Vec2, tex_1: Vec2, tex_2: Vec2) -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex::new([p1.x,p1.y, 0.0], [tex_1.x,tex_2.y]),
        Vertex::new([p1.x, p2.y, 0.0],[tex_1.x,tex_1.y]),
        Vertex::new([p2.x, p1.y, 0.0],[tex_2.x,tex_2.y]),
        Vertex::new([p2.x,p2.y, 0.0],[tex_2.x, tex_1.y]),
    ];

    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}