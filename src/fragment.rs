use crate::Render;
use std::fmt::{Result, Write};

#[derive(Debug, Clone)]
pub struct Fragment<T: Render + Clone> {
    pub children: T,
}

impl<T: Render + Clone> Render for Fragment<T> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        self.children.render_into(writer)
    }
}
