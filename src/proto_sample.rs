#![feature(nll)]
extern crate protobuf;

mod protos;

use std::io::*;
use protos::*;
use protobuf::Message;
use protobuf::Clear;


fn main() {
    let mut a = sample::Test::new();
    println!("message size before setting string = {}", a.compute_size());

    // In following block data is written to the file
    {
        let file = std::fs::File::create(std::path::Path::new("/tmp/demo-temp.bin")).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        let mut output_stream = protobuf::CodedOutputStream::new(&mut writer);

        a.set_id(1);
        a.set_name("First".to_owned());
        a.set_yoyo("Yo-first".to_owned());
        a.write_length_delimited_to(&mut output_stream);

        a.clear();
        a.set_id(2);
        a.set_name("Second".to_owned());
        a.set_sub_message(321);
        a.write_length_delimited_to(&mut output_stream);

        let _ = output_stream.flush();

        if writer.flush().is_ok() {
            println!("file is flushed")
        } else {
            println!("failed to flush the buffer")
        }
    }

    println!("try to read the file");

    {
        let file = std::fs::File::open("/tmp/demo-temp.bin").unwrap();
        let mut reader = std::io::BufReader::new(file);

        {
            let mut input_stream = protobuf::CodedInputStream::new(&mut reader);
            // a.clear();
            let a1 = input_stream.read_message::<sample::Test>().unwrap();
            println!("first message {:?}", a1);

            let a2 = input_stream.read_message::<sample::Test>().unwrap();
            println!("second message {:?}", a2);
            println!("first once again {:?}", a1);
        }

        reader.seek(std::io::SeekFrom::Start(0));
        {
            let mut input_stream = protobuf::CodedInputStream::new(&mut reader);
            a.clear();
            input_stream.merge_message(&mut a);
            println!("once again furst: {:?}", a);

            a.clear();
            input_stream.merge_message(&mut a);
            println!("once again second: {:?}", a);
        }
    }
}
