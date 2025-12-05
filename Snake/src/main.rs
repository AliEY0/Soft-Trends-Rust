mod board;
use board::Board;


fn main() {
    
    let b = Board::new(4,4);
    //let mut Board;
    
    for _i in 0..b.row_size {
        for _j in 0..b.col_size{
            print!("{}",b.board[_i][_j]);
            //println!("hallo");
        }
        println!();
    }



    println!("Hello, world!");
}
