use project_practice::*;
use serde::{Serializer,Serialize};
use serde_json::{self,value::RawValue};
fn main(){
    let mut a=vec![
        vec!['a','b','c','d'],
        vec!['e','f','g','h'],
        vec!['i','j','k','l'],
        vec!['m','n','o','p']
    ];
    let b=serde_json::to_string_pretty(&a);
    match b{
        Ok(c)=>{
            println!("{}",c);
            let mut e:Vec<Vec<char>>=serde_json::from_str(c.as_str()).unwrap();
            println!("{:?}",e[1]);
        },Err(d)=>{
            println!("{}",d);
        }
    }
}