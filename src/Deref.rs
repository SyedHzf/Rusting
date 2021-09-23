
pub fn run(){
    let x = 5;
    let y = Box::new(x); // Act as a &x(refrence)
    assert_eq!(5,x);
    assert_eq!(5,*y);

}