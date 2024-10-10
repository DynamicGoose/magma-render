use magma_render::{wgpu_context::WgpuContext, RenderState};
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::Window};

fn main() {
    env_logger::init();
    let mut app = TestApp {
        window: None,
        _render_state: RenderState::new(),
    };
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

struct TestApp {
    window: Option<Window>,
    _render_state: RenderState,
}

impl ApplicationHandler for TestApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let _wgpu_context = WgpuContext::new(self.window.as_ref().unwrap());
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.exit();
    }
}
