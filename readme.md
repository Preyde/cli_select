# cli_select

A crate build on top of crossterm to provide a simple customizable select dialog for the command line.

<br>

# Usage

Cargo.toml

```toml
[dependencies]
cli_select = "0.1.3"
```
<br>

## Basic

main.rs

```rust
use cli_select::Select;

fn main() {

    let items = vec!["item1", "item2", "item3"];

    let select = Select::new(&items);

    let selected_item = select.start();

    println!("You selected: {}", selected_item);
}
```

### Output

```
> item1
  item2
  item3
  You selected: item1
  ```

<br>

## Customized

main.rs

```rust

use cli_select::Select;

fn main() {

    let items = vec!["item1", "item2", "item3"];

    let select = Select::new(&items);

    let selected_item = select        
        .pointer('◉')
        .not_selected_pointer('🞅')
        .move_selected_item_forward()
        .start();

    println!("You selected: {}", selected_item);
}
```

### Output

```
◉  item1
🞅 item2
🞅 item3
You selected: item1
```
