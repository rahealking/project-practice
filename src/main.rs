use project_practice::{arr_impl::*,*};
fn main(){
    let mut hep:Heap=Heap::new(8);
    hep.insert(5);
    hep.insert(4);
    hep.insert(3);
    hep.insert(6);
    hep.end=7;hep.tree=vec![0,2,4,9,7,3,6,5];
    hep.heapify();
    // println!("{:?}",hep.pop());
    println!("{:?}",hep);
    return;
}