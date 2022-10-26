# rust-lsb
Simple least significant bit (LSB) image steganography algorithm implemented in Rust.

## Usage

To enode a file into an image:

```console
rust-lsb w hidden_message.txt host_image.png encoded.png
```

To extract an encoded file from an image:

```console
rust-lsb r encoded.png hidden_message.txt
```

## Todo

- Implement variable bit usage i.e. use 2 least significant bits etc.
- Implement better logging
- Implement better error handling
