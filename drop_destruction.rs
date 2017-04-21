
struct Data {
    name:   String,
    age :   u32,
}

impl Drop for Data {
    fn drop(&mut self) {
        /* Brutal */
        println!("Name {} of age {} is about to die", self.name, self.age);
    }
}

fn main()
{
    let x = Data {name: "John Goodman".to_string(), age: 24 };
    /* 
       explicit destructor calls not allowed 
       x.drop();
    */
}
