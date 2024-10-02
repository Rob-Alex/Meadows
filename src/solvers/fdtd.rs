use crate::physics::mesh::SimulationMesh;

pub struct FDTD {
    mesh: SimulationMesh,
    pub simulation_time: f64,
}

impl FDTD {
    pub fn new() -> Self {
        let mesh = SimulationMesh::new(10, 10);
        let simulation_time = 0.0;
        Self {
            mesh,
            simulation_time,
        }
    }
}
