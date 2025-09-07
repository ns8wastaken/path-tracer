mod rl {
    pub use raylib::prelude::*;
}

use crate::utils;

pub struct Camera {
    pub position: rl::Vector3,
    pub target: rl::Vector3,
    pub forward: rl::Vector3,
    pub up: rl::Vector3,
    pub right: rl::Vector3,
}

impl Camera {
    pub fn new(position: rl::Vector3, target: rl::Vector3) -> Self {
        let position = position.normalized();
        let target = target.normalized();
        let forward = (target - position).normalized();
        let right = position.cross(forward).normalized();
        let up = forward.cross(right).normalized();

        Self { position, target, forward, up, right }
    }

    fn move_forward(&mut self, dist: f32) {
        let d = self.forward.scale_by(dist);
        self.position += d;
        self.target += d;
    }

    fn move_up(&mut self, dist: f32) {
        let d = self.up.scale_by(dist);
        self.position += d;
        self.target += d;
    }

    fn move_right(&mut self, dist: f32) {
        let d = self.right.scale_by(dist);
        self.position += d;
        self.target += d;
    }

    // Left / Right
    fn yaw(&mut self, angle: f32, rotate_around_target: bool) {
        self.forward = utils::vector3_rotate_by_axis_angle(self.forward, self.up, angle);

        if rotate_around_target {
            self.position = self.target - self.forward;
        } else {
            self.target = self.position + self.forward;
        }
    }

    // Up / Down
    fn pitch(&mut self, mut angle: f32, rotate_around_target: bool, rotate_up: bool) {
        // Clamp view up
        let max_angle_up = utils::vector3_angle(self.up, self.forward) - 0.001;
        if angle > max_angle_up {
            angle = max_angle_up;
        }

        // Clamp view down
        let max_angle_down = -utils::vector3_angle(-self.up, self.forward) + 0.001;
        if angle < max_angle_down {
            angle = max_angle_down;
        }

        // Rotate view vector around right axis
        self.forward = utils::vector3_rotate_by_axis_angle(self.forward, self.right, angle);

        if rotate_around_target {
            self.position = self.target - self.forward;
        } else {
            self.target = self.position + self.forward;
        }

        if rotate_up {
            // Rotate up direction around right axis
            self.up = utils::vector3_rotate_by_axis_angle(self.up, self.right, angle);
        }
    }

    fn orthonormalize(&mut self) {
        self.forward = (self.target - self.position).normalized();
        self.right = self.up.cross(self.forward).normalized();
        self.up = self.forward.cross(self.right).normalized();
    }

    pub fn update_camera(&mut self, rl: &rl::RaylibHandle) {
        static CAMERA_MOVE_SPEED: f32             = 5.4; // Units per second
        static CAMERA_ROTATION_SPEED: f32         = 0.03; // Keyboard
        static CAMERA_PAN_SPEED: f32              = 0.2;
        static CAMERA_MOUSE_MOVE_SENSITIVITY: f32 = 0.001;

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

        self.orthonormalize();
    }
}
