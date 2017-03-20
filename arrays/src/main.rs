
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

/* Method call syntax. use the impl keyword to implement a method for a data */
impl Circle {
    /* Associated functions are functions that don't take self references.
       They are like C++ static member functions.
       */
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x: x, y:y, radius:radius }
    }

    /* All impl functions borrow a self reference of the data they work on */ 
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    /* Implementations that return the struct can be used for method chaining. 
       This is returning a new instance, so original data is still unchanged
     */
    fn grow(&mut self, increment: f64) -> &mut Circle {
        self.radius = self.radius + increment;
        self
        //Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}


/* Generally use builder methods as builder pattern.*/

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    /* Could return CircleBuilder or Self. Self is shorter. */
    fn new () -> Self {
        CircleBuilder {x: 0.0, y:0.0, radius:0.0 }
    }

    /* Take a mutable self borrow and return the mutable self borrow so that mutating methods 
       can be chained to build the circle.
     */
    fn x(&mut self, x:f64) ->&mut CircleBuilder {
        self.x = x;
        self
    }

    /*
       IMPORTANT: self is just a syntactic sugar for self:Self where Self defines the current object type.
       Thus &mut self is self:&mut Self.
     */
    fn y(self: &mut Self, y:f64) ->&mut CircleBuilder {
        self.y = y;
        self
    }

    fn radius(&mut self, radius:f64) ->&mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }
}

struct Rectangle {
    length: f64,
    breadth: f64,
}

/* Define the traits */
trait Geometry {
    fn area(&self) -> f64;
    fn isSquare(&self) -> bool;
}

/* Implement the trait for a specific structure */
impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }

    fn isSquare(&self) -> bool {
        self.length == self.breadth
    }
}

/*
   Traits can also be used as a bound for types in generic function.
   print_area below is valid only for objects that implement the Geometry trait.
*/
fn print_area<T: Geometry> (shape: T)
{
    println!("The shape has an area {}", shape.area());
}


fn main() {
    let mut a = [0; 3];
    a[0] = 7;

    for x in a.iter() {
        println!("{}", x);
    }

    let mut c = Circle::new(0.0, 0.0, 1.0);
    println!("Created a circle of area {}", c.grow(1.0).area());
    println!("Created a circle of area {}", c.area());

    let buildCircle = CircleBuilder::new().x(0.0).y(1.0).radius(3.0).finalize();
    println!("Build a circle of area {}", buildCircle.area());

    let rect = Rectangle {length: 5.0, breadth: 5.0};
    print_area(rect);
}
