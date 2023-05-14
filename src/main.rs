use project_practice::*;
fn main(){
    let mut a=Tree::new();
    a=Tree::insert(a.clone(),0);
    a=Tree::insert(a.clone(),1);
    a=Tree::insert(a.clone(),2);
    a=Tree::insert(a.clone(),3);
    a=Tree::insert(a.clone(),4);
    a=Tree::insert(a.clone(),5);
    a=Tree::insert(a.clone(),6);
    Tree::show(a.clone());
    a=Tree::flatten_binary_search_tree(a.clone());
    // Tree::show(a.clone());
    Tree::list_checker(a.clone());
    return;
}