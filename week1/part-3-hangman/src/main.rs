// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)

    println!("你想知道这个单词的第几个字符：");
    let mut index = String::new();
    let num: i32 = loop {
        io::stdin().read_line(&mut index).expect("无法读取输入");
        match index.trim().parse() {
            Ok(num) => {
                if num > secret_word_chars.len() as i32 || num < 1 {
                    println!("这个单词没有这么长/短，请重新输入：");
                    index.clear();
                    continue;
                } else {
                    break num;
                }
            }
            Err(_) => {
                println!("这不是一个有效的数字，请重新输入：");
                continue;
            }
        }
    };

    println!(
        "这个单词的第{}个字符是{}",
        num,
        secret_word_chars[(num - 1) as usize]
    );
    let mut times = 5;
    let mut input = String::new();
    while times != 0 {
        println!("请输入你所猜测的单词：");
        io::stdin().read_line(&mut input).expect("无法读取输入");
        if input.trim().eq(&secret_word) {
            println!("恭喜你，猜对了！");
            break;
        } else {
            times -= 1;
            if times == 0 {
                println!("抱歉，您已经没有机会了，您输了！");
                break;
            }
            println!("抱歉，您猜错了，您还有{}次机会，请重新输入：", times);
            
        }
    }
    
}
