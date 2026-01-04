use modular_bitfield::prelude::*;


#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct TcpSegment {
    source_port: B16,
    dest_port: B16,
    seq_num: B32,
    ack_num: B32,
    offset: B4,
    reserved: B4,
    c: bool,
    e: bool,
    u: bool,
    a: bool,
    p: bool,
    r: bool,
    s: bool,
    f: bool,
    window: B16,
    checksum: B16,
    urgent_pointer: B16,
    tcp_options: B32,
}


fn main() {
    println!("Hello, world!");
}
