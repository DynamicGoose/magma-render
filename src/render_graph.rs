use compute_pass::ComputePass;
use render_pass::RenderPass;

pub mod color_attachment;
pub mod compute_pass;
pub mod depth_stencil_attachment;
pub mod render_pass;

pub struct RenderGraph<'a> {
    render_passes: Vec<RenderPass<'a>>,
    compute_passes: Vec<ComputePass>,
}
