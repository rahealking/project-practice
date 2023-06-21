use project_practice::{map::HashTable,*};
fn main(){
    let mut a:HashTable<i32,i32>=HashTable::new();
    a.insert(0,6);
    a.insert(1,8);
    a.insert(1,2);
    a.insert(2,3);
    println!("x:{}",a.get(intput()).unwrap());
    println!("{}",a);
}