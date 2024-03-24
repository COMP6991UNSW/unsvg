use resvg::usvg::{NodeExt, TreeWriting, XmlOptions};
/// The unsvg crate provides a very simple interface for drawing images.
///
/// See below for an example:
///
/// ```rust
/// use unsvg::{Image, COLORS, Error};
///
/// fn main() -> Result<(), Error> {
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
use resvg::{tiny_skia, usvg};
use std::rc::Rc;

pub use resvg::usvg::Color;

/// A type encapsulating some error encountered within `unsvg`.
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

/// All fallible functions provided by `unsvg` return our custom
/// [`Error`] type, so we redefine `Result` with fixed error.
type Result<T> = core::result::Result<T, Error>;

/// This contains 16 simple colors which users can select from.
/// These correspond to the 16 colors available in the original Logo language.
/// The colors are:
///  - Black
///  - Blue
///  - Cyan
///  - Green
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

/// Enum of the 16 colors available in the original Logo language.
/// Implements `Into<usize>` to convert the enum to a usize, for indexing into the COLORS array.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorIndex {
    Black,
    Blue,
    Cyan,
    Green,
    Red,
    Magenta,
    Yellow,
    White,
    Brown,
    Tan,
    Forest,
    Aqua,
    Salmon,
    Purple,
    Orange,
    Grey,
}

impl From<ColorIndex> for Color {
    fn from(index: ColorIndex) -> Self {
        COLORS[Into::<usize>::into(index)]
    }
}

impl From<ColorIndex> for usize {
    /// Converts a `ColorIndex` to a usize.
    /// This usize value corresponds with the index of the color in the COLORS array.
    fn from(index: ColorIndex) -> Self {
        match index {
            ColorIndex::Black => 0,
            ColorIndex::Blue => 1,
            ColorIndex::Cyan => 2,
            ColorIndex::Green => 3,
            ColorIndex::Red => 4,
            ColorIndex::Magenta => 5,
            ColorIndex::Yellow => 6,
            ColorIndex::White => 7,
            ColorIndex::Brown => 8,
            ColorIndex::Tan => 9,
            ColorIndex::Forest => 10,
            ColorIndex::Aqua => 11,
            ColorIndex::Salmon => 12,
            ColorIndex::Purple => 13,
            ColorIndex::Orange => 14,
            ColorIndex::Grey => 15,
        }
    }
}

impl TryFrom<usize> for ColorIndex {
    type Error = Error;

    /// Tries to convert from color to a `ColorIndex` variant.
    /// Errors when the the provided index is too large for the COLORS array (index > 15).
    fn try_from(index: usize) -> Result<Self> {
        match index {
            0 => Ok(ColorIndex::Black),
            1 => Ok(ColorIndex::Blue),
            2 => Ok(ColorIndex::Cyan),
            3 => Ok(ColorIndex::Green),
            4 => Ok(ColorIndex::Red),
            5 => Ok(ColorIndex::Magenta),
            6 => Ok(ColorIndex::Yellow),
            7 => Ok(ColorIndex::White),
            8 => Ok(ColorIndex::Brown),
            9 => Ok(ColorIndex::Tan),
            10 => Ok(ColorIndex::Forest),
            11 => Ok(ColorIndex::Aqua),
            12 => Ok(ColorIndex::Salmon),
            13 => Ok(ColorIndex::Purple),
            14 => Ok(ColorIndex::Orange),
            15 => Ok(ColorIndex::Grey),
            _ => Err(Error("Invalid color index".to_string())),
        }
    }
}

impl TryFrom<u64> for ColorIndex {
    type Error = Error;

    /// Tries to convert from color to a `ColorIndex` variant.
    /// Errors when the the provided index is too large for the COLORS array (index > 15).
    fn try_from(index: u64) -> Result<Self> {
        match index {
            0 => Ok(ColorIndex::Black),
            1 => Ok(ColorIndex::Blue),
            2 => Ok(ColorIndex::Cyan),
            3 => Ok(ColorIndex::Green),
            4 => Ok(ColorIndex::Red),
            5 => Ok(ColorIndex::Magenta),
            6 => Ok(ColorIndex::Yellow),
            7 => Ok(ColorIndex::White),
            8 => Ok(ColorIndex::Brown),
            9 => Ok(ColorIndex::Tan),
            10 => Ok(ColorIndex::Forest),
            11 => Ok(ColorIndex::Aqua),
            12 => Ok(ColorIndex::Salmon),
            13 => Ok(ColorIndex::Purple),
            14 => Ok(ColorIndex::Orange),
            15 => Ok(ColorIndex::Grey),
            _ => Err(Error("Invalid color index".to_string())),
        }
    }
}

impl TryFrom<Color> for ColorIndex {
    type Error = Error;

    /// Tries to convert from a `Color` to a `ColorIndex` variant.
    /// Errors when the the provided Color is not in the COLORS array.
    fn try_from(color: Color) -> Result<Self> {
        COLORS
            .iter()
            .position(|&c| c == color)
            .ok_or(Error("Color not in COLORS array".to_string()))
            .and_then(|i| ColorIndex::try_from(i))
    }
}

/// Tells you where a line will end, given a starting point, direction, and length.
/// This is used by `draw_simple_line` to get the end point of a line.
pub fn get_end_coordinates(x: f32, y: f32, direction: i32, length: f32) -> (f32, f32) {
    let x = quantize(x);
    let y = quantize(y);

    // directions start at 0 degrees being straight up, and go clockwise
    // we need to add 90 degrees to make 0 degrees straight right.
    let direction_rad = ((direction as f32) - 90.0).to_radians();

    let end_x = quantize(x + (direction_rad.cos() * length as f32));
    let end_y = quantize(y + (direction_rad.sin() * length as f32));

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
    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let rtree = resvg::Tree::from_usvg(&self.tree);
        let pixmap_size = rtree.size.to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
        pixmap.save_png(path).map_err(|e| Error(e.to_string()))
    }

    /// Save the image to a file.
    ///
    /// ```rs
    /// let image = Image::new(100, 100);
    /// image.save_svg("image.svg");
    /// ```
    pub fn save_svg<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        std::fs::write(path, self.tree.to_string(&XmlOptions::default()))
            .map_err(|e| Error(e.to_string()))
    }

    /// Draw a line on the image, taking a starting point, direction, length, and color.
    /// We return the end point of the line as a tuple of (x, y).
    pub fn draw_simple_line(
        &mut self,
        x: f32,
        y: f32,
        direction: i32,
        length: f32,
        color: Color,
    ) -> Result<(f32, f32)> {
        let x = quantize(x);
        let y = quantize(y);
        let (end_x, end_y) = get_end_coordinates(x, y, direction, length);

        let paint = usvg::Paint::Color(color);
        let mut path = tiny_skia::PathBuilder::new();
        path.move_to(x, y);
        path.line_to(end_x, end_y);

        let mut path = usvg::Path::new(
            path.finish()
                .ok_or(Error("Could not draw line".to_string()))?
                .into(),
        );
        let mut stroke = usvg::Stroke::default();
        stroke.paint = paint;
        path.stroke = Some(stroke);

        self.tree.root.append_kind(usvg::NodeKind::Path(path));

        Ok((end_x, end_y))
    }
}
