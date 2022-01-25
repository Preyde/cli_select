mod line;
mod select;

use core::fmt::Display;
use crossterm::{
    self,
    event::{
        read, Event,
        KeyCode::{self, Down, Up},
        KeyEvent, KeyModifiers,
    },
};
use line::Line;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Debug, Eq, PartialEq)]
enum SelectDialogKey {
    UpKey,
    DownKey,
}

type SelectionChange<T> = Box<dyn Fn(SelectDialogKey, &T)>;
