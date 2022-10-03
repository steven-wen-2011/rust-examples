fn main() {
    // comments
    let mut x= 1;
    println!("The value of x is {x}");
    x = 2;
    println!("The value of x is {x}");
    const THE_NUMBER_OF_STUDENTS:u8 = 255;
    println!("THE NUMBER OF STUDENTS Are {THE_NUMBER_OF_STUDENTS}");


    // mutable can't change the type, but Shadowing can change types
    let i = "123";
    println!("{i}");
    let i = 1f32;
    let i = i + 2f32;
    {
        let i = i * 0.5;
        println!("{i}");
    }
    println!("{i}");
}
