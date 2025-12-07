mod board;
mod snake;
use snake::Snake;
use board::Board;
use console::Term;

fn main() {
    let size = 10;
    let mut b = Board::new(size, size);
    let mut s = Snake::new(size, size);

    let term = Term::stdout();

    b.draw_snake(&s);
    b.add_apple(&s);

    loop {
        let key = term.read_char().unwrap();

        match key {
            'z' => s.change_dir(&mut b, 1),
            'd' => s.change_dir(&mut b, 2),
            's' => s.change_dir(&mut b, 3),
            'q' => s.change_dir(&mut b, 4),
            'x' => break, 
            _ => {}
        }

        if !s.is_move_allowed(&b) {
            println!("U heeft verloren");
            break;
        } else {
            b.draw_snake(&s);
        }
        println!();
    }
}

