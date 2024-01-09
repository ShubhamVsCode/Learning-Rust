fn main() {
    let x = sum(10, 22);
    println!("{}", x);

    // If else statement
    // this should be a boolean value
    if x < 32 {
        println!("Less than 32");
    } else if x > 32 {
        println!("Bigger than 32");
    } else {
        println!("Equal to 32")
    }

    let y = if x < 50 { 20 } else { 30 }; // inline if else to define a variable

    println!("{}", y);

    // Loops
    loop_function();
    while_loop_function();
    for_n_loop_function()
}

fn sum(x: i32, y: i32) -> i32 {
    // two ways to return
    // return data type is important to specify

    // return x + y;
    x + y // without semi colon
}

fn loop_function() {
    let mut count = 0;

    loop {
        if count > 10 {
            break;
        }

        println!("{}", count);
        count += 1;
    }

    let result = loop {
        if count > 10 {
            break count; // return from loop with `break count`;
        }

        println!("{}", count);
        count += 1;
    };

    println!("Result : {}", result)
}

fn while_loop_function() {
    let mut count = 10;

    while count != 0 {
        println!("{}", count);
        count -= 1;
        /*
            10
            9
            8
            7
            6
            5
            4
            3
            2
            1
        */
    }

    println!("Hurrayyy!"); // Hurrayyy!
}

fn for_n_loop_function() {
    let numbers = [20, 30, 10, 100];

    for element in numbers.iter() {
        println!("Element {}", element)
        // Element 20
        // Element 30
        // Element 10
        // Element 100
    }

    for number in 1..5 {
        println!("{}", number);
        // 1
        // 2
        // 3
        // 4
    }
}
