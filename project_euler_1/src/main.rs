fn main() {
    let mut total = 0;
    for number in 1..1000 {
        if number % 3 == 0 || number % 5 == 0 {
            total += number;
        }
    }
    println!("Total is {total}");
}
