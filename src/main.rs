use project_practice::{bgd::AdjacencyList};
fn main(){
    let mut a=AdjacencyList::new();
    a.add_edge(1,2,3,true);
    a.add_edge(1,3,2,true);
    a.add_edge(3,4,5,true);
    a.add_edge(2,4,7,true);
    a.add_edge(4,5,1,true);
    a.add_edge(4,6,1,true);
    a.add_edge(5,6,9,true);
    a.remove_edge(1,2,true);
    println!("{}",a);
    println!("---");
    // #a.remove_edge(1,3,true);
    a.remove_edge(3,4,true);
    a.remove_edge(2,4,true);
    a.remove_edge(4,5,true);
    a.remove_edge(4,6,true);
    // a.remove_edge(5,6,true);
    println!("{}",a);
}

