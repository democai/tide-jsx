use crate::html_escaping::escape_html;
use crate::Render;
use std::fmt::{Result, Write};

impl Render for String {
    fn render_into(&self, writer: &mut String) -> Result {
        escape_html(&self, writer)
    }
}

impl Render for &str {
    fn render_into(&self, writer: &mut String) -> Result {
        escape_html(self, writer)
    }
}

impl Render for std::borrow::Cow<'_, str> {
    fn render_into(&self, writer: &mut String) -> Result {
        escape_html(&self, writer)
    }
}

/// A raw (unencoded) html string
#[derive(Debug, Clone)]
pub struct Raw<'s>(&'s str);

impl<'s> From<&'s str> for Raw<'s> {
    fn from(s: &'s str) -> Self {
        Raw(s)
    }
}

/// A raw (unencoded) html string
impl<'s> Render for Raw<'s> {
    fn render_into(&self, writer: &mut String) -> Result {
        write!(writer, "{}", self.0)
    }
}

/// Creates a raw (unencoded) html string
#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        tide_jsx::Raw::from($text)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_html() {
        use pretty_assertions::assert_eq;
        let rendered = "<Hello />".render();
        assert_eq!(rendered, "&lt;Hello /&gt;");
    }

    #[test]
    fn allows_raw_text() {
        use pretty_assertions::assert_eq;
        let rendered = Raw::from("<Hello />").render();
        assert_eq!(rendered, "<Hello />");
    }
}
