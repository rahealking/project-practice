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
    println!("{:#?}",Tree::bst_constructor_from_pre_order(&[3,1,0,2,5,4,6]));
    return;
}