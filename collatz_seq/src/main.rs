use std :: env;
fn main() {
    let args : Vec<String> = env :: args().collect();
    let start = (&args[1]).parse::<usize>().unwrap();
    println!("{:#?}",collatz_seq(start));
}
fn collatz_next(n : usize) -> usize {
    if n % 2 == 0 {
        return n/2
    } else {
        return 3 * n + 1
    };
}
fn collatz_seq(n : usize) -> Vec<usize> {
    match n {
        4 => vec![4,2,1],
        _ => [vec! [n],collatz_seq(collatz_next(n))].concat(),
    }
}

