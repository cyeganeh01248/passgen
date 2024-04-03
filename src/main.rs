use clap::Parser;
use rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaChaRng;
use std::fs::read_to_string;
use zeroize::{Zeroize, Zeroizing};

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    num: u32,
    #[arg(short, long, default_value_t = 20)]
    length: usize,
}

fn main() {
    let args = Args::parse();

    let words = read_to_string("words_alpha.txt")
        .unwrap()
        .split("\r\n")
        .map(|n| n.to_string())
        .filter(|n| n.len() >= 4)
        .collect::<Vec<String>>();

    for _i in 0..args.num {
        let mut p = gen_random(&words, args.length);
        let t = &*p;
        println!("{}", t);
        p.zeroize();
    }
}
fn gen_random(words: &Vec<String>, length: usize) -> Zeroizing<String> {
    let mut rng = ChaChaRng::from_entropy();
    loop {
        let mut passw = Zeroizing::new(String::new());
        while passw.len() < length {
            let mut p = words.choose(&mut rng).unwrap().clone();
            p = p[0..1].to_string().to_uppercase().clone() + &p[1..];
            passw.extend(&mut p.chars());
            passw.extend(format!("{}", rng.gen_range(0..100)).chars());
            passw.extend(
                ["!@#$%^&*()<>?,./;':\"[]{}|\\"
                    .chars()
                    .choose(&mut rng)
                    .unwrap()]
                .iter(),
            );
        }
        if &passw.len() == &length {
            return passw;
        }
    }
}
