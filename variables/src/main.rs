fn main() {
    let x = 5;

     let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

     println!("The value of spaces is: {}", spaces);
}


// fn main() {
//     let mut x = 5;
//     println!("the value of x is: {}", x);
//     x = 6;
//     println!("the value of x is: {}", x);
// }
