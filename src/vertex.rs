use wgpu::*;
use crate::universe::Universe;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ]
        }
    }
}

pub fn create_grid_vertices(universe: &Universe, cell_size: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let padding = 0.02;

    for row in 0..universe.rows {
        for col in 0..universe.cols {
            let idx = (row * universe.cols + col) as usize;
            
            let color = if universe.cells[idx] {
                [0.2, 0.8, 0.2] // Alive: Green
            } else {
                [0.1, 0.1, 0.1] // Dead: Dark Grey
            };

            let x_offset = (col as f32 * (cell_size + padding)) - 0.6;
            let y_offset = (row as f32 * (cell_size + padding)) - 0.6;

            vertices.extend_from_slice(&[
                Vertex { position: [x_offset, y_offset + cell_size], color },
                Vertex { position: [x_offset, y_offset], color },
                Vertex { position: [x_offset + cell_size, y_offset], color },

                Vertex { position: [x_offset, y_offset + cell_size], color },
                Vertex { position: [x_offset + cell_size, y_offset], color },
                Vertex { position: [x_offset + cell_size, y_offset + cell_size], color },
            ]);
        }
    }
    vertices
}
