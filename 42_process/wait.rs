// wait.rs
/*
When a Process goes out of scope, its drop method will wait until the child process finishes before releasing the resource.

$ rustc wait.rs && ./wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes

*/

use std::process::Command;

fn main() {
    let _process = Command::new("sleep").arg("5").spawn();

    println!("reached end of main");
}

