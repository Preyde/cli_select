mod line;
mod select;

use core::fmt::Display;
pub use crossterm::event::KeyCode;
use crossterm::{
    self,
    event::{
        read, Event,
        KeyCode::{Down, Up},
        KeyEvent, KeyModifiers,
    },
};
use line::Line;

pub use select::Select;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Debug, Eq, PartialEq)]
pub enum SelectDialogKey {
    UpKey,
    DownKey,
}

pub type SelectionChange<T> = Box<dyn Fn(SelectDialogKey, &T)>;
