use crate::html_escaping::escape_html;
use crate::Render;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Result, Write};

type Attributes<'a> = Option<HashMap<&'a str, Cow<'a, str>>>;

/// Simple HTML element tag
#[derive(Debug, Clone)]
pub struct SimpleElement<'a, T: Render + Clone> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Attributes<'a>,
    pub contents: Option<T>,
}

fn write_attributes<W: Write>(maybe_attributes: &Attributes<'_>, writer: &mut W) -> Result {
    match maybe_attributes {
        None => Ok(()),
        Some(attributes) => {
            let mut attributes = attributes.clone();
            for (key, value) in attributes.drain() {
                write!(writer, " {}=\"", key)?;
                escape_html(&value, writer)?;
                write!(writer, "\"")?;
            }
            Ok(())
        }
    }
}

impl<T: Render + Clone> Render for SimpleElement<'_, T> {
    fn render_into(&self, writer: &mut String) -> Result {
        match &self.contents {
            None => {
                if self.is_void_tag() {
                    write!(writer, "<{}", self.tag_name)?;
                    write_attributes(&self.attributes, writer)?;
                    write!(writer, " />")
                } else {
                    write!(writer, "<{}", self.tag_name)?;
                    write_attributes(&self.attributes, writer)?;
                    write!(writer, "></{}>", self.tag_name)
                }
            }
            Some(renderable) => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(&self.attributes, writer)?;
                write!(writer, ">")?;
                renderable.render_into(writer)?;
                write!(writer, "</{}>", self.tag_name)
            }
        }
    }
}

impl<T: Render + Clone> SimpleElement<'_, T> {
    fn is_void_tag(&self) -> bool {
        matches!(
            self.tag_name,
            "area"
                | "base"
                | "br"
                | "col"
                | "command"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }
}
