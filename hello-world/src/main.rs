// // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// // is a structure which contains a single `i32`.
// #[derive(Debug)]
// struct Structure(i32);

// // Put a `Structure` inside of the structure `Deep`. Make it printable
// // also.
// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

use std::env;

fn kriakaj() {
    println!("aeeej ja mam 54 rokov ale vy doplatite")
}

fn greet() {
    println!("achab");
}

fn fair_dice_roll() -> i32 {
    let feeling_lucky = true;

    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

// repl - read eval print tool
fn main() {

    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => println!("Error getting current working directory: {}", e),
    }

    let a = [1, 2, 3, 4, 5];
    println!("aha");

    let (some_char, some_int) = ('a', 1);

    assert_eq!(some_char, 'a');
    assert_eq!(some_int, 1);

    // pele()

    // statement semicolons span multiple lines
    let x = vec![1, 2, 3, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    // pair of prackets defines block / scope
    {
        // inside of block only lives as long as the block does.
        // does not influence the outside of the block
        let x = "in";
    }

    let x = { 42 };

    let x = {
        let y = 10;
        let z = 20;
        // tail of the block, what the block returns
        // omitting semicolon is same as return.
        z + y
    };

    let a = (10, 20);
    a.0;




    println!("{x}");
}