//mod snake;
//use snake::Snake;
use crate::snake::Snake;
use rand::Rng;

pub struct Board {
    pub row_size: usize,
    pub col_size: usize,
    pub board: Vec<Vec<char>>,
}


impl Board{
    pub fn new(row: usize, col: usize) -> Board{
        let board: Vec<Vec<char>> = vec![vec!['.'; col];row]; 
        Board {row_size: row,col_size: col, board: board}
    }

    pub fn draw_snake(&mut self, snake: &Snake) {
        for i in 0..self.row_size {
            for j in 0..self.col_size {
                if self.board[i][j] == '*' || self.board[i][j] == 'X' {
                    self.board[i][j] = '.';
                }
            }
        }
        for point in &snake.body {
            self.board[point.row][point.col] = '*';
        }
        let head = &snake.body[0];
        self.board[head.row][head.col] = 'X';
        for i in 0..self.row_size {
            for j in 0..self.col_size {
                print!("{}", self.board[i][j]);
            }
            println!();
        }
    }


    pub fn add_apple(&mut self, snake: &Snake) {

        let mut rng = rand::thread_rng();

        loop {
            let rnd_row = rng.gen_range(0..self.row_size);
            let rnd_col = rng.gen_range(0..self.col_size);

            if self.board[rnd_row][rnd_col] == '.' {
                self.board[rnd_row][rnd_col] = 'O';
                break;
            }
        }
    }



}
