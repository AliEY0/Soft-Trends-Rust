


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
}
