use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::offset_of!(Vertex, color) as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
    pub count: u32,
}

impl VertexBuffer {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
            count: vertices.len() as u32,
        }
    }
}

pub struct IndexBuffer {
    pub buffer: wgpu::Buffer,
    pub count: u32,
}

impl IndexBuffer {
    pub fn new(device: &wgpu::Device, indices: &[u16]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            buffer,
            count: indices.len() as u32,
        }
    }
}
