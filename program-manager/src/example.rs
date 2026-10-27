pub fn example() 
{
    let testVar = "Yukari"; // Unchangable, fake const
    let mut changableVar = "is the best"; // changable

    println!("Hello, world, {} {}", testVar, changableVar);

    changableVar = "is awesome";

    println!("Hello, world, {} {}", testVar, changableVar);

    const STACK_SIZE : i32 = 64; // Needs type

    // Types arent neccessary here, but i think better to use them

    let num: i32 = 5; // i32 is signed, u32 is unsigned.
    let double: f64 = 5.5;
    let character: char = 'd';
    let boolean: bool = true;
    let string: &str = "hi";

    if num > 2 
    {
        println!("{} is bigger than 2", num);
    }
    else
    {
        println!("{} is 2 or lower", num);
    }

    // Ternary operator equivalent
    let time = 20;
    let greeting = if time < 18 { "Good day." } else { "Good evening." };

    // Switch equivalent
    let day = 6;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // Freaky loop
    let mut count = 0;
    loop {
        println!("Counting: {}", count);

        if count == 3 {
            break;
        }

        count += 1;
    }


    // Referencing, like pointers kinda
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref);

    let list = [1, 2, 3]; // Fixed size
    let vector = vec![1, 2, 3]; // Resizable

}


fn add(a: i32, b:i32) -> i32 
{
    return a + b;
}

// struct, like c

struct Cat {
  name: String,
  age: u32,
  breed: String,
  cute: bool
}

// Enums

enum Direction {
  Up,
  Down,
  Left,
  Right,
}
