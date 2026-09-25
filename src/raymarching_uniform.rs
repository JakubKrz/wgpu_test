#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RaymarchUniform {
    pub aspect_ratio: f32,
    pub fov_scale: f32,
    pub _padding: [f32; 2],
}
