


fn addOne(s: &String) -> String {
    let mut res = s.clone();
    res.push('1');

    res
}


fn main(){
    let mut s1 = String::from("Test ");
    
    let s2 = addOne(&s1);
    println!("S1 na het lenen {}", s1);
    println!("s2 = {}", s2);

    let s3 = s1;
    println!("{}", s1);


}
