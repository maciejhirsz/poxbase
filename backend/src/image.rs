use std::{fs, io};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;

use image::ImageFormat;
use image::io::Reader as ImageReader;
// use image::codecs::dds::DdsDecoder;

pub fn run() -> anyhow::Result<()> {
    let mut icon = fs::File::open("./icon_dot_burn.pep")?;
    let mut bytes = Vec::new();

    icon.read_to_end(&mut bytes)?;

    let len = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
    let head = std::str::from_utf8(&bytes[2..2 + len])?;



    println!("head {:?}", head);

    let mut body = Vec::new();
    body.extend_from_slice(&bytes[2+len..]);

    for (byte, key) in body.iter_mut().zip(head.bytes()) {
        *byte = byte.wrapping_sub(key);
    }

    println!("rest {:?} {:?}", String::from_utf8_lossy(&body[..20]), &body[..20]);

    // let img = ImageReader::new(Cursor::new(body)).decode()?;

    // let img = ImageReader::open("./icon_dot_burn.pep")?.decode()?;

    // println!(">>> {:?}", img.to_rgb());

    Ok(())
}
