fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will panic if the vector is empty.  It is common to check the length first.
    let first = vec[0];
    println!("First element: {}", first);
}