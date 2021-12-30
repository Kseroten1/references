use std::io;

fn main() {
    let mut string = String::new();
    loop {
        io::stdin()
            .read_line(&mut string)
            .expect("connect keyboard you cunt");

        let first_space = string.find(' ').expect("give me a space you idiot");
        let last_space = string.rfind(' ').expect("give me a space you cunt");

        let killer = &string[..first_space];
        let dead = &string[last_space + 1..].trim();

        println!("'{}' was killed by '{}'", dead, killer);

        if killer == "janpaweldrugi" {
            break;
        }
        string.clear();
    }
}
