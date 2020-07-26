use std::env;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn usage() {
    let msg = "Usage: cypher [-ed] <key> <text>";
    let msg2 = "  `key` and `text` are assumed to be files to be read. If no files are found they \
                are treated as literal strings.";
    let msg3 = "  Flags: One of either -e (encypher) or -d (decypher) must be provided.";
    println!("{}\n{}\n{}", msg, msg2, msg3);
    std::process::exit(1);
}

fn nums_to_string(nums: &Vec<usize>) -> String {
    nums.into_iter().map(|&n| &LETTERS[n]).collect()
}

fn char_to_num(c: &char, letters: &[char; 26]) -> usize {
    letters.iter().position(|&x| x == *c).unwrap()
}

fn string_to_nums(s: &String) -> Vec<usize> {
    let c: Vec<char> = s.chars().collect();
    c.into_iter().map(|x| char_to_num(&x, &LETTERS)).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
    }
    let n = vec![0, 1, 2];
    let x = nums_to_string(&n);
    println!("{:?} => {:?}", n, x);
    let s = String::from("HELLO");
    let y = string_to_nums(&s);
    println!("{} => {:?}", s, y);
}
