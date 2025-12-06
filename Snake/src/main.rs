mod board; 
mod snake; 
use snake::Snake; 
use board::Board; 
fn main() { 
    let size = 10; 
    let mut b = Board::new(size, size); 
    let mut s = Snake::new(size, size);


    b.draw_snake(&s);

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
    }
    
}
