use xcb::x;

pub trait TryIntoKeycode {
    fn try_into_keycode(self) -> Option<x::Keycode>;
}

impl TryIntoKeycode for &str {
    fn try_into_keycode(self) -> Option<x::Keycode> {
        match self {
            "q" => Some(24),
            "w" => Some(25),
            "e" => Some(26),
            "t" => Some(28),
            "y" => Some(29),
            "a" => Some(38),
            "s" => Some(39),
            "d" => Some(40),
            "f" => Some(41),
            "g" => Some(42),
            "z" => Some(52),
            "x" => Some(53),
            "c" => Some(54),
            "v" => Some(55),
            "b" => Some(56),
            _ => None,
        }
    }
}

pub trait TryIntoStr<'a> {
    fn try_into_str(self) -> Option<&'a str>;
}

impl<'a> TryIntoStr<'a> for xcb::x::Keycode {
    fn try_into_str(self) -> Option<&'a str> {
        match self {
            24 => Some("q"),
            25 => Some("w"),
            26 => Some("e"),
            28 => Some("t"),
            29 => Some("y"),
            38 => Some("a"),
            39 => Some("s"),
            40 => Some("d"),
            41 => Some("f"),
            42 => Some("g"),
            52 => Some("z"),
            53 => Some("x"),
            54 => Some("c"),
            55 => Some("v"),
            56 => Some("b"),
            _ => None,
        }
    }
}
