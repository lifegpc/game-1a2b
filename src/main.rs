mod game;
use game::Game;
use std::io;

fn new_game() -> Game {
    loop {
        println!("How many digits do you want to play with?");
        let mut input = String::new();
        let re = io::stdin().read_line(&mut input);
        match re {
            Ok(_) => {}
            Err(..)   => {
                continue;
            }
        }
        let trimed = input.trim();
        if trimed.len() == 0 {
            continue;
        }
        let i = trimed.parse::<i8>();
        match i {
            Ok(_) => {}
            Err(..) => {
                println!("this was not an integer: {}", trimed);
                continue;
            }
        };
        let r = Game::new(i.unwrap());
        match r {
            Some(_) => {}
            None    => {
                println!("This interger must be between 1 to 10.");
                continue
            }
        }
        return r.unwrap();
    }
}

fn main() {
    let mut r = new_game();
    r.start();
}
