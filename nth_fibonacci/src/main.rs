use std::io;

fn main() {

    // print msg
    println!("Welcome to nth Fibonacci generator!");
    println!("Enter a value for n: ");

    // make and read choice for n
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read input");

    // convert n from string to int
    let n: u32 = n.trim().parse().expect("Please input a number");

    // create and initialize variables
    let mut f1 = 0;
    let mut f2 = 1;
    let mut temp;

    // loop over n
    for _ in 1..n {

        // put f2 in f1 and sum of f1 and f2 in f2
        temp = f2;
        f2 = f1 + f2;
        f1 = temp;
    }

    // print output
    println!("{}th Fibonacci number is {}", n, f1);

}
