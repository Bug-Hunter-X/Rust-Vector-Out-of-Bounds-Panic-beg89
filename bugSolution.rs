fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Check if the vector is empty before accessing elements.
    if !vec.is_empty() {
        let first = vec[0];
        println!("First element: {}", first);
    } else {
        println!("Vector is empty!");
    }
} 