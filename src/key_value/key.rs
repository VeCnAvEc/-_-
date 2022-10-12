use std::borrow::Cow;
use std::str;

const MOUSE_1: u8 = 1;
const MOUSE_2: u8 = 2;
const MOUSE_CLICK_SCROLL: u8 = 4;

const L_SHIFT: [u8; 2] = [16, 160];
const R_SHIFT: [u8; 2] = [16, 161];
const L_CTRL: [u8; 2] = [17, 28];
const R_CTRL: [u8; 2] = [17, 163];
const L_ALT: [u8; 2] = [18, 164];
const R_ALT: [u8; 2] = [18, 165];

const ESCAPE: u8 = 27;
const BACKSPACE: u8 = 8;
const ENTER: u8 = 13;
const TAB: u8 = 9;
const CAPS_LOCK: u8 = 20;
const WIN: u8 = 91;

const F: [u8; 12] = [112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123];

const CP_KEY: [[u8; 2]; 10] = [
                                [96, 45], [97, 35], [98, 40], [99, 34], [100, 37],
                                [101, 12], [102, 39], [103, 36], [104, 38], [105, 33]
                              ];

const PRINT_SCREENSHOT: u8 = 44;
const SCROLL_LOCK: u8 = 145;

const CP_STAR: u8 = 106;
const CP_PLUS: u8 = 107;
const CP_MINUS: u8 = 109;
const CP_DELETE: u8 = 110;
const CP_SLASH: u8 = 111;
const CP_NUM_LOCK: u8 = 144;

const INSTANCE: u8 = 45;
const DELETE: u8 = 46;
const HOME: u8 = 36;
const END: u8 = 35;
const PAGE_UP: u8 = 33;
const PAGE_DOWN: u8 = 34;

// #[derive(Debug)]
// pub enum Modifier{
//     NotModifier(bool),
//     SHIFT(bool),
//     CAPSLOCK(bool),
//     NumLock(bool),
// }
//
// #[derive(Debug)]
// pub struct InputEvent {
//     pub(crate) value: String,
//     modifier: Modifier,
//     ready: bool
// }

// pub trait Clone {
//     fn clone(&self) -> Self;
//
//     fn new(modifier_bytes: ModifierEvent, latter_bytes: Vec<u8>, prev_byte_modifier: Vec<u8>, ready: bool) -> Self;
//
//     fn define_modifier(&self, bytes: &Vec<u8>) -> BytesModifier;
//
//
//     // fn clone_from(&mut self, source: &Self) -> Self;
// }

#[derive(Debug, Clone, Copy)]
pub enum ModifierEvent {
    SHIFT,
    CAPSLOCK,
    CTRL,
    ALT,
    NOT_MODIFIER
}

#[derive(Debug)]
pub struct BytesModifier {
    pub modifier_bytes: ModifierEvent,
    pub latter_bytes: Vec<u8>,
    pub prev_byte_modifier: Vec<u8>,
    ready: bool
}

impl BytesModifier {
    // fn clone(&self) -> Self {
    //     *Self
    // }

    pub fn new(modifier_bytes: ModifierEvent, latter_bytes: Vec<u8>, prev_byte_modifier: Vec<u8>, ready: bool) -> Self {
        BytesModifier {
            modifier_bytes,
            latter_bytes,
            prev_byte_modifier,
            ready
        }
    }

    pub fn define_modifier(bytes: &Vec<u8>, prev_latter_byte: &Vec<u8>) -> BytesModifier {
        let mut modifier = Vec::new();
        let mut latter = Vec::new();
        let bytes_modifier_method = BytesModifier::new;

        for byte in bytes {
            if *byte == 16 || *byte == 160 || *byte == 161 {
                modifier.push(*byte);
            } else if *byte == 17 || *byte == 162 || *byte == 163 {
                modifier.push(*byte);
            } else if *byte == 18 || *byte == 164 || *byte == 165 {
                modifier.push(*byte);
            }

            if *byte >= 65 && *byte <= 90 {
                latter.push(*byte);
            }
        }
        let mut latter_len = latter.len();

        return if modifier.len() == 2 {
            if modifier[0] == 16 && modifier[1] == 160 || modifier[1] == 161 {
                bytes_modifier_method(ModifierEvent::SHIFT, latter, prev_latter_byte.clone(), latter_len >= 1)
            } else if modifier[0] == 17 && modifier[1] == 162 || modifier[1] == 163 {
                bytes_modifier_method(ModifierEvent::CTRL, latter, prev_latter_byte.clone(), latter_len >= 1)
            } else if modifier[0] == 18 && modifier[1] == 164 || modifier[1] == 165 {
                bytes_modifier_method(ModifierEvent::ALT, latter, prev_latter_byte.clone(), latter_len >= 1)
            } else {
                println!("else");
                bytes_modifier_method(ModifierEvent::NOT_MODIFIER, latter, prev_latter_byte.clone(), latter_len >= 1)
            }
        } else {
            bytes_modifier_method(ModifierEvent::NOT_MODIFIER, latter, prev_latter_byte.clone(), latter_len >= 1)
        }
    }
}

// impl InputEvent {
//   pub fn shift_key(bytes: &Vec<u8>) -> InputEvent  {
//        let last_bytes = bytes.last();
//
//         let mut find_bytes = Vec::new();
//
//         if bytes.len() > 1 {
//             if bytes[0] == 16 && last_bytes == Some(&160) || last_bytes == Some(&161) {
//                 for el in bytes {
//                     if el == &16 || el == &160 || el == &161 {
//                         continue;
//                     } else if bytes.len() == 3 {
//                         find_bytes.push(*el);
//                     } else if bytes.len() > 3 {
//                         continue;
//                     }
//                 }
//             };
//         };
//
//         if find_bytes.len() > 0 {
//             InputEvent {
//                 modifier: Modifier::SHIFT(true),
//                 value: (&String::from_utf8_lossy(&[find_bytes[0]])).parse().unwrap(),
//                 ready: true
//             }
//         } else {
//             InputEvent {
//                 modifier: Modifier::NotModifier(true),
//                 value: "".to_string(),
//                 ready: false,
//             }
//         }
//     }
//
//     pub fn not_modifier_key(bytes: &mut Vec<u8>) -> InputEvent {
//         let mut valid_bytes = InputEvent::valid_bytes(bytes);
//
//         if valid_bytes.len() == 1 {
//             InputEvent {
//                 modifier: Modifier::NotModifier(true),
//                 value: (&String::from_utf8_lossy(&[valid_bytes[0]])).parse().unwrap(),
//                 ready: true
//             }
//         } else {
//             InputEvent {
//                 value: "".to_string(),
//                 modifier: Modifier::NotModifier(true),
//                 ready: false
//             }
//         }
//     }
//
//     pub fn valid_bytes(bytes: &mut Vec<u8>) -> &mut Vec<u8> {
//         let mut byte_len = bytes.len();
//         let mut remove_zero_index: bool = false;
//
//         for byte in 0..byte_len {
//             if !remove_zero_index {
//                 if bytes[byte] == MOUSE_1 || bytes[byte] == MOUSE_2 || bytes[byte] == MOUSE_CLICK_SCROLL {
//                     if byte == 0 {
//                         bytes.remove(byte);
//                         remove_zero_index = true;
//                         byte_len -= 1;
//                         continue;
//                     }
//
//                     bytes.remove(byte);
//                     byte_len -= 1;
//                }
//             }
//         }
//         bytes
//     }
// }
//
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

