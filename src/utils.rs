mod rl {
    pub use raylib::prelude::*;
}

pub fn vector3_angle(v1: rl::Vector3, v2: rl::Vector3) -> f32 {
    let cross = v1.cross(v2);
    let len = cross.length();
    let dot = v1.dot(v2);
    len.atan2(dot)
}

pub fn vector3_rotate_by_axis_angle(v: rl::Vector3, axis: rl::Vector3, angle: f32) -> rl::Vector3 {
    let mut result = v;

    let half_angle = angle / 2.0;
    let a = half_angle.sin();
    let w = axis.normalized().scale_by(a);
    let a = half_angle.cos();

    let wv = w
        .cross(v)
        .scale_by(2.0 * a);

    let wwv = w
        .cross(wv)
        .scale_by(2.0);

    result += wv;
    result += wwv;

    result
}
