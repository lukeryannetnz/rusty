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
    println!("{}", thumbsup)
}

fn printfloatingpoint() {
    let pi: f32 = 3.14;
    14;
    println!("{}", pi);
}

fn main() {
    printarray();
    printslice();
    printchar();
    printfloatingpoint();
    println!("done");
}