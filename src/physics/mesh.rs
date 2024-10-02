#[derive(Debug, Clone)]
pub struct Cell {
    cex: f32,
    cey: f32,
    hx: f32,
    hy: f32,
    chx: f32,
    dz: f32,
    ez: f32,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            cex: 0.0,
            cey: 0.0,
            hx: 0.0,
            hy: 0.0,
            chx: 0.0,
            dz: 0.0,
            ez: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct SimulationMesh {
    cells: Vec<Cell>,
}

impl SimulationMesh {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let cell = Cell::new();
        let cells = vec![cell; size_x * size_y];
        Self { cells }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct RenderMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
extern crate rand;
use rand::Rng; // For generating random numbers
impl RenderMesh {
    pub fn test() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    position: [-1.0, 1.0, 0.0], // Top-left corner
                    color: [0.1, 0.0, 0.5],
                },
                Vertex {
                    position: [-1.0, -1.0, 0.0], // Bottom-left corner
                    color: [0.5, 1.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 0.0], // Bottom-right corner
                    color: [0.1, 0.4, 0.5],
                }, // C
                Vertex {
                    position: [1.0, 1.0, 0.0], // Top-right corner
                    color: [0.5, 0.2, 0.5],
                }, // D
                Vertex {
                    position: [0.1, 0.0, 0.0], // Center (optional, used to create a triangle fan effect)
                    color: [0.5, 0.0, 0.5],
                }, // E
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }
    pub fn new_dense_grid(grid_size: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut rng = rand::thread_rng(); // Random number generator

        let step = 2.0 / (grid_size as f32 - 1.0); // Step size to move from -1 to 1 across the grid

        // Create the grid of vertices
        for y in 0..grid_size {
            for x in 0..grid_size {
                // Normalize x and y to the range [-1.0, 1.0]
                let position = [
                    -1.0 + x as f32 * step, // X position from -1.0 to 1.0
                    -1.0 + y as f32 * step, // Y position from -1.0 to 1.0
                    0.0,                    // Z position (for 2D grid)
                ];

                // Generate random color
                let color = [
                    rng.gen::<f32>(), // Random red
                    rng.gen::<f32>(), // Random green
                    rng.gen::<f32>(), // Random blue
                ];

                vertices.push(Vertex { position, color });

                // Create indices for two triangles (forming a quad) if not on the last row or column
                if x < grid_size - 1 && y < grid_size - 1 {
                    let top_left = (y * grid_size + x) as u32;
                    let top_right = top_left + 1;
                    let bottom_left = ((y + 1) * grid_size + x) as u32;
                    let bottom_right = bottom_left + 1;

                    // First triangle (top left, bottom left, top right)
                    indices.push(top_left);
                    indices.push(bottom_left);
                    indices.push(top_right);

                    // Second triangle (top right, bottom left, bottom right)
                    indices.push(top_right);
                    indices.push(bottom_left);
                    indices.push(bottom_right);
                }
            }
        }

        Self { vertices, indices }
    }
}
