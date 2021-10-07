# Introduction
Lossless compression algorithms implemented in Rust. 
## External Libraries
- Prefix codes are stored using the `bitvec` crate.
- Benchmarking uses the `criterion` crate.
- Testing uses the native testing features.
## Structure 
The logic is in the local library called `compression`. `main.rs` uses the library to take bytes from stdin and either compress or decompress them, then prints the result to stdout. Each compression algorithm is imported as a submodule of the `compression` library. 
The error handling is not very robust, it will crash with an invalid file to decompress or an empty file. 

# Demonstration
```
cargo build --release

# There are other files to test in ./corpus
FILE=./corpus/alice29.txt

# Make compressed file from file in corpus
cat $FILE | target/release/compression c > compressed_file

# Decompress the compressed file
cat compressed_file | target/release/compression d > text_file

# Open the decompressed file to make sure it's correct
less text_file

# Compare the compressed size to the original
du -h text_file
du -h compressed_file
```

# Motivation
- Data structures are ubiquitous 
- Rust is fast and its syntax is logical 
- Compression is useful and easily testable
- Huffman coding is easy to implement
- Github protects my code from my own mistakes
- Other people can start from my knowledge

# Future
This project was short, and now I have some experience with data structures and implementing algorithms. I could implement a hashmap, binary heap, and bit vector to get some more experience, but I want to try out some other topics.
