fn main() {
    let text = include_bytes!("alice.txt");
    println!("The file is {} bytes.", text.len());
    let res = compress_rle(text.to_vec()).unwrap();
    println!("The compressed version is {} bytes.", res.len());
}

fn compress_rle(input: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut output : Vec<u8> = Vec::new();
    let mut run = input[0];
    let mut run_len = 0;
    for cur in input {
        if cur == run {
            run_len = run_len + 1;
        }
        else {
            output.append(&mut vec![run_len, run]);
            run = cur;
            run_len = 1;
        }
    }
    output.append(&mut vec![run_len, run]);
    Ok(output)
}

