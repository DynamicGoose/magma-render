use magma_render::render_graph::{render_node::RenderNode, RenderGraph};

#[test]
fn add_render_node() {
    let mut render_graph = RenderGraph::new();
    render_graph.add_render_node(TestRenderNode);
}

struct TestRenderNode;

impl RenderNode for TestRenderNode {
    fn inputs(&self, render_state: &magma_render::RenderState) {}

    fn outputs(&self, render_state: &magma_render::RenderState) {}

    fn run(&self, render_state: &magma_render::RenderState) {}
}
