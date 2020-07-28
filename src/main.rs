use std::env;
use std::fs;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug)]
enum Mode {
    Encrypt,
    Decrypt,
}

fn usage() {
    let msg = "Usage: cypher [-ed] <key> <text>";
    let msg2 = "  `key` and `text` are assumed to be files to be read. If no files are found they \
                are treated as literal strings.";
    let msg3 = "  Flags: One of either -e (encypher) or -d (decypher) must be provided.";
    println!("{}\n{}\n{}", msg, msg2, msg3);
    std::process::exit(1);
}

fn clean_string(s: &mut String) -> String {
    let mut out = s.to_uppercase();
    out.retain(|c| !c.is_whitespace() && !c.is_ascii_punctuation());
    out
}

fn nums_to_string(nums: &Vec<usize>) -> String {
    nums.into_iter().map(|&n| &LETTERS[n]).collect()
}

fn string_to_nums(s: &String) -> Vec<usize> {
    // Get the index of a character in the alphabet (zero-indexed)
    fn char_to_num(c: &char, letters: &[char; 26]) -> usize {
        letters.iter().position(|&x| x == *c).unwrap()
    }

    let c: Vec<char> = s.chars().collect();
    c.into_iter().map(|x| char_to_num(&x, &LETTERS)).collect()
}

fn read_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename);
    let mut out = match contents {
        Ok(file) => file,
        Err(_) => {
            println!("No file `{}` found, assuming raw text", filename);
            filename.to_string()
        }
    };
    clean_string(&mut out)
}

fn normalise_key_length(key: &mut String, text: &String) {
    // Key must be the same length as input text - repeat / trim as necessary
    let text_len = text.len();
    let key_len = key.len();
    if text_len > key_len {
        println!("Key too short: is {}, should be {}", key_len, text_len);
        let n_reps = text_len / key_len; // Integer division because usize
        if text_len % key_len == 0 {
            key.repeat(n_reps);
        } else {
            // Make over-long, then trim (easier than trying to substring)
            *key = key.repeat(n_reps + 1);
            key.truncate(text_len);
        }
    } else if text_len < key_len {
        key.truncate(text_len);
    }
    // Otherwise they are already the same length and nothing needs doing
}

fn encrypt(key: &String, plaintext: &String) -> String {
    let num_key = string_to_nums(key);
    let num_text = string_to_nums(plaintext);
    let zipper = num_key.iter().zip(num_text.iter());
    let out_nums = zipper
        .map(|x| {
            let (k, t) = x;
            (t + k) % 26
        })
        .collect();
    println!("Out nums: {:?}", out_nums);
    let out_text = nums_to_string(&out_nums);
    out_text
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
    }
    let flag = &args[1];
    if flag != "-e" && flag != "-d" {
        println!("Invalid flag `{}`", flag);
        usage();
    }
    let mode = if flag == "-e" {
        Mode::Encrypt
    } else {
        Mode::Decrypt
    };
    println!("Mode: {:?}", mode);
    let key_input = &args[2];
    let txt_input = &args[3];
    let mut key_text = read_file(key_input);
    let input_text = read_file(txt_input);
    normalise_key_length(&mut key_text, &input_text);
    println!("Key:  {} => {:?}", key_text, string_to_nums(&key_text));
    println!("Text: {} => {:?}", input_text, string_to_nums(&input_text));
    match mode {
        Mode::Encrypt => {
            let out = encrypt(&key_text, &input_text);
            println!("Output: {}", out);
        }
        Mode::Decrypt => {
            println!("Not implemented yet");
        }
    }
}
