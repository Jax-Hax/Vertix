pub fn oriented_bounding_box_with_ray(
    ray_origin: Vec3,    // Ray origin, in world space
    ray_direction: Vec3, // Ray direction (NOT target position!), in world space. Must be normalize()'d.
    aabb_min: Vec3,      // Minimum X,Y,Z coords of the mesh when not transformed at all.
    aabb_max: Vec3,      // Maximum X,Y,Z coords. Often aabb_min*-1 if your mesh is centered, but it's not always the case.
    model_matrix: [[f32; 4]; 4],
) -> (bool,f32) /*intersection distance */ {
    let mut t_min = 0.0; //largest near intersection found
    let mut t_max = 10000.0; //smallest far interaction found
    let obb_worldspace = Vec3::new(model_matrix[3][0],model_matrix[3][1],model_matrix[3][2]); //3rd x,y, and z
    let delta = obb_worldspace - ray_origin;
    //x axis
    let x_axis = Vec3::new(model_matrix[0][0],model_matrix[0][1],model_matrix[0][2]);
    let e = x_axis.dot(delta);
    let f = ray_direction.dot(x_axis);
    //dont do division if f is near 0
    let mut t1 = (e+aabb_min.x)/f;
    let mut t2 = (e+aabb_max.x)/f;
    if t1>t2 { // if wrong order
        let w=t1;
        t1=t2;
        t2=w; // swap t1 and t2
    }
    // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
    if t2 < t_max {t_max = t2;}
    // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
    if t1 > t_min {t_min = t1;}
    println!("x: {},{}, {}", t_max, t_min, t_max < t_min);
    if t_max < t_min {
        return (false,0.);
    }
    //y axis
    t_min = 0.0; //largest near intersection found
    t_max = 10000.0; //smallest far interaction found
    let y_axis = Vec3::new(model_matrix[1][0],model_matrix[1][1],model_matrix[1][2]);
    let e = y_axis.dot(delta);
    let f = ray_direction.dot(y_axis);
    //dont do division if f is near 0
    let mut t1 = (e+aabb_min.y)/f;
    let mut t2 = (e+aabb_max.y)/f;
    if t1>t2 { // if wrong order
        let w=t1;
        t1=t2;
        t2=w; // swap t1 and t2
    }
    // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
    if t2 < t_max {t_max = t2;}
    // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
    if t1 > t_min {t_min = t1;}
    println!("y: {},{}, {}", t_max, t_min, t_max < t_min);
    if t_max < t_min {
        return (false,0.);
    }
    //z axis
    t_min = 0.0; //largest near intersection found
    t_max = 10000.0; //smallest far interaction found
    let z_axis = Vec3::new(model_matrix[2][0],model_matrix[2][1],model_matrix[2][2]);
    let e = z_axis.dot(delta);
    let f = ray_direction.dot(z_axis);
    //dont do division if f is near 0
    let mut t1 = (e+aabb_min.z)/f;
    let mut t2 = (e+aabb_max.z)/f;
    if t1>t2 { // if wrong order
        let w=t1;
        t1=t2;
        t2=w; // swap t1 and t2
    }
    // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
    if t2 < t_max {t_max = t2;}
    // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
    if t1 > t_min {t_min = t1;}
    println!("z: {},{}, {}", t_max, t_min, t_max < t_min);
    if t_max < t_min {
        return (false,0.);
    }
    (true,t_min)
}
pub fn sphere_with_ray_collision(ray_origin: Vec3, ray_direction: Vec3, sphere_radius: f32, sphere_center: Vec3) -> bool {
    let delta = ray_origin - sphere_center;
    let b = delta.dot(ray_direction);
    let c = delta.dot(delta) - sphere_radius*sphere_radius;
    let h = b*b - c;
    if h<0.0 {return false;} // no intersection
    true
}