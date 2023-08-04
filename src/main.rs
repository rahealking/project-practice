use project_practice::{graph::*,map::HashTable,bgd::*};
fn main(){
    let mut a: AdjacencyList<i32>=AdjacencyList::new();
    let mut visited0:HashTable<i32,()>=HashTable::new();
    let mut syscall0:Stack<i32>=Stack::new();
    a.add_edge(1,2,true);
    a.add_edge(2,3,true);
    a.add_edge(2,4,true);
    a.add_edge(3,7,true);
    a.add_edge(3,8,true);
    a.add_edge(4,5,true);
    a.add_edge(4,6,true);
    a.add_edge(5,6,true);
    a.add_edge(6,4,true);
    a.add_edge(8,7,true);
    for (kye ,_) in a.list.iter(){
        match visited0.get(kye){
            Some(_)=>{
            },None=>{
                (visited0,syscall0)=a.topological_traversal(kye, visited0, syscall0);
            }
        }
    }
    println!("{:#?}",syscall0);
    println!("{a}");
}