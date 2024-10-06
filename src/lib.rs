//! The `unsvg` crate provides a very simple interface for drawing images.
//!
//! See below for an example:
//!
//! ```rust
//! use unsvg::{Image, COLORS};
//!
//! fn main() -> Result<(), String> {
//!     let mut img: Image = Image::new(200, 200);
//!     let (x1, y1) = (10, 10);
//!     // Second point's x and y coordinates.
//!     let (x2, y2) = img.draw_simple_line(x1, y1, 120, 100, COLORS[1])?;
//!     // Third point's x and y coordinates.
//!     let (x3, y3) = img.draw_simple_line(x2, y2, 240, 100, COLORS[2])?;
//!     let _ = img.draw_simple_line(x3, y3, 0, 100, COLORS[3])?;
//!
//!     img.save_svg("path_to.svg")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Note that `unsvg`'s underlying SVG library uses floats to represent x and y
//! coordinates. `unsvg`, on the other hand, expects signed integer coordinate
//! values. This design decision was made to ensure consistent, deterministic
//! behaviour for all coordinate inputs, which is not a given when using floats
//! due to float imprecision.

use num_traits::cast;
use resvg::usvg::{NodeExt, TreeWriting, XmlOptions};
use resvg::{tiny_skia, usvg};
use std::rc::Rc;

pub use resvg::usvg::Color;

/// This contains 16 simple colors which users can select from.
/// These correspond to the 16 colors available in the original Logo language.
/// The colors are:
///  - Black
///  - Blue
///  - Green
///  - Cyan
///  - Red
///  - Magenta
///  - Yellow
///  - White
///  - Brown
///  - Tan
///  - Forest
///  - Aqua
///  - Salmon
///  - Purple
///  - Orange
///  - Grey
pub static COLORS: [Color; 16] = [
    Color {
        red: 0,
        green: 0,
        blue: 0,
    },
    Color {
        red: 0,
        green: 0,
        blue: 255,
    },
    Color {
        red: 0,
        green: 255,
        blue: 255,
    },
    Color {
        red: 0,
        green: 255,
        blue: 0,
    },
    Color {
        red: 255,
        green: 0,
        blue: 0,
    },
    Color {
        red: 255,
        green: 0,
        blue: 255,
    },
    Color {
        red: 255,
        green: 255,
        blue: 0,
    },
    Color {
        red: 255,
        green: 255,
        blue: 255,
    },
    Color {
        red: 165,
        green: 42,
        blue: 42,
    },
    Color {
        red: 210,
        green: 180,
        blue: 140,
    },
    Color {
        red: 34,
        green: 139,
        blue: 34,
    },
    Color {
        red: 127,
        green: 255,
        blue: 212,
    },
    Color {
        red: 250,
        green: 128,
        blue: 114,
    },
    Color {
        red: 128,
        green: 0,
        blue: 128,
    },
    Color {
        red: 255,
        green: 165,
        blue: 0,
    },
    Color {
        red: 128,
        green: 128,
        blue: 128,
    },
];

fn u32_to_f32(num: u32) -> f32 {
    cast(num).unwrap_or_else(|| panic!("failed to convert u32 '{num}' to f32"))
}

fn f32_to_u32(num: f32) -> u32 {
    cast(num.round()).unwrap_or_else(|| panic!("failed to convert f32 '{num}' to u32"))
}

fn i32_to_f32(num: i32) -> f32 {
    cast(num).unwrap_or_else(|| panic!("failed to convert i32 '{num}' to f32"))
}

fn f32_to_i32(num: f32) -> i32 {
    cast(num.round()).unwrap_or_else(|| panic!("failed to convert f32 '{num}' to i32"))
}

/// Normalize a direction values in degrees to within [0, 360).
fn normalize_direction(direction: i32) -> i32 {
    let normalized = direction % 360;
    if normalized < 0 {
        normalized + 360
    } else {
        normalized
    }
}

/// Tells you where a line will end, given a starting point, direction, and length.
/// This is used by `draw_simple_line` to get the end point of a line.
pub fn get_end_coordinates(x: i32, y: i32, direction: i32, length: i32) -> (i32, i32) {
    let x = i32_to_f32(x);
    let y = i32_to_f32(y);
    let length = i32_to_f32(length);

    let (end_x, end_y) = get_end_coordinates_precise(x, y, direction, length);

    let end_x = f32_to_i32(end_x);
    let end_y = f32_to_i32(end_y);

    (end_x, end_y)
}

fn get_end_coordinates_precise(x: f32, y: f32, direction: i32, length: f32) -> (f32, f32) {
    let x = quantize(x);
    let y = quantize(y);
    let direction = normalize_direction(direction);

    // directions start at 0 degrees being straight up, and go clockwise.
    // we need to add 90 degrees to make 0 degrees straight right.
    let direction_rad = ((direction as f32) - 90.0).to_radians();

    let end_x = quantize(x + (direction_rad.cos() * length));
    let end_y = quantize(y + (direction_rad.sin() * length));

    (end_x, end_y)
}

/// This represents an image that's being constructed. Use the `new` function
/// to create one.
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    tree: usvg::Tree,
}

fn quantize(x: f32) -> f32 {
    (x * 256.0).round() / 256.0
}

impl Image {
    /// Creates an image.
    pub fn new(width: u32, height: u32) -> Image {
        let size = usvg::Size::from_wh(width as f32, height as f32).unwrap();
        let tree = usvg::Tree {
            size,
            view_box: usvg::ViewBox {
                rect: size.to_non_zero_rect(0.0, 0.0),
                aspect: usvg::AspectRatio::default(),
            },
            root: usvg::Node::new(usvg::NodeKind::Group(usvg::Group::default())),
        };

        let fill = usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::black()));
        let mut path = usvg::Path::new(Rc::from(tiny_skia::PathBuilder::from_rect(
            size.to_non_zero_rect(0.0, 0.0).to_rect(),
        )));
        path.fill = Some(fill);

        tree.root.append_kind(usvg::NodeKind::Path(path));

        Image {
            width,
            height,
            tree,
        }
    }

    /// Get the size of the image as a tuple of (width, height).
    ///
    /// ```rs
    /// let image = Image::new(100, 100);
    /// let (width, height) = image.get_dimensions();
    /// assert_eq!(width, 100);
    /// assert_eq!(height, 100);
    /// ```
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Save the image to a file.
    ///
    /// ```rs
    /// let image = Image::new(100, 100);
    /// image.save_png("image.png");
    /// ```
    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        let rtree = resvg::Tree::from_usvg(&self.tree);

        let pixmap_size = rtree.size.to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
        pixmap.save_png(path).map_err(|e| e.to_string())
    }

    /// Save the image to a file.
    ///
    /// ```rs
    /// let image = Image::new(100, 100);
    /// image.save_svg("image.svg");
    /// ```
    pub fn save_svg<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        std::fs::write(path, self.tree.to_string(&XmlOptions::default())).map_err(|e| e.to_string())
    }

    /// Draw a line on the image, taking a starting point, direction, length, and color.
    /// We return the end point of the line as a tuple of (x, y).
    pub fn draw_simple_line(
        &mut self,
        x: i32,
        y: i32,
        direction: i32,
        length: i32,
        color: Color,
    ) -> Result<(i32, i32), String> {
        let (end_x, end_y) = get_end_coordinates(x, y, direction, length);

        let paint = usvg::Paint::Color(color);
        let mut path = tiny_skia::PathBuilder::new();
        path.move_to(i32_to_f32(x), i32_to_f32(y));
        path.line_to(i32_to_f32(end_x), i32_to_f32(end_y));

        let mut path = usvg::Path::new(
            path.finish()
                .ok_or("Could not draw line".to_string())?
                .into(),
        );
        let mut stroke = usvg::Stroke::default();
        stroke.paint = paint;
        path.stroke = Some(stroke);

        self.tree.root.append_kind(usvg::NodeKind::Path(path));

        Ok((end_x, end_y))
    }
}
