use std::fmt::{Display, Formatter};

pub struct Line {
    text: String,
    is_selected: bool,
    pointer: char,
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
        }
    }
    /// Show the pointer for this line
    pub fn select(&mut self) {
        self.is_selected = true;
    }
    pub fn underline(&mut self) {
        self.text = format!("[4m{}[0m", self.text);
    }
    pub fn space_from_pointer(&mut self, space: usize) {
        self.space = space;
    }
    /// set all changes made after creation back to default
    pub fn default(&mut self) {
        self.is_selected = false;
        self.space = 1;
        self.underline = false;
    }
    /// ascii code to underline
    fn underline_text(&self, text: &str) -> String {
        format!("[4m{}[0m", text)
    }
}

impl Display for Line {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let text = if self.underline {
            Some(self.underline_text(&self.text))
        } else {
            None
        };

        println!(
            "{}{}{}",
            self.pointer,
            " ".repeat(self.space),
            text.as_ref().unwrap_or(&self.text)
        );
        Ok(())
    }
}
