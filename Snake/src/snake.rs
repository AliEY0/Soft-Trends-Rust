

pub struct Point {
    pub row: usize,
    pub col: usize
}


pub struct Snake{
    pub body: Vec<Point>


}
impl Snake{
    pub fn new(board_rows_size: usize, board_cols_size: usize) -> Snake{
        //let _size = board_rows_size * board_cols_size;
        let mut body : Vec<Point>   = Vec::new();      // = vec![Point; _size];
        
        let row_start = board_rows_size/2;
        let col_start = board_cols_size / 2;

        let length = 5;
        for i in 0..length{
            body.push(Point {row: row_start, col: col_start + i});
        }



        Snake {body: body}
    }
    
}
