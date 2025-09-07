mod rl { pub use raylib::prelude::*; }

use crate::utils;

pub struct Camera {
    pub position: rl::Vector3,
    pub target: rl::Vector3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: rl::Vector3, target: rl::Vector3, fov: f32) -> Self {
        Self { position, target, fov }
    }

    pub fn get_forward(&self) -> rl::Vector3 {
        self.target - self.position
    }

    pub fn get_right(&self) -> rl::Vector3 {
        self.get_forward().cross(rl::Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn get_up(&self) -> rl::Vector3 {
        self.get_right().cross(self.get_forward())
    }

    fn move_forward(&mut self, dist: f32) {
        let d = self.get_forward()
            .normalized()
            .scale_by(dist);
        self.position += d;
        self.target += d;
    }

    fn move_up(&mut self, dist: f32) {
        let d = self.get_up().normalized().scale_by(dist);
        self.position += d;
        self.target += d;
    }

    fn move_right(&mut self, dist: f32) {
        let d = self.get_right().normalized().scale_by(dist);
        self.position += d;
        self.target += d;
    }

    // Left / Right
    fn yaw(&mut self, angle: f32, rotate_around_target: bool) {
        let target_pos = utils::vector3_rotate_by_axis_angle(self.get_forward(), self.get_up(), angle);

        if rotate_around_target {
            self.position = self.target - target_pos;
        } else {
            self.target = self.position + target_pos;
        }
    }

    // Up / Down
    fn pitch(&mut self, mut angle: f32, rotate_around_target: bool, rotate_up: bool) {
        let up = self.get_up().normalized();
        let target_pos = self.get_forward();

        // Clamp view up
        let max_angle_up = utils::vector3_angle(up, target_pos) - 0.001;
        if angle > max_angle_up {
            angle = max_angle_up;
        }

        // Clamp view down
        let max_angle_down = -utils::vector3_angle(-up, target_pos) + 0.001;
        if angle < max_angle_down {
            angle = max_angle_down;
        }

        // Rotate view vector around right axis
        let target_pos = utils::vector3_rotate_by_axis_angle(target_pos, self.get_right().normalized(), angle);

        if rotate_around_target {
            self.position = self.target - target_pos;
        } else {
            self.target = self.position + target_pos;
        }

        if rotate_up {
            // Rotate up direction around right axis
            // self.get_up() = utils::vector3_rotate_by_axis_angle(self.get_up(), self.get_right_normalized(), angle);
        }
    }

    pub fn update(&mut self, rl: &rl::RaylibHandle) {
        static CAMERA_MOVE_SPEED: f32             = 5.4; // Units per second
        static CAMERA_ROTATION_SPEED: f32         = 0.03; // Keyboard
        static CAMERA_PAN_SPEED: f32              = 0.2;
        static CAMERA_MOUSE_MOVE_SENSITIVITY: f32 = 0.0006;

        let mouse_position_delta = rl.get_mouse_delta();

        let rotate_around_target = false;
        let rotate_up = true;

        // Camera speeds based on frame time
        let frame_time = rl.get_frame_time();
        let camera_move_speed     = CAMERA_MOVE_SPEED     * frame_time;
        let camera_rotation_speed = CAMERA_ROTATION_SPEED * frame_time;
        let camera_pan_speed      = CAMERA_PAN_SPEED      * frame_time;

        // Camera movement
        if rl.is_mouse_button_down(rl::MouseButton::MOUSE_BUTTON_MIDDLE) {
            if mouse_position_delta.x > 0.0 { self.move_right(camera_pan_speed); }
            if mouse_position_delta.x < 0.0 { self.move_right(-camera_pan_speed); }
            if mouse_position_delta.y > 0.0 { self.move_up(-camera_pan_speed); }
            if mouse_position_delta.y < 0.0 { self.move_up(camera_pan_speed); }
        } else {
            self.yaw(-mouse_position_delta.x * CAMERA_MOUSE_MOVE_SENSITIVITY, rotate_around_target);
            self.pitch(-mouse_position_delta.y * CAMERA_MOUSE_MOVE_SENSITIVITY, rotate_around_target, rotate_up);
        }

        // Keyboard camera rotation
        if rl.is_key_down(rl::KeyboardKey::KEY_DOWN)  { self.pitch(-camera_rotation_speed, rotate_around_target, rotate_up); }
        if rl.is_key_down(rl::KeyboardKey::KEY_UP)    { self.pitch(camera_rotation_speed, rotate_around_target, rotate_up); }
        if rl.is_key_down(rl::KeyboardKey::KEY_RIGHT) { self.yaw(-camera_rotation_speed, rotate_around_target); }
        if rl.is_key_down(rl::KeyboardKey::KEY_LEFT)  { self.yaw(camera_rotation_speed, rotate_around_target); }

        // Keyboard support
        if rl.is_key_down(rl::KeyboardKey::KEY_W) { self.move_forward(camera_move_speed); }
        if rl.is_key_down(rl::KeyboardKey::KEY_S) { self.move_forward(-camera_move_speed); }
        if rl.is_key_down(rl::KeyboardKey::KEY_D) { self.move_right(camera_move_speed); }
        if rl.is_key_down(rl::KeyboardKey::KEY_A) { self.move_right(-camera_move_speed); }

        if rl.is_key_down(rl::KeyboardKey::KEY_SPACE) { self.move_up(camera_move_speed); }
        if rl.is_key_down(rl::KeyboardKey::KEY_LEFT_CONTROL) { self.move_up(-camera_move_speed); }
    }
}

impl Into<rl::ffi::Camera3D> for &Camera {
    fn into(self) -> rl::ffi::Camera3D {
        rl::ffi::Camera3D {
            position: self.position.into(),
            target: self.target.into(),
            up: self.get_up().into(),
            fovy: self.fov,
            projection: rl::ffi::CameraProjection::CAMERA_PERSPECTIVE as i32,
        }
    }
}
