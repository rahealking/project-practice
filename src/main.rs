use project_practice::*;
fn main(){
    let a=backtracing::n_queen(
        vec![
            vec!['o','o','o','o'],
            vec!['o','o','o','o'],
            vec!['o','o','o','o'],
            vec!['o','o','o','o']
        ]
    );for i in 0..a.len(){
        println!("{:?}",a[i]);
    }
}