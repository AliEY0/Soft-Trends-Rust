


fn isPrime(n: i32) -> bool {
    if n < 2{
        return false;
    }
   
    if n % 2 == 0{
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0{
            return false;
        }
        i += 2;
    }

    return true;
}


fn main(){
    use std::time::Instant;
    let now = Instant::now();
    
    let n = 20000000;
    let mut totaal = 1;
    println!("Aantal priemgetallen onder {}:", n);
    for i in 3..n{
        if isPrime(i) == true{
            totaal += 1;
            //println!("{} is een priemgetal",i);
        }
    }
    
    println!("Er zijn onder {}, {} priemgetallen.", n, totaal);
    let elapsed = now.elapsed().as_secs_f64();
    println!("Totale tijd gespendeerd is: {:.6}", elapsed);

}
