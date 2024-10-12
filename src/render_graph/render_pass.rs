use super::{color_attachment::ColorAttachment, depth_stencil_attachment::DepthStencilAttachment};

pub struct RenderPass<'a> {
    pub label: Option<&'a str>,
    pub color_attachments: &'a [&'a ColorAttachment],
    pub depth_stencil_attachment: &'a DepthStencilAttachment,
}
