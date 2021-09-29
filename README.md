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
- Data structures are abstract and complex
- Rust is fast and its syntax is logical 
- Compression is useful and easily testable
- Huffman coding is simple and commonly used
- Github protects my code from my own mistakes
- Other people can start from my knowledge

