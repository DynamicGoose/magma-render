use magma_render::{wgpu_context::WgpuContext, RenderState};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    env_logger::init();
    let render_state = RenderState::new();
    let event_loop = EventLoop::new().unwrap();
    let window = event_loop
        .create_window(Window::default_attributes())
        .unwrap();
    let wgpu_context = WgpuContext::new(&render_state.instance, &window);
}
