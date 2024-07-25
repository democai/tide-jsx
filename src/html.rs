use crate::Render;
use std::fmt::{Result, Write};

#[derive(Debug, Clone)]
pub struct HTML5Doctype;

impl Render for HTML5Doctype {
    fn render_into(&self, writer: &mut String) -> Result {
        write!(writer, "<!DOCTYPE html>")
    }
}
