// Color object
#[derive(Debug, Clone)]
enum Color {
    // color is either:
    Hex(String),     // hex string
    Rgb(u8, u8, u8), // red, green, blue tuple
}

// implement some methods
impl Color {
    // return hex string from color
    fn hex(&self) -> String {
        match self {
            Color::Hex(s) => s.clone(),
            Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
        }
    }

    // return RGB tuple from color
    fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Hex(s) => (
                u8::from_str_radix(&s[1..3], 16).unwrap(),
                u8::from_str_radix(&s[3..5], 16).unwrap(),
                u8::from_str_radix(&s[5..], 16).unwrap(),
            ),
            Color::Rgb(r, g, b) => (*r, *g, *b),
        }
    }
}

// implement a way to create hex color
impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('#') && value.len() == 7 {
            if value[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                Ok(Color::Hex(String::from(value).to_lowercase()))
            } else {
                Err(String::from("Invalid Hex"))
            }
        } else {
            Err(String::from("Invalid Hex"))
        }
    }
}

// implement a way to create RGB color
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::Rgb(r, g, b)
    }
}

fn main() {
    println!("Hello, world! ");
    println!("Hello, Andrii! ");
}

// create test to try if it works (run "cargo test")
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let col1 = Color::try_from("#FF0000").unwrap();
        let col2 = Color::from((255, 0, 0));
        assert_eq!(col1.hex(), "#ff0000");
        assert_eq!(col2.hex(), "#ff0000");
        assert_eq!(col1.rgb(), (255, 0, 0));
        assert_eq!(col2.rgb(), (255, 0, 0));
        assert!(Color::try_from("#FFG000").is_err());
    }
}
