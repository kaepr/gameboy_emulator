use std::{fs, path::Path};

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: String) -> Self {
        let path = Path::new(&path);

        match fs::read(path) {
            Ok(data) => Cartridge { data },
            Err(e) => panic!("Error in reading ROM {:?}", e),
        }
    }

    pub fn stat(&self) {
        println!("Cart Size: {}", self.data.len());
    }

    pub fn cart_read(addr: u16) {}
}
