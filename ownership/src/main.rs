fn main() {
    let x = 20;
    println!("{}", x); // 20
    let y = x;
    println!("{}", x); // 20 - COPY - because of primitive data type
    println!("{}", y); // 20

    let name = String::from("Shubham");

    let new_name = name; // not COPIED it is moved from name to new_name

    println!("{}", new_name); // Shubham

    // println!("{}", name); // Error

    // for copying we can use .clone()

    let very_new_name = new_name.clone();

    println!("{}", very_new_name); // Shubham
    println!("{}", new_name); // Shubham
}
