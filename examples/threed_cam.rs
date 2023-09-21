use vertix::camera::CamController;
fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
    let dt = dt.as_secs_f32();

    // Move forward/backward and left/right
    let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
    let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
    camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
    camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

    // Move in/out (aka. "zoom")
    // Note: this isn't an actual zoom. The camera's position
    // changes when zooming. I've added this to make it easier
    // to get closer to an object you want to focus on.
    let (pitch_sin, pitch_cos) = camera.pitch.sin_cos();
    let scrollward =
    Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
    camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
    self.scroll = 0.0;

    // Move up/down. Since we don't use roll, we can just
    // modify the y coordinate directly.
    camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

    // Rotate
    camera.yaw += self.rotate_horizontal * self.sensitivity * dt;
    camera.pitch += -self.rotate_vertical * self.sensitivity * dt;

    // If process_mouse isn't called every frame, these values
    // will not get set to zero, and the camera will rotate
    // when moving in a non cardinal direction.
    self.rotate_horizontal = 0.0;
    self.rotate_vertical = 0.0;

    // Keep the camera's angle from going too high/low.
    if camera.pitch < -SAFE_FRAC_PI_2 {
        camera.pitch = -SAFE_FRAC_PI_2;
    } else if camera.pitch > SAFE_FRAC_PI_2 {
        camera.pitch = SAFE_FRAC_PI_2;
    }
}