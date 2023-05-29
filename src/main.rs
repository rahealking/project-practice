use project_practice::*;
fn main(){
    let mut a=Tree::new();
    a=Heap::insert(a,intput());
    a=Heap::insert(a,intput());
    a=Heap::insert(a,intput());
    a=Heap::insert(a,intput());
    a=Heap::insert(a,intput());
    // a=Heap::insert(a,intput());
    // a=Heap::insert(a,intput());
    println!("{:#?}",a);
    a=Heap::pop(a.clone());
    println!("{:#?}",a);
    a=Heap::pop(a.clone());
    println!("{:#?}",a);
    a=Heap::pop(a.clone());
    println!("{:#?}",a);
    a=Tree::flatten_binary_search_tree_in_place(a.clone());
    Tree::list_checker(a.clone());
    return;
}
