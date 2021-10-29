## Introduction
Lossless compression algorithms implemented in Rust. 

## External Libraries
- `bitvec`: stores prefix codes
- `criterion`: benchmarks the program
- `clap`: handles command line interface logic
- Native testing features make sure the code works

## Structure 
The logic is in the local library called `compression`. `main.rs` uses the library to take bytes from stdin and either compress or decompress them, then prints the result to stdout. There's only Huffman Coding, so nested directories are unnecessary. 
The error handling is not very robust, it will crash with an invalid file to decompress or an empty file. 

## Demonstration
```
cargo build --release

# There are other files to test in ./corpus
FILE=./corpus/alice29.txt

# Make compressed file from file in corpus
target/release/compression -c -i $FILE -o compressed_file

# Decompress the compressed file
target/release/compression -d -i compressed_file -o text_file

# Open the decompressed file to make sure it's correct
less text_file

# Compare the compressed size to the original
du -h compressed_file text_file
```

## Motivation
- Data structures are ubiquitous 
- Rust is fast and its syntax is logical 
- Compression is useful and easily testable
- Huffman coding is easy to implement

## Future
This project was short, and now I have some experience with data structures and implementing algorithms. I could implement a binary heap or bit vector to get some more experience, but I want to try out some other topics.
