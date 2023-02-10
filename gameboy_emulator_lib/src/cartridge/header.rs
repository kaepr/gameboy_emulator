pub struct CartridgeHeader {
    data: Vec<u8>,
    title: String,
}

const HEADER_START: u16 = 0x0100;

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Self {

        let title = ((0x134 - HEADER_START)..=(0x143 - HEADER_START))
            .fold("".to_string(), |acc, x| {
                let byte = data[x as usize];
                let c = if byte.is_ascii() { byte as char } else { '_' };
                format!("{acc}{c}")
            });

        CartridgeHeader {
            data: data.to_vec(),
            title,
        }
    }

    pub fn print(&self) {
        println!("Title: {}", self.title);
    }
}
