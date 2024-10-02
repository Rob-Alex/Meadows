use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, ModifiersState, PhysicalKey},
    window::{Window, WindowId},
};

use crate::gpu::rendering::renderer;
use crate::solvers::fdtd::FDTD;
use renderer::State;

#[derive(Default)]
pub struct Application<'window_state> {
    window: Option<Arc<Window>>,
    state: Option<State<'window_state>>,
    fdtd_solver: Option<FDTD>,
    simulation_time: f64,
}

impl Application<'_> {
    fn init_fdtd(&mut self) {
        let mut fdtd_solver = FDTD::new();
        self.fdtd_solver = Some(fdtd_solver);
        self.simulation_time = 0.0;
    }

    fn run_fdtd_simulation(&mut self, delta_time: f64) {
        if let Some(ref mut fdtd_solver) = self.fdtd_solver {
            fdtd_solver.simulation_time += delta_time;
            println!("{:}", fdtd_solver.simulation_time)
        }
    }
}

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Simulator");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let state = State::new(window.clone());
        self.state = Some(state);

        //fdtd init
        self.init_fdtd();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                if let (Some(state), Some(window)) = (self.state.as_mut(), self.window.as_ref()) {
                    state.resize(physical_size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_mut() {
                    window.request_redraw();
                    state.update();
                    state.draw();

                    //Run fdtd
                    let delta_time = 1.0 / 120.0; //assumes 120fps
                    self.run_fdtd_simulation(delta_time);
                }
            }
            WindowEvent::KeyboardInput { .. } => {
                // Dispatch actions only on press.
                if let Some(state) = self.state.as_mut() {
                    state.input(&event);
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}
