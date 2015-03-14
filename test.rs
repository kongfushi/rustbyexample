//
#![feature(str_words)]

fn main() {
    let s = "a b c";
    for w in s.words() {
        println!("{}",w);
    }
    
    let s = &8;
    let c: &u8 = s as &u8;
    println!("{}",c);
}


