use std::env::args;
use png::Decoder;
use std::fs::read;
use std::fs::File;


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
        for i in bin_u8(*byte).iter() {
            output.push(*i);
        }
    }
    return output;
}


fn main() {

    // pull command line arguments into strings vector
    let args: Vec<String> = args().collect();

    // read contents of message file
    let msg_contents: Vec<u8> = read(&args[1]).unwrap();

    // convert contents of message to boolean vector
    let bin: Vec<bool> = bin_vec_u8(msg_contents);

    // pull info and reader objects from new decoder object for image
    let decoder = Decoder::new(File::open(&args[2]).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    // initialize mutable buffer vector
    let mut buf = vec![0; info.buffer_size()];

    // read image from reader into buffer vector
    reader.next_frame(&mut buf).unwrap();

}
