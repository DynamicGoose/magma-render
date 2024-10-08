use magma_ecs::World;

#[derive(Default)]
pub struct RenderState {
    pub render_world: World,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            render_world: World::new(),
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
