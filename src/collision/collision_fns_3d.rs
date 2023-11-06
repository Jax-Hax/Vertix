use std::f32::INFINITY;

use glam::Vec3;

pub fn oriented_bounding_box_with_ray(
    ray_origin: Vec3,    // Ray origin, in world space
    ray_direction: Vec3, // Ray direction (NOT target position!), in world space. Must be normalize()'d.
    aabb_min: Vec3,      // Minimum X,Y,Z coords of the mesh when not transformed at all.
    aabb_max: Vec3, // Maximum X,Y,Z coords. Often aabb_min*-1 if your mesh is centered, but it's not always the case.
    model_matrix: [[f32; 4]; 4],
) -> Option<f32> /*intersection distance */ {
    let mut t_min = 0.0; //largest near intersection found
    let mut t_max = INFINITY; //smallest far interaction found
    const THRESHOLD: f32 = 0.0000000001;
    let obb_worldspace = Vec3::new(model_matrix[3][0], model_matrix[3][1], model_matrix[3][2]); //3rd x,y, and z
    let delta = obb_worldspace - ray_origin;

    // Test intersection with the 2 planes perpendicular to the OBB's X axis
    let x_axis = Vec3::new(model_matrix[0][0], model_matrix[0][1], model_matrix[0][2]);
    let e = x_axis.dot(delta);
    let f = ray_direction.dot(x_axis);
    if f.abs() > THRESHOLD {
        //standard case
        let mut t1 = (e + aabb_min.x) / f;
        let mut t2 = (e + aabb_max.x) / f;
        // t1 and t2 now contain distances betwen ray origin and ray-plane intersections
        if t1 > t2 {
            // if wrong order
            let w = t1;
            t1 = t2;
            t2 = w; // swap t1 and t2
        }
        // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
        if t2 < t_max {
            t_max = t2;
        }
        // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
        if t1 > t_min {
            t_min = t1;
        }
        // If "far" is closer than "near", then there is NO intersection.
        if t_max < t_min {
            return None;
        }
    } else {
        // Rare case : the ray is almost parallel to the planes, so they don't have any "intersection"
        if -e + aabb_min.x > 0.0 || -e + aabb_max.x < 0.0 {
            return None;
        }
    }
    //Test intersection with the 2 planes perpendicular to the OBB's Y axis
    let y_axis = Vec3::new(model_matrix[1][0], model_matrix[1][1], model_matrix[1][2]);
    let e = y_axis.dot(delta);
    let f = ray_direction.dot(y_axis);
    if f.abs() > THRESHOLD {
        let mut t1 = (e + aabb_min.y) / f;
        let mut t2 = (e + aabb_max.y) / f;
        if t1 > t2 {
            // if wrong order
            let w = t1;
            t1 = t2;
            t2 = w; // swap t1 and t2
        }
        // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
        if t2 < t_max {
            t_max = t2;
        }
        // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
        if t1 > t_min {
            t_min = t1;
        }
        if t_max < t_min {
            return None;
        }
    } else {
        if -e + aabb_min.y > 0.0 || -e + aabb_max.y < 0.0 {
            return None;
        }
    }
    // Test intersection with the 2 planes perpendicular to the OBB's Z axis
    let z_axis = Vec3::new(model_matrix[2][0], model_matrix[2][1], model_matrix[2][2]);
    let e = z_axis.dot(delta);
    let f = ray_direction.dot(z_axis);
    if f.abs() > THRESHOLD {
        let mut t1 = (e + aabb_min.y) / f;
        let mut t2 = (e + aabb_max.y) / f;
        if t1 > t2 {
            // if wrong order
            let w = t1;
            t1 = t2;
            t2 = w; // swap t1 and t2
        }
        // tMax is the nearest "far" intersection (amongst the X,Y and Z planes pairs)
        if t2 < t_max {
            t_max = t2;
        }
        // tMin is the farthest "near" intersection (amongst the X,Y and Z planes pairs)
        if t1 > t_min {
            t_min = t1;
        }
        if t_max < t_min {
            return None;
        }
    } else {
        if -e + aabb_min.y > 0.0 || -e + aabb_max.y < 0.0 {
            return None;
        }
    }
    Some(t_min)
}
pub fn sphere_with_ray_collision(
    ray_origin: Vec3,
    ray_direction: Vec3,
    sphere_radius: f32,
    sphere_center: Vec3,
) -> bool {
    let delta = ray_origin - sphere_center;

    let b = delta.dot(ray_direction);
    let c = delta.dot(delta) - sphere_radius * sphere_radius;
    let h = b * b - c;
    if h < 0.0 {
        return false;
    } // no intersection
    true
}
