use crate::board::Board;

pub struct Point {
    pub row: usize,
    pub col: usize
}


pub struct Snake{
    pub body: Vec<Point>,
    pub direction: usize,


}
impl Snake{
    pub fn new(board_rows_size: usize, board_cols_size: usize) -> Snake{
        //let _size = board_rows_size * board_cols_size;
        let mut body : Vec<Point>   = Vec::new();      // = vec![Point; _size];
        
        let row_start = board_rows_size/2;
        let col_start = board_cols_size / 2;

        let length = 3;
        for i in 0..length{
            body.push(Point {row: row_start + i, col: col_start});
        }

        Snake {body: body, direction: 3}
    }

pub fn change_dir(&mut self, board: &mut Board, key_dir: usize) {

    let x = self.body.len();

    board.board[self.body[x-1].row][self.body[x-1].col] = '.';

    for i in (1..self.body.len()).rev() {
        self.body[i].row = self.body[i - 1].row;
        self.body[i].col = self.body[i - 1].col;
    }

    if self.direction == 1 && key_dir != 3 { 
        self.direction = key_dir; 
    }
    if self.direction == 3 && key_dir != 1 {
        self.direction = key_dir; 
    }
    if self.direction == 2 && key_dir != 4 { 
        self.direction = key_dir; 
    }
    if self.direction == 4 && key_dir != 2 { 
        self.direction = key_dir; 
    }

    if self.direction == 1 {
        self.body[0].row -= 1; 
    }
    if self.direction == 2 {
        self.body[0].col += 1;
    }
    if self.direction == 3 {
        self.body[0].row += 1;
    }
    if self.direction == 4 {
        self.body[0].col -= 1;
    }

    for point in &self.body {
            board.board[point.row][point.col] = '*';
        }
    }

    
}
