use std::fmt;

/// The basic 16 colors you can use for ANSI.
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn int_value(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
            Color::BrightBlack => 60,
            Color::BrightRed => 61,
            Color::BrightGreen => 62,
            Color::BrightYellow => 63,
            Color::BrightBlue => 64,
            Color::BrightMagenta => 65,
            Color::BrightCyan => 66,
            Color::BrightWhite => 67,
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Blue => write!(f, "Blue"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::White => write!(f, "White"),
            Color::BrightBlack => write!(f, "Bright Black"),
            Color::BrightRed => write!(f, "Bright Red"),
            Color::BrightGreen => write!(f, "Bright Green"),
            Color::BrightYellow => write!(f, "Bright Yellow"),
            Color::BrightBlue => write!(f, "Bright Blue"),
            Color::BrightMagenta => write!(f, "Bright Magenta"),
            Color::BrightCyan => write!(f, "Bright Cyan"),
            Color::BrightWhite => write!(f, "Bright White"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Struct that indicates the position at which a ANSI code should be located in the colored
/// string output.
struct CodeMarker {
    index: usize,
    code: u8,
}

/// Colored string builder.
pub struct ColoredString {
    raw: String,
    code_markers: Vec<CodeMarker>,
}

const ANSI_ESCAPE_START: &str = "\x1b[";
const ANSI_ESCAPE_END: &str = "m";
const ANSI_ESCAPE_LEN: usize = ANSI_ESCAPE_START.len() +
                               1 + // Color on one character.
                               ANSI_ESCAPE_END.len();

impl Default for ColoredString {
    fn default() -> Self {
        Self::new()
    }
}

impl ColoredString {
    /// Create a new colored string.
    pub fn new() -> Self {
        ColoredString {
            raw: String::new(),
            code_markers: Vec::new(),
        }
    }

    /// Create a new colored string with a specific capacity.
    pub fn with_capacity(capacity: usize) -> ColoredString {
        ColoredString {
            raw: String::with_capacity(capacity),
            code_markers: Vec::new(),
        }
    }

    fn push_code(&mut self, code: u8) {
        let color_marker = CodeMarker {
            index: self.raw.len(),
            code,
        };

        self.code_markers.push(color_marker)
    }

    fn push_color_code(&mut self, color: &Color, base: u8) {
        self.push_code(base + color.int_value())
    }

    fn push_format_code(&mut self, code: u8, enable: bool) {
        let mut code_ = code;

        if !enable {
            code_ += 20
        }

        self.push_code(code_)
    }

    /// Reset the formatting to the default on from this stage of the string.
    pub fn reset(&mut self) {
        self.push_code(0)
    }

    /// Set the foreground color from this stage of the string.
    pub fn set_fg(&mut self, color: &Color) {
        self.push_color_code(color, 30);
    }

    /// Set the background color from this stage of the string.
    pub fn set_bg(&mut self, color: &Color) {
        self.push_color_code(color, 40);
    }

    /// Enable or disable the text style to faint one from this stage of the string.
    pub fn set_faint(&mut self, enable: bool) {
        self.push_format_code(1, enable)
    }

    /// Enable or disable the text style to bold one from this stage of the string.
    pub fn set_bold(&mut self, enable: bool) {
        self.push_format_code(2, enable)
    }

    /// Enable or disable the text style to italic one from this stage of the string.
    pub fn set_italic(&mut self, enable: bool) {
        self.push_format_code(3, enable)
    }

    /// Enable or disable the text style to underline one from this stage of the string.
    pub fn set_underline(&mut self, enable: bool) {
        self.push_format_code(4, enable)
    }

    /// Enable or disable the text slow blinking from this stage of the string.
    pub fn set_slow_blink(&mut self, enable: bool) {
        self.push_format_code(5, enable)
    }

    /// Enable or disable the text fast blinking from this stage of the string.
    pub fn set_fast_blink(&mut self, enable: bool) {
        self.push_format_code(6, enable)
    }

    /// Push a character to the colored string.
    pub fn push(&mut self, ch: char) {
        self.raw.push(ch)
    }

    /// Push a string to the colored string.
    pub fn push_str(&mut self, string: &str) {
        self.raw.push_str(string)
    }

    /// Get the raw content of the string without colors or any formatting.
    #[inline]
    pub fn raw(&self) -> String {
        self.raw.clone()
    }

    /// Get the colored string. The colored output will always be so the colors
    /// are reset at the end of the string.
    pub fn colored(&self) -> String {
        if self.code_markers.is_empty() {
            return self.raw();
        }

        let mut index: usize = 0;
        let mut ret = String::with_capacity(
            self.raw.len() + ((self.code_markers.len() + 1) * ANSI_ESCAPE_LEN),
        );

        for color_marker in self.code_markers.iter() {
            if ret.is_empty() {
                ret += ANSI_ESCAPE_START;
            } else if index != color_marker.index {
                ret += ANSI_ESCAPE_END;
                ret += &self.raw[index..color_marker.index];
                ret += ANSI_ESCAPE_START;
            } else {
                ret.push(';')
            }

            ret += &color_marker.code.to_string();

            index = color_marker.index;
        }

        ret += ANSI_ESCAPE_END;
        ret += &self.raw[index..self.raw.len()];

        ret += ANSI_ESCAPE_START;
        ret.push('0');
        ret += ANSI_ESCAPE_END;

        ret
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colored())
    }
}
