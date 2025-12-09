


fn fib(n: i64) -> i64{
    if n == 0 {
        return 0;
    }
    if n == 1{
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}


fn main(){
    use std::time::Instant;
    let now = Instant::now();
    let num = 50;
    for i in 1..(num+1) {
        let x =  fib(i);
        println!("Fib({}) is = {}",i, fib(i));
    }

    let elapsed = now.elapsed().as_secs_f64();
    println!("Totale tijd gespendeerd is: {:.6}", elapsed);

}
