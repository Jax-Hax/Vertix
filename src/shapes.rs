use glam::Vec2;

use crate::prelude::Vertex;

pub fn rect(height: f32, width: f32) -> (Vec<Vertex>, Vec<u32>) {
    let y = height/2.;
    let x = width/2.;
    let vertices = vec![
        Vertex::new([-x,-y, 0.0], [0.0,1.0]),
        Vertex::new([-x,y, 0.0],[0.0,0.0]),
        Vertex::new([x,-y, 0.0],[1.0,1.0]),
        Vertex::new([x,y, 0.0],[1.0, 0.0]),
    ];
    
    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}
pub fn rect_with_tex_coords(height: f32, width: f32, tex_1: Vec2, tex_2: Vec2) -> (Vec<Vertex>, Vec<u32>) {
    let y = height/2.;
    let x = width/2.;
    let vertices = vec![
        Vertex::new([-x,-y, 0.0], [tex_1.x,tex_2.y]),
        Vertex::new([-x,y, 0.0],[tex_1.x,tex_1.y]),
        Vertex::new([x,-y, 0.0],[tex_2.x,tex_2.y]),
        Vertex::new([x,y, 0.0],[tex_2.x, tex_1.y]),
    ];

    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}