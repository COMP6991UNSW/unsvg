/// The unsvg crate provides a very simple interface for drawing images.
///
/// See below for an example:
///
/// ```rust
/// use unsvg::{Image, COLORS};
///
/// fn main() -> Result<(), String> {
///     let mut img: Image = Image::new(200, 200);
///     let second_point = img.draw_simple_line(10.0, 10.0, 120, 100.0, COLORS[1])?;
///     let third_point = img.draw_simple_line(second_point.0, second_point.1, 240, 100.0, COLORS[2])?;
///     let _ = img.draw_simple_line(third_point.0, third_point.1, 0, 100.0, COLORS[3])?;
///
///     img.save_svg("path_to.svg")?;
///
///     Ok(())
/// }
/// ```

use resvg::{usvg, tiny_skia};
use std::rc::Rc;
use resvg::usvg::{NodeExt, Transform, TreeWriting, XmlOptions, Stroke};

pub use resvg::usvg::Color;


/// This contains 16 simple colors which users can select from.
/// These correspond to the 16 colors available in the original Logo language.
pub static COLORS: [Color; 16] = [
    /// Black
    Color {red: 0, green: 0, blue: 0},
    /// Blue
    Color {red: 0, green: 0, blue: 255},
    /// Green
    Color {red: 0, green: 255, blue: 0},
    /// Cyan
    Color {red: 0, green: 255, blue: 255},
    /// Red
    Color {red: 255, green: 0, blue: 0},
    /// Magenta
    Color {red: 255, green: 0, blue: 255},
    /// Yellow
    Color {red: 255, green: 255, blue: 0},
    /// White
    Color {red: 255, green: 255, blue: 255},
    /// Brown
    Color {red: 165, green: 42, blue: 42},
    /// Tan
    Color {red: 210, green: 180, blue: 140},
    /// Forest
    Color {red: 34, green: 139, blue: 34},
    /// Aqua
    Color {red: 127, green: 255, blue: 212},
    /// Salmon
    Color {red: 250, green: 128, blue: 114},
    /// Purple
    Color {red: 128, green: 0, blue: 128},
    /// Orange
    Color {red: 255, green: 165, blue: 0},
    /// Grey
    Color {red: 128, green: 128, blue: 128},
];

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

        let mut fill = usvg::Fill::from_paint(usvg::Paint::Color(usvg::Color::black()));
        let mut path = usvg::Path::new(Rc::from(tiny_skia::PathBuilder::from_rect(
            size.to_non_zero_rect(0.0, 0.0).to_rect()
        )));
        path.fill = Some(fill);


        tree.root.append_kind(usvg::NodeKind::Path(path));

        Image {
            width,
            height,
            tree
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
    pub fn draw_simple_line(&mut self, x: f32, y: f32, direction: i32, length: f32, color: Color) -> Result<(f32, f32), String> {
        let x = quantize(x);
        let y = quantize(y);
        let mut paint = usvg::Paint::Color(color);

        let mut path = tiny_skia::PathBuilder::new();
        path.move_to(x, y);

        // directions start at 0 degrees being straight up, and go clockwise
        // we need to add 90 degrees to make 0 degrees straight right.
        let direction_rad = ((direction as f32) - 90.0).to_radians();

        let end_x = quantize(x + (direction_rad.cos() * length as f32));
        let end_y = quantize(y + (direction_rad.sin() * length as f32));
        path.line_to(end_x, end_y);

        let mut path = usvg::Path::new(path.finish().ok_or("Could not draw line".to_string())?.into());
        let mut stroke = usvg::Stroke::default();
        stroke.paint = paint;
        path.stroke = Some(stroke);

        self.tree.root.append_kind(usvg::NodeKind::Path(path));

        Ok((end_x, end_y))
    }

}
