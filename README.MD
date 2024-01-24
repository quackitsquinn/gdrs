# gdrs: A Geometry Dash level modification tool.

gdrs is a tool that allows you to modify Geometry Dash levels from within rust.

The end goal is a library that can streamline the process of creating Geometry Dash levels programmatically.

The ideal usage is as follows: (note that this is not yet implemented)

```rust
use gdrs::Level;

fn main() {
    let mut level = Level::with_file("CCLocalLevels.dat").level("example level").unwrap();
    level.get_objects().iter_mut().for_each(|obj| {
        obj.x += 10.0;
    });
    level.save("CCLocalLevels.dat").unwrap();
}
```

or, if you want gdrs to find the file for you:

```rust
use gdrs::Level;

fn main() {
    let mut level = Level::from_game("example level").unwrap();
    level.get_objects().iter_mut().for_each(|obj| {
        obj.x += 10.0;
    });
    level.save_to_game().unwrap();
}
```
