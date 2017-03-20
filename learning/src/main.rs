
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
 */
fn main() {
    let (string, z)   = ("Hello World".to_string(), "Original String".to_string());
    /*
        mutable_borrow(&mut string, 1);
        error: cannot borrow immutable local variable `string` as mutable
     */
    mutable_move(string, 2);
    let mut mut_string:String = "Mutable String".to_string();
    mutable_borrow(&mut mut_string, 3);
    mutable_move(mut_string, 4);

    let borrow_string:&String = &z;
    immutable_borrow(borrow_string, 5);
    println!("6. {}", immutable_borrow_move(borrow_string, 6));

    let mut mut_string = "THIS IS MUTABLE".to_string();
    mutable_borrow_borrow(&mut mut_string, 7);

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
