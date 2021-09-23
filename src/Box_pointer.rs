#[derive(Debug)]

enum List{
    Node(i32,Box<List>),
    Nil
}
use List::*;

pub fn run(){
    let list = Box::new(
        Node(2,Box::new(
            Node(3,Box::new(
                Node(4,Box::new(
                    Nil)
                   )
                )
            )
        )
    )
);
    println!("{:?}",list);
}