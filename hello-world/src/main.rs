#![allow(dead_code)]
#![allow(unused_variables)]

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

fn cwd() { 
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => println!("Error getting current working directory: {}", e),
    }
}

fn tuples() {
    let (some_char, some_int) = ('a', 1);

    assert_eq!(some_char, 'a');
    assert_eq!(some_int, 1);
}

fn iterators() { 
    let x = vec![1, 2, 3, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
}

fn writing_contracts() {
    // writing contract how the world works
    #[derive(Debug)]
    enum Living { Alive, Dead }
    
    #[derive(Debug)]
    enum Planet {
        Mercury, Venus, Earth
    }

    #[derive(Debug)]
    struct Human {
        name: String,
        state: Living,
        home: Planet
    }

    let user = Human {
        name: "Tris".to_string(),
        state: Living::Alive,
        home: Planet::Earth
    };

    println!("{:?}", user);
}

// repl - read eval print tool
fn main() {

    println!("test");

    let number = 3;

    if number != 0 {
        println!("Num is something else than 0");
    }

    cwd();

    iterators();

    let number = 6;
    
    // executes the first if condition 
    if number % 4 == 0 {
        println!("num divisible by 4");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("{}", number);
    

    loop {
        println!("again");
    }

    // // statement semicolons span multiple lines
    // let x = vec![1, 2, 3, 5, 6, 7, 8]
    //     .iter()
    //     .map(|x| x + 3)
    //     .fold(0, |x, y| x + y);

    // // pair of prackets defines block / scope
    // {
    //     // inside of block only lives as long as the block does.
    //     // does not influence the outside of the block
    //     let x = "in";
    // }

    // let x = { 42 };

    // let x = {
    //     let y = 10;
    //     let z = 20;
    //     // tail of the block, what the block returns
    //     // omitting semicolon is same as return.
    //     z + y
    // };

    // let a = (10, 20);
    // a.0;

    // println!("{x}");
}