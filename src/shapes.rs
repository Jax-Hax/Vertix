use glam::{Vec2, Vec3};

use crate::prelude::Vertex;

pub fn rect(x_width: f32, y_height: f32) -> (Vec<Vertex>, Vec<u32>) {
    let y = y_height/2.;
    let x = x_width/2.;
    let vertices = vec![
        Vertex::new([-x,-y, 0.0], [0.0,1.0]),
        Vertex::new([-x,y, 0.0],[0.0,0.0]),
        Vertex::new([x,-y, 0.0],[1.0,1.0]),
        Vertex::new([x,y, 0.0],[1.0, 0.0]),
    ];
    
    let indices = vec![2,1,0, 1, 2, 3];
    (vertices,indices)
}
pub fn line_3d(p1: Vec3, p2: Vec3) -> (Vec<Vertex>, Vec<u32>){
    let vertices = vec![
        Vertex::new([p2.x,p2.y, p2.z], [0.0,1.0]),
        Vertex::new([p1.x+0.5,p1.y, p1.z],[0.0,0.0]),
        Vertex::new([p2.x+0.5,p2.y, p2.z],[1.0,1.0]),
        Vertex::new([p1.x,p1.y, p1.z],[1.0, 0.0]),
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
pub fn cube(x_width: f32, y_height: f32, z_length: f32) -> (Vec<Vertex>, Vec<u32>) {
    let x = x_width/2.;
    let y = y_height/2.;
    let z = z_length/2.;
    let indices = vec![
        //Top
        7, 6, 2,
        2, 3, 7,

        //Bottom
        0, 4, 5,
        5, 1, 0,

        //Left
        0, 2, 6,
        6, 4, 0,

        //Right
        7, 3, 1,
        1, 5, 7,

        //Front
        3, 2, 0,
        0, 1, 3,

        //Back
        4, 6, 7,
        7, 5, 4
    ];


    let vertices = vec![
        Vertex::new([-x,-y, z], [0.0,0.0]),
        Vertex::new([x,-y, z], [0.0,0.0]),
        Vertex::new([-x,y, z], [0.0,0.0]),
        Vertex::new([x,y, z], [0.0,0.0]),
        Vertex::new([-x,-y, -z], [0.0,0.0]),
        Vertex::new([x,-y, -z], [0.0,0.0]),
        Vertex::new([-x,y, -z], [0.0,0.0]),
        Vertex::new([x,y, -z], [0.0,0.0]),
    ];
    (vertices,indices)
}