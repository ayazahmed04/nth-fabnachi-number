// use std::array;

fn main() {
    println!("Hello, world!");

    println!("Fibonacci nums is {} " , nth_fabonachi(15));

    // fabnacchi number 
}

fn nth_fabonachi (num : u32) -> u32  {

    if  num ==0 { 
        return num;
    } else if num ==  1 {
        return 1;
    }

    let mut fib = (0, 1);

    for _ in 2..=num {
        fib = (fib.1 , fib.0 + fib.1);
    }

     return fib.1 ;
}

