pub struct CartridgeHeader {
    data: Vec<u8>,
    title: String,
}

impl CartridgeHeader {
    const HEADER_START: u16 = 0x0100;

    pub fn new(data: &[u8]) -> Self {
        let mut title: String = "".to_string();

        for n in (0x134 - Self::HEADER_START)..=(0x143 - Self::HEADER_START) {
            let byte = data[n as usize];
            let c = if byte.is_ascii() { byte as char } else { '_' };
            title.push(c);
        }

        CartridgeHeader {
            data: data.to_vec(),
            title,
        }
    }

    pub fn print(&self) {
        println!("Title: {}", self.title);
    }
}
