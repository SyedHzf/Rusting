struct Smartpointer{
    data: String
}
impl Drop for Smartpointer{
    fn drop(&mut self) {
        println!("{}", self.data);
    }
}
pub fn run() {
    let c = Smartpointer{
        data : String::from("hello")
        
    };
    let d = Smartpointer{
        data : String::from("world")
    };

}