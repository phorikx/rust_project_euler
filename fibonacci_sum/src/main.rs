fn main() {
    let vec = fibonacci_vector_builder(4_000_000);
    let mut total = 0;
    for i in vec {
        if i%2 == 0 {
            total += i;
        }
    }
    println!("total is {total}");
}

fn fibonacci_vector_builder(max_number: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(1);
    let mut n = 1;
    let mut previous = 1;
    while n <= max_number {
        vec.push(n);
        let storage = n;
        n = n + previous;
        previous = storage;
    }
    vec
}
