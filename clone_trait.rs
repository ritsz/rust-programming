
use std::fmt;
/* Copy can be derived only by data that can implement implicit copy.
   Can't have String, we used &str here.

   That results in another issue then. Since str is a reference and we don't own it,
   we need to tell how long the reference needs to stay around.

   We use 'a to specify that reference stays till the lifetime of the struct.
   This is called lifetime specifier.

 */
#[derive(Copy)]
struct Data<'a> {
    name : &'a str,
    age : i32,
}

/*
  Lifetime elision allows lifetimes to be calculated implicitly.
  eg:
  1. impl Reader for BufReader instead of impl<'a> Reader for BufReader<'a>
  2. fn get_str(s: &str) -> &str instead of fn get_str<'a>(s: &'a str') -> &'a str;

  Since Data needs to have lifetime 'a, we need to specify impl lifetime as 'a as well
  or else we get :=> use of undeclared lifetime name `'a` for Data
 */
impl<'a> Clone for Data<'a> {
    fn clone(&self) -> Data<'a>
    {
        *self
    }
}

impl<'a> fmt::Debug for Data<'a> {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Name {} of age {}", self.name, self.age)
    }
}


struct Foo<'a> {
    x: &'a i32,
}

fn main()
{
    let string = "Hello World".to_string();
    let s2 = string.clone();
    println!("{} and {}", string , s2 );

    let obj = Data{name: "John", age : 42};
    let mut cloned_obj = obj.clone();
    cloned_obj.name = "Jane";
    cloned_obj.age = 43;
    println!("{:?} \n{:?}", obj, cloned_obj);

/* LIFETIMES:
    let x;                    // -+ `x` comes into scope.
                              //  |
    {                         //  |
        let y = &5;           // ---+ `y` comes into scope.
        let f = Foo { x: y }; // ---+ `f` comes into scope.
        x = &f.x;             //  | | This causes an error.
    }                         // ---+ `f` and y go out of scope.
                              //  |
    // Error: borrowed value does not live long enough. Lifetime of f.x is much smaller that x
    println!("{}", x);
  */
}
