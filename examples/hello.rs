fn main() {
    let s = create_large_shirt();

    let mut file = std::fs::File::create("./tmp").unwrap();

    let mut v = vec![];
    println!("{}", s.encoded_len());
    let _ = s.encode(&mut v).unwrap();
    file.write_all(v.as_slice()).unwrap();
}

extern crate alloc;
extern crate core;
extern crate prost;
extern crate prost_build;
extern crate prost_types;
extern crate protobuf;

mod examples {
    include!(concat!(env!("OUT_DIR"), "/examples.items.rs"));
}

use std::io::Write;

use examples::Event;
use prost::Message;
use prost_types::Timestamp;

pub fn create_large_shirt() -> Event {
    let mut data = Event::default();
    data.event_time = Some(Timestamp::date(2024, 12, 13).unwrap());
    data.payload = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    data
}
