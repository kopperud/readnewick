use std::io;
use std::io::{BufRead, BufReader};

extern crate bytecount;

pub fn count_lines<R: io::Read>(handle: R) -> Result<usize, io::Error> {
    let mut reader = BufReader::with_capacity(1024*32, handle);
    let mut count = 0;

    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty(){
                break;
            }
            count += bytecount::count(buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    }
    Ok(count)
}
