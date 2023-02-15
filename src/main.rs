#![feature(test)]
#![feature(portable_simd)]
use std::{simd::*, fs::File, io::{Read, BufReader, Write}};

fn main() {
    sisd();
    simd();

}

const w: usize = 4032;
const h: usize = 3024;
const header: &[u8] = b"P6\n4032 3024\n255\n";

fn sisd() {
    
    let mut of = File::create("out_sisd.ppm").unwrap();

    let f = File::open("img.ppm").unwrap();
    let mut reader = BufReader::new(f);
    
    let mut buf = Box::new(Vec::new());
    reader.read_to_end(&mut buf).unwrap();

    if !buf.starts_with(header) {
        println!("wrong file");
        return
    }
    let mut output = vec![0_u8; 3*w*h+header.len()];

    
    // let mask = u8x32::from_slice([0b00111111 as u8,0b00001111,0b01111111].iter().cycle().take(32).collect());
    let mask = [0b00111111_u8, 0b00001111, 0b01111111];

    
    for x in header.len()..w*h*3 {
        output[x] = buf[x] ^ mask[(x+1)%3];
    }   


    of.write(header).unwrap();
    of.write_all(output.as_ref()).unwrap();
}



fn simd() {
    
    let mut of = File::create("out_simd.ppm").unwrap();

    let f = File::open("img.ppm").unwrap();
    let mut reader = BufReader::new(f);
    
    let mut buf = Box::new(Vec::new());
    reader.read_to_end(&mut buf).unwrap();


    
    if !buf.starts_with(header) {
        println!("wrong file");
        return
    }
    let mut output = vec![0_u8; 3*w*h+header.len()];

    
    // let mask = u8x32::from_slice([0b00111111 as u8,0b00001111,0b01111111].iter().cycle().take(32).collect());
    let mask = u8x32::from_slice(&[0b00111111_u8,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111,0b01111111,0b00111111,0b00001111]);
    
    for x in 0..((w*h)/8)-1 {
        let aa = u8x32::from_slice(&buf[header.len()+24*x .. header.len()+24*(x+1)+(32-24)]);

        output[24*x..24*(x+1)].clone_from_slice(&(aa ^ mask).to_array()[0..24]);
    }   

    of.write(header).unwrap();
    of.write_all(output.as_ref()).unwrap();
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn a(b: &mut Bencher) {
        b.iter(|| sisd());
    }


    #[bench]
    fn abc(b: &mut Bencher) {
        b.iter(|| simd());
    }
}