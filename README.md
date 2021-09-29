# Introduction
Lossless compression algorithms implemented in Rust. Most underlying data structures are implemented by other libraries, including `std::HashMap` and `bitvec::vec::BitVec`. The `criterion` library is used for benchmarking, and the native testing library is used for testing. The logic is in the local library called `compression`. `main.rs` uses the library to take bytes from stdin and either compress or decompress them, then prints the result to stdout. The error handling is not very robust, it will crash with an invalid file to decompress or an empty file. 

# Demonstration
```
cargo build --release

# There are other files to test in ./corpus
FILE=./corpus/alice.txt

# Make compressed file from file in corpus
cat $FILE | target/release/compression c > compressed_file

# Decompress the compressed file
cat compressed_file | target/release/compression d > text_file

# There is no difference between the decompressed file and the original
diff text_file $FILE

# Or you can look at the decompressed file to make sure it's the same
less text_file
```

# Motivation
- Data structures are ubiquitous 
- Rust is fast and its syntax is logical 
- Compression is useful and easily testable
- Huffman coding is easy to implement
- Github protects my code from my own mistakes
- Other people can start from my knowledge

