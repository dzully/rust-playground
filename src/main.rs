use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    // create loop to print message 10 times
    let mut key = 0;
    for _ in 0..10 {
        key += 1;
        if key == 5 {
            println!("{} is the magic number!", key);
        } else {
            println!("{} {}", message, key);
        }
    }

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_str(), width, &mut writer).unwrap();
}
