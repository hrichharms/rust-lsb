use std::env::args;
use std::fs::read;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::string::String;
use std::io::Write;

use png::Decoder;
use png::Encoder;


// converts an unsigned 16-bit integer into its binary form
// represented by a boolean vector
// EXAMPLE: 5 -> 0000 0000 0000 0101
//          bin_u16(5u16) -> [false, false, ..., false, true, false, true]
fn bin_u16(mut x: u16) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for _ in 0..16 {
        output.push(x % 2 == 1);
        x /= 2;
    }
    output.reverse();
    return output;
}


// converts a 16-element boolean vector representing a binary number
// to the unsigned 16-bit interger it represents
// EXAMPLE: 0000 0000 0000 0101 -> 5
//          u16_bin(vec![false, false, ..., false, true, false, true])
fn u16_bin(x: Vec<bool>) -> u16 {
    let mut output = 0u16;
    for i in 0..16 {
        if x[i] {
            output += 2u16.pow(15 - i as u32);
        }
    }
    return output;
}


// converts an unsigned 8-bit integer into its binary form
// represented by a boolean vector
// EXAMPLE: 5 -> 0000 0101
//          bin_u8(5u8) -> [false, false, false, false, false, true, false, true]
fn bin_u8(mut x: u8) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for _ in 0..8 {
        output.push(x % 2 == 1);
        x /= 2;
    }
    output.reverse();
    return output;
}


// converts a vector of unsigned 8-bit intergers into their
// binary form represented by a boolean vector
// EXAMPLE: [5, 9] -> 0000 0101 0000 1001
//          bin_vec_u8(vec![5u8, 9u8]) -> vec![false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, true]
fn bin_vec_u8(x: Vec<u8>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for byte in x.iter() {
        for bit in bin_u8(*byte).iter() {
            output.push(*bit);
        }
    }
    return output;
}


// converts an 8-element boolean vector representing a binary number
// to the unsigned 8-bit integer it represents
// EXAMPLE: 0000 0101 -> 5
//          u8_bin(vec![false, false, false, false, false, true, false, true]) -> 5u8
fn u8_bin(x: Vec<bool>) -> u8 {
    let mut output = 0u8;
    for i in 0..8 {
        if x[i] {
            output += 2u8.pow(7 - i as u32);
        }
    }
    return output;
}


// converts a vector of booleans and converts each 8-element chunk
// into the unsigned 8-bit integer that they represent then returns
// a vector of the results
// EXAMPLE: 0000 0101 0000 1001 -> [5, 9]
//          u8_vec_bin(vec![false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, true]) -> vec![5u8, 9u8]
fn u8_vec_bin(x: Vec<bool>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for byte in x.chunks(8) {
        if byte.len() == 8 {
            output.push(u8_bin(byte.to_vec()));
        }
    }
    return output;
}


// reads the least significant bits of a given vector of unsigned
// 8-bit integers and stores them in a vector of booleans
fn vec_u8_lsb(x: Vec<u8>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for byte in x.iter() {
        output.push(byte % 2 == 1);
    }
    return output;
}


fn main() {

    // pull command line arguments into strings vector
    let args: Vec<String> = args().collect();

    if args[1].eq(&String::from("w")) {
        // args[2] -> hidden message to hide in host image
        // args[3] -> host image to hide message in
        // args[4] -> where the results should be written (filename)

        // read contents of message file
        let message_contents = read(&args[2]).unwrap();

        // convert contents of message to boolean vector
        let message_bin = bin_vec_u8(message_contents);

        // create 16-bit length header for hidden message
        let header = bin_u16(message_bin.len() as u16);

        // combine header and message_bin for encoding payload
        let mut encoding_payload: Vec<bool> = Vec::new();
        encoding_payload.extend(header.iter());
        encoding_payload.extend(message_bin.iter());

        // pull info and reader objects from new decoder object for image
        let decoder = Decoder::new(File::open(&args[3]).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        // check if the file is capable of holding the hidden message
        if info.buffer_size() > encoding_payload.len() {

            // initialize mutable buffer vector
            let mut data = vec![0; info.buffer_size()];

            // read image from reader into buffer vector
            reader.next_frame(&mut data).unwrap();

            // encode payload binary data into target image data
            let mut i = 0;
            for bit in encoding_payload.iter() {
                if *bit && data[i] % 2 == 0 {
                    data[i] += 1;
                } else if !*bit && data[i] % 2 != 0 {
                    data[i] -= 1;
                }
                i += 1;
            }

            // create new output file
            let output_file = File::create(Path::new(&args[4])).unwrap();

            // println!("{} {} {} {}", info.width, info.height, info.width * info.height, data.len());

            // create image BufWriter and Encoder objects
            let ref mut w = BufWriter::new(output_file);
            let mut encoder = Encoder::new(w, info.width, info.height);

            // set color type and bit depth attributes of new image encoder
            encoder.set_color(info.color_type);
            encoder.set_depth(info.bit_depth);

            // create writer object
            let mut writer = encoder.write_header().unwrap();

            // write data to output file
            writer.write_image_data(&data).unwrap();

        } else {

            // file is too large
            println!("Hidden message too large for target image!");

        }

    } else if args[1].eq(&String::from("r")) {
        // args[2] -> image with hidden message
        // args[3] -> where the extracted message should be written (filename)

        // pull reader object from new decoder object for target image
        let decoder = Decoder::new(File::open(&args[2]).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        // initialize mutable buffer vector
        let mut data = vec![0; info.buffer_size()];

        // read image from reader into buffer vector
        reader.next_frame(&mut data).unwrap();

        // extract the 16-bit length header from the image data
        // and calculate the length of the hidden message
        let (header, remainder) = data.split_at(16);
        let message_len = u16_bin(vec_u8_lsb(header.to_vec()));

        // extract the binary representation of the hidden message from
        // the remainder according to the calculated message length
        let (message_bytes, _) = remainder.split_at(message_len as usize);
        let message_bin = vec_u8_lsb(message_bytes.to_vec());

        // convert the message binary to a u8 vector
        let message = u8_vec_bin(message_bin);

        // create output file and write message to it
        let mut output_file = File::create(Path::new(&args[3])).unwrap();
        output_file.write_all(&message).unwrap();

    }
}
