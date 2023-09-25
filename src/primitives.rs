pub fn square(p1: Vec2, p2: Vec2) {
    let vertices = vec![
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [0.4, 0.4],
        }, // A
        Vertex {
            position: [-0.5, 1., 0.0],
            tex_coords: [0.5, 0.5],
        }, // B
        Vertex {
            position: [1., 0., 0.0],
            tex_coords: [0.6, 0.6],
        }, // C
        Vertex {
            position: [1., 1., 0.0],
            tex_coords: [0.7, 0.7],
        }, // D
    ];

    let indices = vec![2,1,0, 1, 2, 3];
}