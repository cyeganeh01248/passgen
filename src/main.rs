use clap::Parser;
use clap::Subcommand;
use rand::prelude::*;
use rand::seq::SliceRandom;
use zeroize::Zeroizing;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    num: u32,
    #[arg(short, long, default_value_t = 20)]
    length: usize,
    #[command(subcommand)]
    cmd: Option<Subcommands>,
}
#[derive(Subcommand, Debug, Clone, Copy)]
enum Subcommands {
    Words,
    Chars,
}

fn main() {
    let args = Args::parse();
    let sub = args.cmd.unwrap_or(Subcommands::Words);
    let passwords = match sub {
        Subcommands::Words => gen_word_passwords(args),
        Subcommands::Chars => gen_char_passwords(args),
    };
    for password in passwords {
        println!("{}", &*password);
    }
}

fn gen_word_passwords(args: Args) -> Vec<Zeroizing<String>> {
    let words = include_str!("./words_alpha.txt")
        .to_string()
        .split("\n")
        .map(|n| n.to_string())
        .filter(|n| n.len() >= 4 && n.len() <= 16)
        .collect::<Vec<String>>();
    let mut passwords = vec![];
    let mut rng = thread_rng();

    for _i in 0..args.num {
        passwords.push(gen_word_password(&mut rng, &words, args.length));
    }
    return passwords;
}

fn gen_word_password(rng: &mut ThreadRng, words: &Vec<String>, length: usize) -> Zeroizing<String> {
    let mut passw;
    loop {
        passw = Zeroizing::new(String::new());
        while passw.len() < length {
            let mut p = words.choose(rng).unwrap().clone();
            p = p[0..1].to_string().to_uppercase().clone() + &p[1..];
            passw.extend(&mut p.chars());
            passw.extend(format!("{}", rng.gen_range(0..1000)).chars());
            passw.extend(["!@#$%^&*()<>?,.;':\"[]{}|/\\".chars().choose(rng).unwrap()].iter());
        }
        if &passw.len() == &length {
            return passw;
        }
    }
}

fn gen_char_passwords(args: Args) -> Vec<Zeroizing<String>> {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()<>?,.;':\"[]{}|/\\"
        .chars()
        .collect::<Vec<char>>();
    let mut passwords = vec![];
    let mut rng = thread_rng();

    for _ii in 0..args.num {
        passwords.push(gen_char_password(&mut rng, &chars, args.length));
    }
    return passwords;
}

fn gen_char_password(rng: &mut ThreadRng, chars: &Vec<char>, length: usize) -> Zeroizing<String> {
    let mut passw = Zeroizing::new(String::new());
    while passw.len() < length {
        let c = chars.choose(rng).unwrap();
        passw.extend([c]);
    }
    return passw;
}
