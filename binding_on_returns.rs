
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

/* Can be called for T == i32 as ConvertTo is implemented */
fn normal<T> (x: &T) -> i64 where T : ConvertTo<i64> {
    x.convert()
}

/* This adds a bound using the return type. Method works only
   for data types to which i32 can be converted to.
  */
fn inverse<T>(x: &i32) -> T where i32 : ConvertTo<T> {
    x.convert()
}

fn main() {
    let mut x: i32 = 11;
    println!("{:?}", normal(&x));
    x = 42;
    println!("{:?}", inverse(&x));
}
