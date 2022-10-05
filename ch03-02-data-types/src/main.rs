use std::io;

fn main() {
    //     let value = "127".parse::<i8>().expect("parse error!");
    //     println!("{}",value);

    //     let value : i32 = "127".parse().expect("parse error!");
    //     println!("{}",value);

    //     let value = 2.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000;
    //     println!("{}",value);

    //     //add
    //     let _:u8 = 2+4;

    //     //sub
    //     let _:u8 = 8-3;

    //     //multip
    //     let _:u8 = 5*21;

    //     //div
    //     let _:u8 = 33/2;

    //     //remiander
    //     let _:u8 = 246%4;

    //     let _ =  true;
    //     let _ = false;

    // //     let value = 8i32;
    // //     println!("{}",value);
    // //     let value = 0b_1000 << 1;
    // //     println!("{}",value);
    // //     let value = 0b;
    // //     println!("{}",value);
    // //     let value = 0xffff;
    // //     println!("{}",value);
    //     let tup = (10,8.724,"abc");
    //     println!("{}",tup.0);
    //     println!("{}",tup.1);
    //     println!("{}",tup.2);

    //     let (age,money,str) = tup;
    //     println!("{}",age);
    //     println!("{}",money);
    //     println!("{}",str);

    //     println!("{:?}",tup);
    //     let _ = [1,2,3,4,5];

    let array = [1, 2, 3, 4, 5];

    println!("输入数组索引。");

    // get stdio device
    let stdin = io::stdin();

    // porpering a string as read buffer
    let mut buffer = String::new();
    //use stdio device read line and fill the data into string
    let _read_len = stdin.read_line(&mut buffer).expect("error");

    //parse the string value to usize
    let index: usize = buffer.trim().parse().expect("error");

    let index = buffer.trim().parse::<usize>().expect("error");
    //use index to get the element from the array
    let element = array[index];
    //print the element
    println!("Your selected value is {element}");
}
