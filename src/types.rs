/*
primitive types--
integers (u,i)8,(u,i)16,(u,i)32,(u,i)64,(u,i)128 (numbers of bits they take in memory)
Floats f(32,64)
Boolean (bool)
Charaters (char)
Tuples
Arrays 
*/

pub fn run (){
let x = 2;
let y: i32 = 22323;
let z: f32 = 2.1221;
let is_active = true;
let is_greater: bool = 10<5;
let a1 = 'a';
let face = '\u{1F600}'; // uni

println!("{:?   }",(x,y,z,is_active,is_greater,a1,face))
}