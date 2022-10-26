use std::env::args;
use std::fs::read;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::string::String;

use png::Decoder;
use png::Encoder;

/// converts an unsigned 16-bit integer into its binary form
/// represented by a boolean vector
///
/// # Example
///
/// ```no_run
/// 5 -> 0000 0000 0000 0101
/// u16_to_bin(5u16) -> [false, false, ..., false, true, false, true]
/// ```
fn u16_to_bin(x: u16) -> [bool; 16] {
    let mut output: [bool; 16] = [false; 16];
    for (idx, i) in output.iter_mut().enumerate() {
        *i = (x >> idx & 1) != 0;
    }
    output.reverse();
    output
}

/// converts a 16-element boolean vector representing a binary number
/// to the unsigned 16-bit interger it represents
///
/// # Example
///
/// ```no_run
/// 0000 0000 0000 0101 -> 5
/// bin_to_u16(vec![false, false, ..., false, true, false, true])
/// ```
fn bin_to_u16(x: &[bool]) -> u16 {
    let mut output = 0u16;
    for (idx, i) in x.iter().enumerate().take(16) {
        if *i {
            output += 1u16 << (15 - idx as u32);
        }
    }
    output
}

/// converts an unsigned 8-bit integer into its binary form
/// represented by a boolean vector
///
/// # Example
///
/// ```no_run
/// 5 -> 0000 0101
/// u8_to_bin(5u8) -> [false, false, false, false, false, true, false, true]
/// ```
fn u8_to_bin(x: u8) -> [bool; 8] {
    let mut output: [bool; 8] = [false; 8];
    for (idx, i) in output.iter_mut().enumerate() {
        *i = (x >> idx & 1) != 0;
    }
    output.reverse();
    output
}

/// converts an 8-element boolean vector representing a binary number
/// to the unsigned 8-bit integer it represents
///
/// # Example
///
/// ```no_run
/// 0000 0101 -> 5
/// bin_to_u8(vec![false, false, false, false, false, true, false, true]) -> 5u8
/// ```
fn bin_to_u8(x: &[bool]) -> u8 {
    let mut output = 0u8;
    for (idx, i) in x.iter().enumerate().take(8) {
        if *i {
            output += 2u8.pow(7 - idx as u32);
        }
    }
    output
}

/// converts a vector of unsigned 8-bit intergers into their
/// binary form represented by a boolean vector
///
/// # Example
///
/// ```no_run
/// [5, 9] -> 0000 0101 0000 1001
/// vec_u8_to_bin(vec![5u8, 9u8]) -> vec![false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, true]
/// ```
fn vec_u8_to_bin(x: &[u8]) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::with_capacity(x.len() * 8);
    for byte in x.iter() {
        for bit in u8_to_bin(*byte).iter() {
            output.push(*bit);
        }
    }
    output
}

/// converts a vector of booleans and converts each 8-element chunk
/// into the unsigned 8-bit integer that they represent then returns
/// a vector of the results
///
/// # Example
///
/// ```no_run
/// 0000 0101 0000 1001 -> [5, 9]
/// bin_to_vec_u8(vec![false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, true]) -> vec![5u8, 9u8]
/// ```
fn bin_to_vec_u8(x: &[bool]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for byte in x.chunks(8) {
        if byte.len() == 8 {
            output.push(bin_to_u8(byte));
        }
    }
    output
}

/// reads the least significant bits of a given vector of unsigned
/// 8-bit integers and stores them in a vector of booleans
fn vec_u8_to_lsb(x: &[u8]) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::with_capacity(x.len());
    for byte in x.iter() {
        output.push(byte % 2 == 1);
    }
    output
}

fn main() {
    // pull command line arguments into strings vector
    let args: Vec<String> = args().collect();

    if args[1].as_str().eq("w") {
        // args[2] -> hidden message to hide in host image
        // args[3] -> host image to hide message in
        // args[4] -> where the results should be written (filename)

        // read contents of message file
        let message_contents = read(&args[2]).unwrap();

        // convert contents of message to boolean vector
        let message_bin = vec_u8_to_bin(&message_contents);

        // create 16-bit length header for hidden message
        let header = u16_to_bin(message_bin.len() as u16);

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
            for (i, bit) in encoding_payload.iter().enumerate() {
                if *bit && data[i] % 2 == 0 {
                    data[i] += 1;
                } else if !*bit && data[i] % 2 != 0 {
                    data[i] -= 1;
                }
            }

            // create new output file
            let output_file = File::create(Path::new(&args[4])).unwrap();

            // println!("{} {} {} {}", info.width, info.height, info.width * info.height, data.len());

            // create image BufWriter and Encoder objects
            let w = &mut BufWriter::new(output_file);
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
    } else if args[1].as_str().eq("r") {
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
        let message_len = bin_to_u16(&vec_u8_to_lsb(header));

        // extract the binary representation of the hidden message from
        // the remainder according to the calculated message length
        let (message_bytes, _) = remainder.split_at(message_len as usize);
        let message_bin = vec_u8_to_lsb(message_bytes);

        // convert the message binary to a u8 vector
        let message = bin_to_vec_u8(&message_bin);

        // create output file and write message to it
        let mut output_file = File::create(Path::new(&args[3])).unwrap();
        output_file.write_all(&message).unwrap();
    }
}
