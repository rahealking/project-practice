use project_practice::arr_impl::*;
fn main(){
    let mut hep:Heap=Heap::new();
    hep.insert(5);
    hep.insert(4);
    hep.insert(3);
    hep.insert(6);
    // println!("{:?}",hep.pop());
    println!("{:?}",hep);
    return;
}