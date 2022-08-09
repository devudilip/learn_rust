// Refactoring with Structs:

#[derive(Debug)]
struct Rectangle {
     width: u32,
    height: u32
}

fn main() {
     let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

println!("{:?}", rect1);
println!("{:#?}", rect1);

    println!("area of rectanlgle is : {}", area(rect1));
}

fn area(rect: Rectangle) -> u32 {
   rect.width * rect.height
}



// using tuples:

// fn main() {
//     let rect1 = (12,13);


//     println!("area of rectanlgle is : {}", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }



//  simple one
// fn main() {
//     let height = 12;
//     let width  = 13;



//     println!("area of rectanlgle is : {}", area(height, width));
// }

// fn area(height: u32, width: u32) -> u32 {
//     height * width
// }
