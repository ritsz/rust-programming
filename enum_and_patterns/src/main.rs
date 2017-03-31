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
}
