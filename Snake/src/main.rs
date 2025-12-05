mod board;
mod snake;

use snake::Snake;


use board::Board;




fn main() {
    let size = 10;    
    let mut b = Board::new(size, size);
    //let mut Board;
    
    let s = Snake::new(size, size);

    b.draw_snake(&s);

    println!("Hello, world!");
}
