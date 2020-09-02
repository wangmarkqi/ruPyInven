use std::fs::{read_to_string, File};
use std::io::prelude::*;
pub const Wrong: &'static str = "something wrong";

pub fn default_string() -> String {
    "".to_string()
}

pub fn default_bool_false() -> bool {
    false
}

pub fn default_f32() -> f32 {
    0.0
}

pub fn default_vec_string() -> Vec<String> {
    vec![]
}

pub fn test() {}

pub fn create_dir_if_not_exists(dir: &str) {
    let p = std::path::Path::new(dir);
    if p.exists() {
        return;
    };
    std::fs::create_dir(p).unwrap();
}
