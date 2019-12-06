use  std::io;

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let unumber: u32 = number.trim().parse()
        .expect("Please enter a valid number!");

    if is_armstrong_number(unumber){
        println!("This is an armstrong number!")
    }
    else {
        println!("This isn`t an armstrong number!")

    }

}

fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    num_str
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|digit| digit.pow(num_str.len() as u32))
        .sum::<u32>()
        == num
}
