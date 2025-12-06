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
        for point in &snake.body {
            self.board[point.row][point.col]  = '*';
        }
        self.board[snake.body[0].row][snake.body[0].col] = 'X';
        /*for i in 0..snake.body.len(){
            board.board[snake.body[i].row][snake.body[i].col] = '*';
            
        }*/

        for i in 0..self.row_size{
            for j in 0..self.col_size {
                print!("{}",self.board[i][j]);                
            }
            println!();
        }
        //Board {row_size: board.row_size, col_size: board.col_size, board: board}
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
