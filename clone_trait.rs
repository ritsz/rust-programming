
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

fn main()
{
    let string = "Hello World".to_string();
    let s2 = string.clone();
    println!("{} and {}", string , s2 );
}