#[allow(unused_variable)]

pub fn factorl(number : u32) -> u32 {
    let mut num = 1;
    if number == 1 || number == 0{
        return num;
    }else{
    for i in (1..number+1).rev(){
        num*=i;
    }
}
    num
    
}
