# `unsvg`

`unsvg` is a Rust crate that provides a very simple SVG (Scalable Vector
Graphics) rendering library. It is built upon the solid foundation of the
[`resvg`](https://github.com/RazrFalcon/resvg) crate and offers developers an
easy to use system for generating simple images.

Unsvg was developed for COMP6991: Solving Modern Programming Problems with Rust,
a course at the University of New South Wales.


## Usage

To use `unsvg` in your Rust project, simply add it as a dependency in your
`Cargo.toml`:

```toml
[dependencies]
unsvg = "0.1"
```

Then, import it into your code:

```rust
use unsvg::{Image, COLORS};

fn main() -> Result<(), String> {
    let mut img: Image = Image::new(200, 200);
    let second_point = img.draw_simple_line(10.0, 10.0, 120, 100.0, COLORS[1])?;
    let third_point = img.draw_simple_line(second_point.0, second_point.1, 240, 100.0, COLORS[2])?;
    let _ = img.draw_simple_line(third_point.0, third_point.1, 0, 100.0, COLORS[3])?;

    img.save_svg("path_to.svg")?;

    Ok(())
}
```

For detailed usage instructions and examples, please refer to the documentation.

## Documentation

Explore the full capabilities of `unsvg` by visiting our
[documentation](https://docs.rs/unsvg/latest/unsvg/). You'll find comprehensive
guides, API references, and example code to help you get started quickly.

## Contributions

We welcome contributions from the open-source community. If you find issues,
have feature requests, or want to contribute code, please visit our GitHub
repository [here](https://github.com/COMP6991UNSW/unsvg) and get involved.

## License

`unsvg` is Copyright © University of New South Wales.
