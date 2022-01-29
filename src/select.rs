// pub extern crate crossterm;
// use crossterm::event::KeyCode::{self, Down, Up};

use crossterm::event::{
    read, Event, KeyCode,
    KeyCode::{Down, Up},
    KeyEvent, KeyModifiers,
};
use std::{fmt::Display, io::Write};

use crate::{line::Line, SelectDialogKey};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub mod junk {}
/// Struct to create a select dialog and get the users chosen item
///
/// The input is retrieved over an endless loop. When the user presses enter,
/// the loop stops and the chosen item is returned.
///
/// # Example
///
/// Create the dialog with default settings
///
/// ```
/// use cli_select::Select;
///
/// let items = vec!["item1", "item2", "item3"];
/// let selected_item = Select::new(&items).start();
/// ```
///
/// Customize dialog before starting
///
/// ```
/// use cli_select::{Select, KeyCode};
///
/// let items = vec!["item1", "item2", "item3"];
/// let selected_item = Select::new(&items)
///     .add_up_key(KeyCode::Char('j'))
///     .pointer('◉')
///     .not_selected_pointer('○')
///     .underline_selected_item()
///     .start();
/// ```
pub struct Select<'a, I>
where
    I: ToString + Display,
    // W: std::io::Write, // F: Fn(SelectDialogKey, &I),
{
    items: &'a Vec<I>,
    lines: Vec<Line>,
    selected_item: usize,
    pointer: char,
    not_selected_pointer: Option<char>,
    default_up: KeyCode,
    default_down: KeyCode,
    up_keys: Vec<KeyCode>,
    down_keys: Vec<KeyCode>,
    pub selection_changed: Option<Box<dyn Fn(SelectDialogKey, &I)>>,
    move_selected_item_forward: bool,
    underline_selected_item: bool,
    longest_item_len: usize,
    item_count: usize,
    // logger: Logger<W>,
}
///Logger Struct for Testing. Not used yet.
#[allow(dead_code)]
struct Logger<W: Write>(W);
#[allow(dead_code)]
impl<W> Logger<W>
where
    W: Write,
{
    pub fn log(&mut self, msg: &str) -> Result<usize, std::io::Error> {
        Ok(self.0.write(msg.as_bytes())?)
    }
}

impl<'a, I> Select<'a, I>
where
    I: ToString + Display + core::fmt::Debug,
    // W: std::io::Write,
    // F: Fn(SelectDialogKey, &I),
{
    pub fn new(items: &'a Vec<I>) -> Select<'a, I> {
        Select {
            items,
            pointer: '>',
            selected_item: 0,
            default_up: Up,
            default_down: Down,
            selection_changed: None,
            not_selected_pointer: None,
            move_selected_item_forward: false,
            underline_selected_item: false,
            up_keys: vec![],
            down_keys: vec![],
            lines: vec![],
            longest_item_len: 0,
            item_count: 0,
            // writer: std::io::stdout() as W,
        }
    }
    /// Builds the lines and store them for later usage. item_count and longest_item_len is initialized.
    fn build_lines(&mut self) {
        let mut lines: Vec<Line> = vec![];
        let mut item_count: usize = 0;
        for item in self.items {
            let mut line = Line::new(item.to_string(), self.pointer);

            if let Some(pointer) = self.not_selected_pointer {
                line.not_selected_pointer(pointer);
            }

            if line.len() > self.longest_item_len {
                self.longest_item_len = line.len()
            }
            lines.push(line);
            item_count += 1;
        }
        self.lines = lines;
        self.item_count = item_count;
    }
    fn print_lines(&mut self) {
        self.lines.iter_mut().for_each(|line| line.default());

        self.lines[self.selected_item].select();

        if self.underline_selected_item {
            self.lines[self.selected_item].underline();
        }
        if self.move_selected_item_forward {
            self.lines[self.selected_item].space_from_pointer(1);
        }

        self.lines.iter().for_each(|line| println!("{}", line))
    }

    fn erase_printed_items(&self) {
        self.move_n_lines_up(self.item_count + 1);

        for line in &self.lines {
            println!("{}", " ".repeat(line.len()));
        }
        self.move_n_lines_up(self.item_count + 1);
    }
    fn move_n_lines_up(&self, n: usize) {
        println!("[33[{}A", n);
    }

    fn move_up(&mut self) {
        if self.selected_item == 0 {
            return;
        };
        self.selected_item -= 1;
        self.erase_printed_items();
        self.print_lines();
    }
    fn move_down(&mut self) {
        if self.selected_item == self.items.len() - 1 {
            return;
        }

        self.selected_item += 1;
        self.erase_printed_items();
        self.print_lines();
    }
    fn call_event_handler_if_supplied(&self, key: SelectDialogKey) {
        if let Some(event_handler) = self.selection_changed.as_ref() {
            let current_item = &self.items.to_owned()[self.selected_item];
            event_handler(key, current_item);
        }
    }
    pub fn start(&mut self) -> &I {
        self.build_lines();
        self.print_lines();

        self.up_keys.push(self.default_up);
        self.down_keys.push(self.default_down);

        loop {
            let event = read().unwrap();

            if event
                == Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                })
            {
                break;
            }
            if self.event_contains_key(event, &self.up_keys) {
                self.move_up();
                self.call_event_handler_if_supplied(SelectDialogKey::UpKey);
                continue;
            } else if self.event_contains_key(event, &self.down_keys) {
                self.move_down();
                self.call_event_handler_if_supplied(SelectDialogKey::DownKey);
                continue;
            }
        }
        &self.items.to_owned()[self.selected_item]
    }
    fn event_contains_key(&self, event: Event, keys: &Vec<KeyCode>) -> bool {
        for key in keys.iter() {
            if event
                == Event::Key(KeyEvent {
                    code: key.clone(),
                    modifiers: KeyModifiers::NONE,
                })
            {
                return true;
            }
        }
        false
    }
    /// Set a custom pointer to show in the select dialog
    pub fn pointer(&mut self, pointer: char) -> &mut Self {
        self.pointer = pointer;
        self
    }
    pub fn set_up_key(&mut self, key: KeyCode) -> &mut Self {
        self.default_up = key;
        self
    }
    pub fn set_down_key(&mut self, key: KeyCode) -> &mut Self {
        self.default_down = key;
        self
    }
    pub fn not_selected_pointer(&mut self, pointer: char) -> &mut Self {
        self.not_selected_pointer = Some(pointer);
        self
    }
    pub fn move_selected_item_forward(&mut self) -> &mut Self {
        self.move_selected_item_forward = true;
        self
    }
    pub fn underline_selected_item(&mut self) -> &mut Self {
        self.underline_selected_item = true;
        self
    }
    pub fn add_up_key(&mut self, key: KeyCode) -> &mut Self {
        self.panic_if_key_is_enter(key);
        self.up_keys.push(key);
        self
    }
    pub fn add_down_key(&mut self, key: KeyCode) -> &mut Self {
        self.panic_if_key_is_enter(key);
        self.down_keys.push(key);
        self
    }
    fn panic_if_key_is_enter(&self, key: KeyCode) {
        if key == KeyCode::Enter {
            panic!("Enter key is not supported as up/down key")
        }
    }
}
