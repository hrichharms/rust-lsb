use std::env::args;
use png::Decoder;
use png::Encoder;
use std::fs::read;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;


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
//          bin_vec_u8(vec![5u8, 9u8]) -> [false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, true]
fn bin_vec_u8(x: Vec<u8>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for byte in x.iter() {
        for bit in bin_u8(*byte).iter() {
            output.push(*bit);
        }
    }
    return output;
}


fn main() {

    // pull command line arguments into strings vector
    let args: Vec<String> = args().collect();

    // read contents of message file
    let message_contents: Vec<u8> = read(&args[1]).unwrap();

    // convert contents of message to boolean vector
    let message_bin: Vec<bool> = bin_vec_u8(message_contents);

    // pull info and reader objects from new decoder object for image
    let decoder = Decoder::new(File::open(&args[2]).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    // initialize mutable buffer vector
    let mut data = vec![0; info.buffer_size()];

    // check if the file is capable of holding the hidden message
    if info.buffer_size() > message_bin.len() {

        // read image from reader into buffer vector
        reader.next_frame(&mut data).unwrap();

        // encode message binary data into target image data
        let mut i = 0;
        for bit in message_bin.iter() {
            if *bit && data[i] % 2 == 0{
                data[i] += 1;
            } else if !*bit && data[i] % 2 != 0 {
                data[i] -= 1;
            }
            i += 1;
        }

        // create new output file
        let output_file = File::create(Path::new(&args[3])).unwrap();

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

}
