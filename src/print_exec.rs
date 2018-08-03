#[cfg(target_os="macos")]
extern crate core_foundation as cf;

use std::fs::File;
use std::io::{Read, BufReader};
use std::path::PathBuf;


#[cfg(target_os="macos")]
fn exec_path() -> Option<PathBuf> {
    use cf::bundle::CFBundle;

    CFBundle::main_bundle()
        .executable_url()
        .and_then(|x| x.to_path())
}

#[cfg(target_os = "linux")]
fn exec_path() -> Option<PathBuf> {
    use std::fs::read_link;

    read_link("/proc/self/exe").ok()
}

fn main() {
    let path = exec_path().expect("cannot obtain executable path of the main bundle");

    let file = File::open(path).expect("must be able to read the file");
    let mut buf = BufReader::new(file);

    let mut read_buf: &mut [u8] = &mut [0u8; 1024];
    let mut size = 0usize;
    let mut total_size = 0usize;

    while {
        size = buf.read(read_buf).iter().fold(0usize, |_, &v| v);
        size > 0
    } {
        total_size += size;
        for x in read_buf.iter().take(size) {
            print!("{:02x} ", x);
        }
    }
    println!("");
    println!("Total executable size is {} bytes", total_size);
}
