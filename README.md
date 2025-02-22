# rust_cli_example
The program takes an image file as an input and creates two tinted copies of it. Used tints are red and blue. Both copies will be created concurrently on their own threads.

# Supported images
Only uncompressed RGB .tga image files are supported.

# How to run the code
cargo run [input_file] [output_folder]

Use absolute paths for both arguments.
