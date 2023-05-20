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
    let mut b=Tree::new();
    b=Tree::insert(b.clone(),7);
    b=Tree::insert(b.clone(),8);
    b=Tree::insert(b.clone(),9);
    b=Tree::insert(b.clone(),10);
    b=Tree::insert(b.clone(),11);
    b=Tree::insert(b.clone(),12);
    b=Tree::insert(b.clone(),13);
    Tree::show(Tree::merge_two_bst(a, b));
    // Tree::show(Tree::merge_two_bst(a,b));
    return;
}