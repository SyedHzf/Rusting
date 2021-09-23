use std::rc::Rc;
enum List{
    Node(i32,Rc<List>),
    Nil
}
use List::*;

pub fn run() {

    let a = Rc::new(Node(3,Rc::new(Node(4,Rc::new(Nil)))));
    println!("refrence count of a {}",Rc::strong_count(&a));
    
    let c = Node(6,Rc::clone(&a));
    println!("refrence count of a {}",Rc::strong_count(&a));

    {
        let b = Node(5,Rc::clone(&a));
        println!("refrence count of a {}",Rc::strong_count(&a));
    }
    println!("refrence count of a {}",Rc::strong_count(&a));


} 