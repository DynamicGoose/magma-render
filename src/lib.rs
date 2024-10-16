use magma_ecs::World;
use wgpu_context::WgpuContext;

pub mod render_graph;
pub mod wgpu_context;

#[derive(Default)]
pub struct RenderState {
    pub render_world: World,
    wgpu_context: WgpuContext,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            render_world: World::new(),
            wgpu_context: WgpuContext::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_render_state() {
        let render_state = RenderState::new();
        render_state.render_world.add_resource(10_u32);
        let resources = render_state.render_world.resources_read();
        assert_eq!(10_u32, *resources.get_ref::<u32>().unwrap())
    }
}
