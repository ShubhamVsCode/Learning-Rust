fn main() {
    let mut x = 5; // we should give mut if we want to mutate any variable
    println!("{}", x);

    x = 6;
    println!("{}", x);

    const SUB: u32 = 10; // best practice for const to use CAPITAL LATTERS
                         // it should have type annotation
                         // we can't declare this as a mut

    let y = 6;
    let y = "six"; // shadowing

    // DATA TYPES

    // 1. Scaler Types
    // - intergers
    // - floating-point numbers
    // - boolean
    // - character

    // 2. Compound Types
    // - tupels -- fixed sized array of related data but can be different data types
    // - array  -- fixed length, if we want to have dynmic length of array we should use vector
    //
    //
    //

    // Tupels
    let tup = ("Shubham", 20);

    let (name, age) = tup; // destructuring

    let person_name = tup.0; // dot notation

    println!("{} : {}", name, age); // Shubham : 20
    println!("{}", person_name); // Shubham

    // Array
    let arr = [20, 30, 50, 100];
    println!("{:?}", arr); // [20, 30, 50, 100]

    let first = arr[0];
    // let last = arr[5]; // index out of bounds: the length is 4 but the index is 5

    let allTwoArr = [2; 10];
    println!("{:?}", allTwoArr); // [2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
}
