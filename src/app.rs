use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}
};
mod renderer;
use renderer::State;

#[derive(Default)]
pub struct Application <'window_state> {
    window: Option<Arc<Window>>,
    state: Option<State<'window_state>>
}

impl ApplicationHandler for Application <'_>{ 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
        .with_title("Simulator");
        let window = Arc::new(
            event_loop
            .create_window(window_attributes).unwrap()
        );
        self.window = Some(window.clone());

        let state = State::new(window.clone());
        self.state = Some(state);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit()
                },
                WindowEvent::Resized(physical_size) => {
                    if let (Some(state), Some(window)) = 
                    (self.state.as_mut(), self.window.as_ref())
                    {
                        state.resize(physical_size);
                        window.request_redraw();
                    }
                },
                WindowEvent::RedrawRequested => {
                    if let Some(state) = self.state.as_mut() {
                        state.draw();
                    }
                },
                _ => {},
            }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        
    }

}
