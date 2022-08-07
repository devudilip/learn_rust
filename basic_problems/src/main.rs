fn main() {
  //   try out Fibonacci using rust
  let number = 6;
  let fib_number = fib(number);
  println!("fib number for {} is {}", number, fib_number);
}

// copid version best one


fn fib(num: u32) -> u32 {
  match num {
      0 => 0,
      1 => 1,
      _ => fib(num - 1) + fib(num - 2)
  }
}


// Better version
// fn fib(num: i32) -> i32 {
//    if num <= 1 {
//        num
//    } else {
//     fib(num-1) + fib(num-2)
//    }
// }

// First try

// fn fib(num: i32) -> i32 {
//    let fib_number;
//    if num <= 1 {
//        fib_number = num;
//    } else {
//        fib_number = fib(num-1) + fib(num-2);
//    }
//    fib_number
// }