use project_practice::*;
fn main(){
    let a:Vec<Vec<u8>>=vec![
        vec![1,0,0,0],
        vec![1,1,0,0],
        vec![1,1,0,0],
        vec![0,1,1,1]
    ];
    let mut b1:Vec<u8>=vec![0,0,0,0];
    let mut b2:Vec<u8>=vec![0,0,0,0];
    let mut b3:Vec<u8>=vec![0,0,0,0];
    let mut b4:Vec<u8>=vec![0,0,0,0];
    let mut c=Vec::new();
    for i in 0..a.len(){
        c.push(a[i].as_slice());
    }
    let d=c.as_slice();
    let mut e=vec![
        b1.as_mut_slice(),
        b2.as_mut_slice(),
        b3.as_mut_slice(),
        b4.as_mut_slice()
    ];
    let mut f=backtracing::rat_in_maze(map::Node::new(d),map::Node::new(e.as_mut_slice()),backtracing::Position::new(0,0,0),String::new());
    println!("{:#?}",f);
}
// implemented function rat_in_maze at backtracing