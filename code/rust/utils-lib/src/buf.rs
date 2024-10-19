pub fn max<T:PartialOrd+Clone>(a:T,b:T)->T
{return if a.clone()>b.clone(){a}else{b};}
pub fn min<T:PartialOrd+Clone>(a:T,b:T)->T
{return if a.clone()<b.clone(){a}else{b};}
pub fn intput()->i32{
    let mut word:String=String::new();
    println!("[intput]");
    match std::io::stdin().read_line(&mut word){
        Ok(_)=>{
            word.pop();word.pop();
            word=word.replace("\n","");
            word=word.replace("\r","");
            return word.parse().unwrap();
        },Err(e)=>{panic!("{}",e);}
    }
}pub fn input()->String{
    let mut word:String=String::new();
    match std::io::stdin().read_line(&mut word){
        Ok(_)=>{
            word.pop();word.pop();
            return word;
        },Err(e)=>{panic!("{}",e);}
    }
}pub fn signum<T:PartialEq+PartialOrd+Clone>(a:T,b:T)->i32{
    if a.clone()==b.clone(){return 0;}
    else if a.clone()>b.clone(){return 1;}
    else if a.clone()<b.clone(){return -1;} 
    else{panic!("unexpected case");}
}