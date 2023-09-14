macro_rules! color_tests {
    ($($ty:tt)*) => {$(
        #[allow(non_snake_case)]
        pub mod $ty {
            use colost::Color;
            use colost::ColoredString;

            #[test]
            fn print_fg() {
                let mut cs = ColoredString::default();
                let color = Color::$ty;

                cs.set_fg(&color);

                cs.push_str("Foreground ");
                cs.push_str(&color.to_string());

                print!("{}", cs.colored());
            }

            #[test]
            fn print_bg() {
                let mut cs = ColoredString::default();
                let color = Color::$ty;

                cs.set_fg(&color);

                cs.push_str("Background ");
                cs.push_str(&color.to_string());

                print!("{}", cs.colored());
            }
        }
    )*};
}

color_tests!(
    Black
    Red
    Green
    Yellow
    Blue
    Magenta
    Cyan
    White
    BrightBlack
    BrightRed
    BrightGreen
    BrightYellow
    BrightBlue
    BrightMagenta
    BrightCyan
    BrightWhite
);
