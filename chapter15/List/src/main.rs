enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
