
fn main() {
    let value = "127".parse::<i8>().expect("parse error!");
    println!("{}",value);

    
    let value : i32 = "127".parse().expect("parse error!");
    println!("{}",value);

    let value = 2.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;
    println!("{}",value);

    //add
    let _:u8 = 2+4;

    //sub
    let _:u8 = 8-3;

    //multip
    let _:u8 = 5*21;

    //div
    let _:u8 = 33/2;

    //remiander
    let _:u8 = 246%4;

    let _ =  true;
    let _ = false;


//     let value = 8i32;
//     println!("{}",value);
//     let value = 0b_1000 << 1;
//     println!("{}",value);
//     let value = 0b;
//     println!("{}",value);
//     let value = 0xffff;
//     println!("{}",value);
}
