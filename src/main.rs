use std::io;

fn main() {
    println!("Please enter a number!");

    let mut number_str = String::new();

    io::stdin().read_line(&mut number_str)
        .expect("Failed reading line");

    let _: u32 = number_str.trim().parse()
        .expect("please enter a valid number!");

    if is_armstrong_number(number_str.trim()){
        println!("It`s an armstrong number!")
    }
    else {
        println!("it isn`t an armstrong number");
    }

}

fn is_armstrong_number(number_str: &str) -> bool {
    let mut number: u32 = 0;
    let number_len: usize = number_str.len();

    for char_str in number_str.chars(){
        let char_u32 : u32 = char_str.to_digit(10).unwrap();
        number = number + u32::pow(char_u32, number_len as u32);
    }
    number == number_str.parse().unwrap()
}