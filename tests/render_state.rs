use magma_render::{wgpu_context::WgpuContext, RenderState};
use winit::{event_loop::EventLoop, window::Window};

fn main() {
    env_logger::init();
    let _render_state = RenderState::new();
    let event_loop = EventLoop::new().unwrap();
    println!("1");
    #[allow(deprecated)]
    let window = event_loop
        .create_window(Window::default_attributes())
        .unwrap();
    println!("2");
    let _wgpu_context = WgpuContext::new(&window);
    println!("3");
}
