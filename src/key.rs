use strum::EnumString;
use xcb::x;

pub struct KeyTable {
    keycode_table: [Code; 256]
}
impl KeyTable {
    pub fn new() -> Self {
        Self { keycode_table: build_keycode_table(), }
    }
    pub fn code(&self, xcode: x::Keycode) -> Option<Code> {
        let xcode = xcode as usize;
        if xcode >= self.keycode_table.len() {
            eprintln!("Keycode 0x{:x} is out of bounds", xcode);
            // return Some(Code::Unknown);
            return None;
        }
        Some(self.keycode_table[xcode])
    }
    pub fn keycode(&self, letter: &str) -> Option<x::Keycode> {
        let code: Code = letter.parse().unwrap();
        self.keycode_table.iter()
            .position(|&c| c == code)
            .map(|c| c as x::Keycode)
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = format!("{:#?}", self);
        f.write_str(&text.to_lowercase())
    }
}


// Taken from the xbindings: https://github.com/rust-x-bindings/toy_xcb/blob/master/src/keyboard.rs
// 134 modified to LeftSuper
fn build_keycode_table() -> [Code; 256] {
    [
        // 0x00     0
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Escape,
        Code::N1,
        Code::N2,
        Code::N3,
        Code::N4,
        Code::N5,
        Code::N6,
        // 0x10     16
        Code::N7,
        Code::N8,
        Code::N9,
        Code::N0,
        Code::Minus,
        Code::Equals,
        Code::Backspace,
        Code::Tab,
        Code::Q,
        Code::W,
        Code::E,
        Code::R,
        Code::T,
        Code::Y,
        Code::U,
        Code::I,
        // 0x20     32
        Code::O,
        Code::P,
        Code::LeftBracket,
        Code::RightBracket,
        Code::Enter,
        Code::LeftCtrl,
        Code::A,
        Code::S,
        Code::D,
        Code::F,
        Code::G,
        Code::H,
        Code::J,
        Code::K,
        Code::L,
        Code::Semicolon,
        // 0x30     48
        Code::Quote,
        Code::Grave,
        Code::LeftShift,
        Code::UK_Hash,
        Code::Z,
        Code::X,
        Code::C,
        Code::V,
        Code::B,
        Code::N,
        Code::M,
        Code::Comma,
        Code::Period,
        Code::Slash,
        Code::RightShift,
        Code::KP_Multiply,
        // 0x40     64
        Code::LeftAlt,
        Code::Space,
        Code::CapsLock,
        Code::F1,
        Code::F2,
        Code::F3,
        Code::F4,
        Code::F5,
        Code::F6,
        Code::F7,
        Code::F8,
        Code::F9,
        Code::F10,
        Code::KP_NumLock,
        Code::ScrollLock,
        Code::KP_7,
        // 0x50     80
        Code::KP_8,
        Code::KP_9,
        Code::KP_Subtract,
        Code::KP_4,
        Code::KP_5,
        Code::KP_6,
        Code::KP_Add,
        Code::KP_1,
        Code::KP_2,
        Code::KP_3,
        Code::KP_0,
        Code::KP_Period,
        Code::Unknown,
        Code::Unknown,
        Code::UK_Backslash,
        Code::F11,
        // 0x60     96
        Code::F12,
        Code::Unknown,
        Code::LANG3,   // Katakana
        Code::LANG4,   // Hiragana
        Code::Unknown, // Henkan
        Code::Unknown, // Hiragana_Katakana
        Code::Unknown, // Muhenkan
        Code::Unknown,
        Code::KP_Enter,
        Code::RightCtrl,
        Code::KP_Divide,
        Code::PrintScreen,
        Code::RightAlt,
        Code::Unknown, // line feed
        Code::Home,
        Code::Up,
        // 0x70     112
        Code::PageUp,
        Code::Left,
        Code::Right,
        Code::End,
        Code::Down,
        Code::PageDown,
        Code::Insert,
        Code::Delete,
        Code::Unknown,
        Code::Mute,
        Code::VolumeDown,
        Code::VolumeUp,
        Code::Unknown, // power off
        Code::KP_Equal,
        Code::KP_PlusMinus,
        Code::Pause,
        // 0x80     128
        Code::Unknown, // launch A
        Code::KP_Decimal,
        Code::LANG1, // hangul
        Code::LANG2, // hangul/hanja toggle
        Code::Unknown,
        Code::Unknown,
        Code::LeftSuper,
        Code::Menu,
        Code::Cancel,
        Code::Again,
        Code::Unknown, // SunProps
        Code::Undo,
        Code::Unknown, // SunFront
        Code::Copy,
        Code::Unknown, // Open
        Code::Paste,
        // 0x90     144
        Code::Find,
        Code::Cut,
        Code::Help,
        Code::Unknown, // XF86MenuKB
        Code::Unknown, // XF86Calculator
        Code::Unknown,
        Code::Unknown, //XF86Sleep
        Code::Unknown, //XF86Wakeup
        Code::Unknown, //XF86Explorer
        Code::Unknown, //XF86Send
        Code::Unknown,
        Code::Unknown, //Xfer
        Code::Unknown, //launch1
        Code::Unknown, //launch2
        Code::Unknown, //WWW
        Code::Unknown, //DOS
        // 0xA0     160
        Code::Unknown, // Screensaver
        Code::Unknown,
        Code::Unknown, // RotateWindows
        Code::Unknown, // Mail
        Code::Unknown, // Favorites
        Code::Unknown, // MyComputer
        Code::Unknown, // Back
        Code::Unknown, // Forward
        Code::Unknown,
        Code::Unknown, // Eject
        Code::Unknown, // Eject
        Code::Unknown, // AudioNext
        Code::Unknown, // AudioPlay
        Code::Unknown, // AudioPrev
        Code::Unknown, // AudioStop
        Code::Unknown, // AudioRecord
        // 0xB0     176
        Code::Unknown, // AudioRewind
        Code::Unknown, // Phone
        Code::Unknown,
        Code::Unknown, // Tools
        Code::Unknown, // HomePage
        Code::Unknown, // Reload
        Code::Unknown, // Close
        Code::Unknown,
        Code::Unknown,
        Code::Unknown, // ScrollUp
        Code::Unknown, // ScrollDown
        Code::Unknown, // parentleft
        Code::Unknown, // parentright
        Code::Unknown, // New
        Code::Unknown, // Redo
        Code::Unknown, // Tools
        // 0xC0     192
        Code::Unknown, // Launch5
        Code::Unknown, // Launch6
        Code::Unknown, // Launch7
        Code::Unknown, // Launch8
        Code::Unknown, // Launch9
        Code::Unknown,
        Code::Unknown, // AudioMicMute
        Code::Unknown, // TouchpadToggle
        Code::Unknown, // TouchpadPadOn
        Code::Unknown, // TouchpadOff
        Code::Unknown,
        Code::Unknown, // Mode_switch
        Code::Unknown, // Alt_L
        Code::Unknown, // Meta_L
        Code::Unknown, // Super_L
        Code::Unknown, // Hyper_L
        // 0xD0     208
        Code::Unknown, // AudioPlay
        Code::Unknown, // AudioPause
        Code::Unknown, // Launch3
        Code::Unknown, // Launch4
        Code::Unknown, // LaunchB
        Code::Unknown, // Suspend
        Code::Unknown, // Close
        Code::Unknown, // AudioPlay
        Code::Unknown, // AudioForward
        Code::Unknown,
        Code::Unknown, // Print
        Code::Unknown,
        Code::Unknown, // WebCam
        Code::Unknown,
        Code::Unknown,
        Code::Unknown, // Mail
        // 0xE0     224
        Code::Unknown, // Messenger
        Code::Unknown, // Seach
        Code::Unknown, // GO
        Code::Unknown, // Finance
        Code::Unknown, // Game
        Code::Unknown, // Shop
        Code::Unknown,
        Code::Unknown, // Cancel
        Code::Unknown, // MonBrightnessDown
        Code::Unknown, // MonBrightnessUp
        Code::Unknown, // AudioMedia
        Code::Unknown, // Display
        Code::Unknown, // KbdLightOnOff
        Code::Unknown, // KbdBrightnessDown
        Code::Unknown, // KbdBrightnessUp
        Code::Unknown, // Send
        // 0xF0     240
        Code::Unknown, // Reply
        Code::Unknown, // MailForward
        Code::Unknown, // Save
        Code::Unknown, // Documents
        Code::Unknown, // Battery
        Code::Unknown, // Bluetooth
        Code::Unknown, // WLan
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
        Code::Unknown,
    ]
}
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Code {
    None = 0,
    ErrorRollOver = 1,
    POSTFail = 2,
    ErrorUndefined = 3,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    N1 = 30,
    N2 = 31,
    N3 = 32,
    N4 = 33,
    N5 = 34,
    N6 = 35,
    N7 = 36,
    N8 = 37,
    N9 = 38,
    N0 = 39,
    Enter = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    UK_Hash = 50,
    Semicolon = 51,
    Quote = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    KP_NumLock = 83,
    KP_Divide = 84,
    KP_Multiply = 85,
    KP_Subtract = 86,
    KP_Add = 87,
    KP_Enter = 88,
    KP_1 = 89,
    KP_2 = 90,
    KP_3 = 91,
    KP_4 = 92,
    KP_5 = 93,
    KP_6 = 94,
    KP_7 = 95,
    KP_8 = 96,
    KP_9 = 97,
    KP_0 = 98,
    KP_Period = 99,
    UK_Backslash = 100,
    KP_Equal = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,
    Help = 117,
    Menu = 118,
    Select = 119,
    Stop = 120,
    Again = 121,
    Undo = 122,
    Cut = 123,
    Copy = 124,
    Paste = 125,
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,
    LockingCapsLock = 130,
    LockingNumLock = 131,
    LockingScrollLock = 132,
    KP_Comma = 133,
    KP_EqualSign = 134,
    International1 = 135,
    International2 = 136,
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,
    LANG1 = 144, // Hangul / English toggle
    LANG2 = 145, // Hanja conversion
    LANG3 = 146, // Katakana
    LANG4 = 147, // Hiragana
    LANG5 = 148, // Zenkaku/Hankaku
    LANG6 = 149,
    LANG7 = 150,
    LANG8 = 151,
    LANG9 = 152,
    AltErase = 153,
    SysReq = 154,
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSelProps = 163,
    ExSel = 164,

    KP_00 = 176,
    KP_000 = 177,
    ThousandsSep = 178,
    DecimalSep = 179,
    CurrencyUnit = 180,
    CurrencySubUnit = 181,
    KP_LeftParent = 182,
    KP_RightParent = 183,
    KP_LeftCurly = 184,
    KP_RightCurly = 185,
    KP_Tab = 186,
    KP_Backspace = 187,
    KP_A = 188,
    KP_B = 189,
    KP_C = 190,
    KP_D = 191,
    KP_E = 192,
    KP_F = 193,
    KP_XOR = 194,
    KP_Pow = 195,
    KP_Percent = 196,
    KP_LeftAngle = 197,
    KP_RightAngle = 198,
    KP_BitAnd = 199,
    KP_LogicAnd = 200,
    KP_BitOr = 201,
    KP_LogicOr = 202,
    KP_Colon = 203,
    KP_Hash = 204,
    KP_Space = 205,
    KP_At = 206,
    KP_Not = 207,
    KP_MemStore = 208,
    KP_MemRecall = 209,
    KP_MemClear = 210,
    KP_MemAdd = 211,
    KP_MemSubtract = 212,
    KP_MemMultiply = 213,
    KP_MemDivide = 214,
    KP_PlusMinus = 215,
    KP_Clear = 216,
    KP_ClearEntry = 217,
    KP_Binary = 218,
    KP_Octal = 219,
    KP_Decimal = 220,
    KP_Hexadecimal = 221,

    #[strum(serialize = "ctrl")]
    LeftCtrl = 224,
    #[strum(serialize = "shift")]
    LeftShift = 225,
    LeftAlt = 226,
    LeftSuper = 227,
    RightCtrl = 228,
    RightShift = 229,
    RightAlt = 230,
    RightSuper = 231,

    Unknown = 255,
}
