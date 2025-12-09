//https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
//https://users.rust-lang.org/t/beginner-question-read-char-in-console-without-pressing-enter/18925



mod board;
mod snake;

use snake::Snake;
use board::Board;
use console::Term;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


fn greetings(s: String) -> String{
    let upper = s.to_uppercase();

    let newstr = format!("Welkom op mijn Snake Game {}", upper); 
    println!("{}", newstr);
    
    upper
}


fn main() {
    print!("Wat is je naam: ");
    io::stdout().flush().unwrap();

    let mut name = String::new(); 
    std::io::stdin().read_line(&mut name).unwrap();
    let nameupper = greetings(name);
    println!("{}", nameupper);
    
    let enter =  format!("Druk op enter om te starten {}", nameupper);
    println!("{}",enter);
    
    name.clear();

    std::io::stdin().read_line(&mut name).unwrap();
    println!("Lets Go {}", nameupper);

    let size = 30;
    let speed = 150;
    let mut b = Board::new(size, size);
    let mut s = Snake::new(size, size);

    b.draw_snake(&s);
    b.add_apple(&s);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let term = Term::stdout();
        loop {
            if let Ok(key) = term.read_char() {
                if tx.send(key).is_err() {
                    break; 
                }
            }
        }
    });

    let term = Term::stdout();
    loop {
        let input = rx.recv_timeout(Duration::from_millis(speed));
        match input {
            Ok(key) => {
                match key {
                    'z' => s.change_dir(&mut b, 1), 
                    'd' => s.change_dir(&mut b, 2), 
                    's' => s.change_dir(&mut b, 3), 
                    'q' => s.change_dir(&mut b, 4), 
                    'x' => break,                  
                    _ => {}
                }
            }
            Err(_) => {
                s.change_dir(&mut b, s.direction);
                //s.step_forward();
            }
        }

        if !s.is_move_allowed(&b) {
            term.clear_screen().unwrap();
            b.draw_snake(&s);
            println!("\nU heeft verloren");
            break;
        }
        term.clear_screen().unwrap();
        b.draw_snake(&s);
    }
}

