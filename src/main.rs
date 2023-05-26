use project_practice::{arr_impl::*,*};
fn main(){
    let arr:[i32;13]=[1,6,4,8,7,3,11,31,55,22,21,23,14];
    println!("{}",Heap::kth_min_int(&arr,3));
    return;
}