use std::io;

fn main() {
    // let mut x = 7;
    // println!("value of x: {x}");
    // x = 10;
    // println!("value of x: {x}");
    fiboncai(); 
}

fn fiboncai(){
    println!("Please Enter The Number to Print Fibonaci");
    let mut number = String::new();
    let mut a:u32;
    let mut b:u32;
    
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read the number");

    let number:u32 = match number.trim().parse() {
        Ok(num)=>num,
        Err(_)=>23
    };
    
    println!("you have entered: {number}")


}