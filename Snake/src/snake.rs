use crate::board::Board;
#[derive(Clone)]
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

        Snake {body: body, direction: 1}
    }
     pub fn is_move_allowed(&self, board: &Board) -> bool{
        let rows = board.row_size;
        let cols = board.col_size;
        //let x = self.body.len() - 1;
        let x = 0;
        if self.body[x].row == 999 || self.body[x].col == 999 {
            return false;
        }
        if self.body[x].col >= cols || self.body[x].row >= rows {
            return false;
        }
    
        return true;

    }
    pub fn eat_apple(&mut self, board: &mut Board, new_row: usize, new_col: usize){
         let last = &self.body[self.body.len() - 1];
        self.body.push(Point { row: last.row, col: last.col });
    }
    pub fn change_dir(&mut self, board: &mut Board, key_dir: usize)  {

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
        let head = &self.body[0];
        let mut new_row = self.body[0].row as isize;
        let mut new_col = self.body[0].col as isize;        
        
        if self.direction == 1 { new_row -= 1;}
        if self.direction == 2 { new_col += 1;}
        if self.direction == 3 { new_row += 1;}
        if self.direction == 4 { new_col -= 1;}

        if new_row < 0 || new_col < 0 || new_row as usize >= board.row_size || new_col as usize >= board.col_size {
            self.body[0].row = 999;
            self.body[0].col = 999;
            return;
        }

        let old_tail = self.body[self.body.len() - 1].clone();
        self.body.insert(0, Point { row: new_row as usize, col: new_col as usize });
            

        if board.board[new_row as usize][new_col as usize] != 'O' {
            self.body.pop();
            board.board[old_tail.row][old_tail.col] = '.';
        } else {
            board.add_apple(self);
        }
        

    }
}
