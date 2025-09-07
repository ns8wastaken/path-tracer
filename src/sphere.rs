#[repr(C)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Self { center: [x, y, z], radius }
    }
}
