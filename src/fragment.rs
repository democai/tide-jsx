use crate::Render;
use std::fmt::Result;

#[derive(Debug, Clone)]
pub struct Fragment<T: Render + Clone> {
    pub children: T,
}

impl<T: Render + Clone> Render for Fragment<T> {
    fn render_into(&self, writer: &mut String) -> Result {
        self.children.render_into(writer)
    }
}
