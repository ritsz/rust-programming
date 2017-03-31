use std::fs::File;
use std::fmt;

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
    GenericMessage,
    SpecialMessage,
}

/* Implement fmt::Debug on Message using Pattern Matching. */
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Message::Quit => write!(f, "Got a Quit message"),
            Message::ChangeColor(r,g,b) => write!(f, "Color : {} {} {}", r,g,b),
            Message::Move{ref x, ref y} => write!(f, "We are at point ({},{}).", x,y),
            /* 
               String doesn't implement clone, we can't move out String from self as we 
               have borrowed it, so we take a reference as well in pattern.
              */
            Message::Write(ref st) => write!(f, "__{}", st),
            Message::GenericMessage | Message::SpecialMessage => write!(f, "Don't know what to do"),
        }

    }
}

/*
 * error[E0573]: expected type, found variant `Message::Move`
   --> src/main.rs:10:20
      |
      10 | fn show_point(msg: Message::Move)
         |
   We need to start matching the types of enum. This is where pattern matching comes in.

fn show_point(msg: Message::Move)
{
    let Message::Move{ x, y } = msg;
    println!("{}, {} is the point", x, y);
}
*/

fn OptionOpenFile(name: String) -> Option<File> {
    let result = File::open(name);
    match result {
        Ok(file) => return Some(file),
        Err(error) => None
    }
}

fn main()
{
    let x : Message = Message::Move{x:3, y:10};
    
    /*
       WILL CAUSE ERROR
       show_point(x);
     */
    println!("{:?}", x);

    /* Converts string to Message::Write. Enum constructors can be used as a regular function that returns its type. */
    let m = Message::Write("What's up!".to_string());

    /* Example, we can use enum constructors as closures to change one type to annother */
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
    /* into_iter moves the data */
    println!("{:?}", v1);

    let x = 1;

    /* e binds to the value if it falls in the range */
    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
          _ => println!("anything"),
    }

    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: Some(ref a), .. }) => println!("{:?}", a),
        _ => {}
    }

    /* Pattern match on Result<File> */
    let result = File::open("DoesntExists.txt");
    match result {
        Ok(ref file) => println!("Opened the file {:?}", file),
        Err(ref some) => println!("Error {}", some),
    }

    let output = OptionOpenFile("DoesExist.txt".to_string());
    match output {
        Some(ref x) => println!("File is {:?}", x),
        None    => println!("There was some error. Consumed"),
    }
    /* If we don't take as ref in Some(), this will give error as moved data. */
    println!("{:?}", output);

    /* Use Option<T> to check if the match returned a valid data */
    enum Kingdom { Plant(u32, &'static str), Animal(u32, &'static str) }

    // A list of data to search through.
    let all_the_big_things = [
        Kingdom::Plant(250, "redwood"),
        Kingdom::Plant(230, "noble fir"),
        Kingdom::Plant(229, "sugar pine"),
        Kingdom::Animal(25, "blue whale"),
        Kingdom::Animal(19, "fin whale"),
        Kingdom::Animal(15, "north pacific right whale"),
        ];

    // We're going to search for the name of the biggest animal,
    // but to start with we've just got `None`.
    let mut name_of_biggest_animal = None;
    let mut size_of_biggest_animal = 0;
    for big_thing in &all_the_big_things {
        match *big_thing {
            Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
                // Now we've found the name of some big animal
                size_of_biggest_animal = size;
                name_of_biggest_animal = Some(name);
            }
            Kingdom::Animal(..) | Kingdom::Plant(..) => ()
        }
    }

    match name_of_biggest_animal {
        Some(name) => println!("the biggest animal is {}", name),
        None => println!("there are no animals :("),
    }
}
