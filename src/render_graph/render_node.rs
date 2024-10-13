use crate::RenderState;

pub trait RenderNode {
    fn inputs(&self, render_state: &RenderState);
    fn outputs(&self, render_state: &RenderState);
    fn run(&self, render_state: &RenderState);
}
