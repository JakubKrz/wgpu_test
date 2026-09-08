pub struct Instance {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

impl Instance {
    pub fn new(position: glam::Vec3, rotation: glam::Quat) -> Self {
        Self {
            position,
            rotation,
            scale: glam::Vec3::ONE,
        }
    }

    pub fn with_scale(mut self, scale: glam::Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let scale_inv = glam::Mat3::from_diagonal(glam::Vec3::new(
            1.0 / self.scale.x,
            1.0 / self.scale.y,
            1.0 / self.scale.z,
        ));
        InstanceRaw {
            model: (glam::Mat4::from_translation(self.position)
                * glam::Mat4::from_quat(self.rotation)
                * glam::Mat4::from_scale(self.scale))
            .to_cols_array_2d(),

            normal: (glam::Mat3::from_quat(self.rotation) * scale_inv).to_cols_array_2d(), // lub odpowiednik tablicowy zależnie od definicji InstanceRaw
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}

const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
    //model
    5 => Float32x4,
    6 => Float32x4,
    7 => Float32x4,
    8 => Float32x4,
    //normal
    9 => Float32x3,
    10 => Float32x3,
    11 => Float32x3,
];

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBS,
        }
    }
}
