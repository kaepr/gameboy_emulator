use std::{fs, path::Path};

pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: String) -> Self {
        let path = Path::new(&path);

        match fs::read(path) {
            Ok(data) => Rom { data },
            Err(e) => panic!("Error in reading ROM {:?}", e),
        }
    }

    pub fn stat(&self) {
        println!("Cart Size: {}", self.data.len());
    }
}
