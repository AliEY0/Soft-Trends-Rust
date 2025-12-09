fn main() {
    let src = String::from("Hello BufferOverflow"); 
    let mut dest = String::from("kort");    
    
     unsafe {
        for i in 0..src.len() {
            dest.as_mut_vec()[i] = src.as_bytes()[i];
        }
    }
   
    println!("{}", dest);
}

