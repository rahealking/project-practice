// use project_practice::*;
fn main(){
    println!("[debug output]");
    let o=[0,1,2,3,4,5,6];
    let n=&o[..];
    let a=&n[..3];
    let b=&n[3+1..];
    println!("o:{:?}",o);
    println!("n:{:?}",n);
    println!("a:{:?}",a);
    println!("b:{:?}",b);
    println!("n.len:{:#?}",n.len());
    println!("o[2]:({})",o[2]);
    println!("n[2]:({})",n[2]);
    println!("a[2]:({})",a[2]);
    println!("b[2]:({})",b[2]);
    println!("n[7..]-size:({:?})",&n[7..].len());
    return;
}