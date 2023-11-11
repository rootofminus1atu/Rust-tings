
struct CStyle;

impl CStyle {
    const NAME: &'static [u8] = b"HI";
    const ANOTHER_ONE: &'static [u8] = b"hello";
}

pub fn display() {
    let response = CStyle::NAME;
    println!("{:?}", response);
}


