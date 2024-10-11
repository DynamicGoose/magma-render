use render_pass::RenderPass;

pub mod render_pass;

pub struct RenderGraph {
    passes: Vec<RenderPass>,
}
