use cli_utils::colors::{ColorString, Color};

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn coloring_changes_colorized() {
    let raw_string = "Colorized string";
    let color_string1 = ColorString::new(Color::Red, raw_string.to_string());
    let color_string2 = ColorString::new(Color::Blue, raw_string.to_string());

    assert_ne!(color_string1.colorized, color_string2.colorized);
}