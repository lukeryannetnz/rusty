fn printarray() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in 0..array.len() {
        println!("{}", array[i]);
    }
}

fn printslice() {
    let array = [11, 12, 13, 14, 15, 16, 17, 18];
    let slice = &array[0..3]; // slices refer to a range. they are a pointer to the data with a length
    for i in 0..slice.len() {
        println!("{}", slice[i]);
    }
}

fn printchar() {
    let thumbsup: char = '👍'; //chars are 4 bytes so hold any unicode character
    println!("{}", thumbsup);
}

fn printfloatingpoint() {
    let pi: f32 = 3.14;
    println!("{}", pi);
}

fn printtuple() {
    let tuple: (i32, &str) = (13, "honeybadger");
    println!("({},{})", tuple.0, tuple.1);
}

fn printsinglevaluetuple() {
    let singlevaluetuple: (&str,) = ("single value tuple",); // the comma differentiates this from a regular value in parenthesis.
    println!("({},)", singlevaluetuple.0);
}

fn message() -> u32 {
    15 // note: no semi-colon here as this is an expression (returns a value), not a statement. Expressions don't end in semicolons in rust, only statements do.
}

fn main() {
    printarray();
    printslice();
    printchar();
    printfloatingpoint();
    printtuple();
    printsinglevaluetuple();
    let fnpointer = message;
    println!("This integer was printed via a variable binding function pointer! {}",
             fnpointer());
    println!("done");
}