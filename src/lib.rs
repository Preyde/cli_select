mod line;
mod select;

pub use crossterm::event::KeyCode;
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
