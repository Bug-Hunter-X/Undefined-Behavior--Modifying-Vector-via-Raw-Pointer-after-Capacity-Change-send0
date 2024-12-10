fn main() {
    let mut v = vec![1, 2, 3];
    let first = v.get_mut(0);

    if let Some(element) = first {
        *element = 10;
    }
    println!("The first element is now: {}", v[0]);
} 