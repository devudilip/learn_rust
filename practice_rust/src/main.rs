// Loops

fn main() {

    println!("loop");
   let mut count = 0;
   'counting_Up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_Up;
            }

            remaining -= 1;
        }

        count += 1;
   }

   println!("End count = {}", count);


println!("while");
   let mut number = 3;

   while number != 0 {
        println!("number is : {}", number);
        number -= 1;
   }

   println!("Done dana done");

   println!("for");

   let number_list = [1,2,3];

   for n in number_list {
    println!("the number is : {}", n);
   }
}




// condition
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }


// fn main() {
//     println!("Hello, main function!");
//     another_function(5, 'r');
// }

// fn another_function(x: i32, unit_label: char) {
//     println!("Hello, another function!");
//     println!("Hello, The value of x : {}-{}", x,unit_label);
// }