use core::fmt;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn selected_line_printed_with_pointer() {
        let mut line = Line::new(String::from("test"), '>');
        line.select();
        assert_eq!(line.draw(), "> test")
    }
    #[test]
    fn unselected_line_printed_without_pointer() {
        let line = Line::new(String::from("test"), '>');
        assert_eq!(line.draw(), "  test")
    }
}
#[derive(Debug)]
pub struct Line {
    text: String,
    is_selected: bool,
    pointer: char,
    not_selected_pointer: char,
    space: usize,
    underline: bool,
}

impl Line {
    /// Creates a new Line with given text and pointer
    pub fn new(text: String, pointer: char) -> Self {
        Line {
            text,
            is_selected: false,
            pointer,
            space: 1,
            underline: false,
            not_selected_pointer: ' ',
        }
    }
    /// Show the pointer for this line
    pub fn select(&mut self) {
        self.is_selected = true;
    }
    pub fn not_selected_pointer(&mut self, pointer: char) {
        self.not_selected_pointer = pointer;
    }
    pub fn underline(&mut self) {
        self.underline = true;
    }
    pub fn space_from_pointer(&mut self, space: usize) {
        self.space = space;
    }
    /// set all changes back to default that were made after creation
    pub fn default(&mut self) {
        self.is_selected = false;
        self.space = 1;
        self.underline = false;
    }
    /// ascii code to underline
    fn underline_text(&self, text: &str) -> String {
        format!("[4m{}[0m", text)
    }
    pub fn len(&self) -> usize {
        self.text.chars().count() + self.space + 1
    }
    fn draw(&self) -> String {
        let text = if self.underline {
            Some(self.underline_text(&self.text))
        } else {
            None
        };
        let pointer = if self.is_selected {
            self.pointer
        } else {
            self.not_selected_pointer
        };

        format!(
            "{}{}{}",
            pointer,
            " ".repeat(self.space),
            text.as_ref().unwrap_or(&self.text),
        )
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.draw())?;
        Ok(())
    }
}
