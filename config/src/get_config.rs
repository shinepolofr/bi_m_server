use super::cfgs::Configs;
use once_cell::sync::Lazy;
use std::{fs::File, io::Read};
const CFG_FILE: &str = "config/src/config.toml";

pub static CFG: Lazy<Configs> = Lazy::new(Configs::init);

impl Configs {
    pub fn init() -> Self {
        let mut file = match File::open(CFG_FILE) {
            Ok(f) => f,
            Err(e) => panic!("{} file does not exists, err msg: {}", CFG_FILE, e),
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => s,
            Err(e) => panic!("read file failed, err msg: {}", e),
        };
        toml::from_str(&cfg_contents).expect("parse config file failed!")
    }
}
