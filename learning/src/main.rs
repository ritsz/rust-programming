
/*
   0. Arguments during sending and function parameters must match.
      That is how move and borrows are decided. Variables can be
      mutable or immutable. Immutable variables can be moved and 
      borrowed immutably.
   1. Immutable variables CAN'T have mutable borrows
   2. Immutable variables CAN have mutable ownership moves
   3. Mutable variables can have mutable borrows.
   4. Mutable variables can have mutable moves.
   5. Borrowed variables can be borrowed again.
   6. Borrowed variables can't be moved by borrower. Moves can only happen from original owner.
   7. Borrowed varibles can be further borrowed.
   8. Single reference borrow and return doesn't need reference lifetimes
   9. Multiple refernces borrow require a lifetime parameter.
 */
fn main() {
    let (string, zString)   = ("Hello World".to_string(), "Original String".to_string());
    /*
        mutable_borrow(&mut string, 1);
        error: cannot borrow immutable local variable `string` as mutable
     */
    mutable_move(string, 2);
    let mut mut_string:String = "Mutable String".to_string();
    mutable_borrow(&mut mut_string, 3);
    mutable_move(mut_string, 4);

    let borrow_string:&String = &zString;
    immutable_borrow(borrow_string, 5);
    println!("6. {}", immutable_borrow_move(borrow_string, 6));

    let mut mut_string = "THIS IS MUTABLE".to_string();
    mutable_borrow_borrow(&mut mut_string, 7);

    borrow_single_ref(&mut_string);
    
    mut_string = "THIS IS FOR LIFETIME CHECK".to_string();
    borrow_two_ref(&mut_string, &9);

    test_nested_lifetimes();
}

fn mutable_borrow(x: &mut String, index: u32) {
    println!("{}. {}", index, x);
}

fn mutable_move (mut x: String, index: u32) {
    x = "FOO".to_string();
    println!("{}. {}", index, x);
}

fn immutable_borrow(x: &String, index: u32) {
    println!("{}. {}", index, x);
}

fn immutable_move(x:String, index: u32) {
    println!("{}. {}", index, x);
}

fn immutable_borrow_move(x: &String, index: u32) -> String {
    /*
        *x
        error[E0507]: cannot move out of borrowed content
        x was borrowed, you don't own it. You can't move it to someone else.
     */
    "Dummy".to_string()
}

fn mutable_borrow_borrow(x: &mut String, index: u32) -> &mut String {
    println!("{}. {}", index, x);
    x
}

/* Since these refernces will have the same lifetime, we don't need a lifetime parameter. */
fn borrow_single_ref(x: &String) -> &String {
    let some = "Hello".to_string();
    println!("8. {}", x);
    x
}

fn borrow_two_ref<'a, 'b>(x: &'a String, index: &'b u32) -> &'a String {
    println!("{}. {}", index, x);
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_nested_lifetimes() {
    let x = "Longish string";

    {
        let y = "Small";
        let result = longest(x,y);
        println!("10. {:?}. /*Generic lifetimes get set to the smallest lifetime*/", result);
    }

    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
     //string2 doesn't live long enough. It could still be borrowed here but is out of scope
    println!("The longest string is {}", result);
    */
}
