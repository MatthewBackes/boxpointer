use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl <T> MyBox<T> {

    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //Boxes provide the indirect so that the compiler can determine the size of the List enum
    //Boxes provide only indirection and heap allocation


    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //Asterisk dereferences the value of y here

    let x1 = 5;
    let y1 = MyBox::new(x);

    assert_eq!(5, x1);
    assert_eq!(5, *y1);
}
