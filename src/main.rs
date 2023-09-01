// use project_practice::bgd::{AdjacencyList,finites};
use std::collections::btree_set::BTreeSet;
fn main(){
    let mut a: BTreeSet<i32>=BTreeSet::new();
    a.insert(21);
    a.insert(24);
    a.insert(22);
    a.insert(12);
    println!("{:?}",a.first());
    println!("{:#?}",a);
    a.pop_first();
    println!("{:#?}",a);

}