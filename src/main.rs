use physics::material::Material;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
pub mod gpu {
    pub mod rendering {
        pub mod renderer;
    }
}
mod physics {
    pub mod electromagnetics;
    pub mod geometry;
    pub mod material;
    pub mod mesh;
}

mod solvers {
    pub mod fdtd;
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = app::Application::default();
    event_loop.run_app(&mut app)
}
