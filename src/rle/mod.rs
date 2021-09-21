pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut output : Vec<u8> = Vec::new();
    let mut run = input[0];
    let mut run_len = 0;
    for cur in input {
        if *cur == run {
            run_len = run_len + 1;
        }
        else {
            output.append(&mut vec![run_len, run]);
            run = *cur;
            run_len = 1;
        }
    }
    output.append(&mut vec![run_len, run]);
    output
}
