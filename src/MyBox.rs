use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    } 
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
pub fn run(){
    let x = 5;
    let y = MyBox::new(x); // Act as a &x(refrence)
    assert_eq!(5,x);
    assert_eq!(5,*y);// *(y.deref())
  // implicit deref coersions
    let mb = MyBox::new(String::from("hello world"));
    hello(&mb);// hello(*(&mb)[..]);
    //&MyBox -> &String -> &str


}
fn hello(s:&str){
    println!("{}",s);

}