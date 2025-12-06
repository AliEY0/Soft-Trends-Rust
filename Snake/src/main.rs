mod board; 
mod snake; 
use snake::Snake; 
use board::Board; 
use std::io;
fn main() { 
    let size = 10; 
    let mut b = Board::new(size, size); 
    let mut s = Snake::new(size, size);


    b.draw_snake(&s);
    let mut input = String::new(); 
    while input != "x" {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input == "z\n" {
            s.change_dir(&mut b, 1);
        } else if input == "q\n" {
            s.change_dir(&mut b, 4);
        } else if input == "s\n" {
            s.change_dir(&mut b, 3);
        } else if input == "d\n" {
            s.change_dir(&mut b, 2);
        }
        if s.is_move_allowed(&b) == false {
            println!("U heeft verloren");
            input = "x".to_string();
            break;
        }else {
            b.draw_snake(&s);
        }
    }

    /*
    println!();
    s.change_dir(&mut b, 2);
    b.draw_snake(&s);

    println!();
    s.change_dir(&mut b, 2);
    b.draw_snake(&s);
    
    println!();
    s.change_dir(&mut b, 2);
    b.draw_snake(&s);
    
    println!();
    s.change_dir(&mut b, 2);
    if  s.is_move_allowed(&b) == false {
        println!("stop maar");
    } else {
        b.draw_snake(&s);
    }

    s.change_dir(&mut b, 2);
    if  s.is_move_allowed(&b) == false {
        println!("stop maar");
    } else {
        b.draw_snake(&s);
    }*/
    



}
