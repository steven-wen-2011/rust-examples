fn main() -> () {
    println!("Hello, world!");
    second_function();
    parameter_function(30000);
    parameters_function(10086, "g");
}
 // fn keyword + name + () parameters list  -> return alias  () is unit empty
fn second_function() -> () {
    println!("123");
}

fn parameter_function(x:i16){
    println!("{x}")
}

fn parameters_function(value:i16,unit:&str){
    println!("The value is {value}({unit})");

}
