pub struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    scale: cgmath::Vector3<f32>,
}
impl Instance {
    pub fn new(position: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>) -> Self {
        Self {
            position,
            rotation,
            scale: cgmath::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn with_scale(mut self, scale: cgmath::Vector3<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let scale_inv = cgmath::Matrix3::new(
            1.0 / self.scale.x,
            0.0,
            0.0,
            0.0,
            1.0 / self.scale.y,
            0.0,
            0.0,
            0.0,
            1.0 / self.scale.z,
        );
        InstanceRaw {
            model: ((cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
                * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z))
            .into(),

            normal: (cgmath::Matrix3::from(self.rotation) * scale_inv).into(),
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
