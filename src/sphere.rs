mod rl {
    pub use raylib::prelude::*;
}

#[repr(C)]
pub struct Sphere {
    center: [f32; 3],
    radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, radius: f32) -> Self {
        Self { center: [x, y, z], radius }
    }
}

impl rl::ShaderV for Sphere {
    const UNIFORM_TYPE: rl::ShaderUniformDataType = rl::ShaderUniformDataType::SHADER_UNIFORM_VEC4;

    unsafe fn value(&self) -> *const std::os::raw::c_void {
        self.center.as_ptr() as *const std::os::raw::c_void
    }
}
