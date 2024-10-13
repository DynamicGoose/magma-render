use render_node::RenderNode;

pub mod render_node;

#[derive(Default)]
pub struct RenderGraph {
    render_nodes: Vec<Box<dyn RenderNode>>,
}

impl RenderGraph {
    pub fn new() -> Self {
        Self {
            render_nodes: vec![],
        }
    }
    pub fn add_render_node(&mut self, node: impl RenderNode + 'static) {
        self.render_nodes.push(Box::new(node));
    }
}
