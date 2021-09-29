```
cargo build --release

# ./corpus has text files to test on
cat text_file | target/release/compression c > compressed_file
cat compressed_file | target/release/compression d > text_file
```

- Data structures are abstract and complex
- Rust is fast and its syntax is logical 
- Compression is useful and easily testable
- Huffman coding is simple and commonly used
- Github protects my code from my own mistakes
- Other people can start from my knowledge

