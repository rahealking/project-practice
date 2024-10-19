use super::{buf,bgd};
pub fn node(){
    let mut input_choice:String;
    println!("choose datatype\n[float/string]");
    input_choice=buf::input();
    match input_choice.as_str(){
        "float"=>{
            let mut head:bgd::node<f64>=bgd::Node::blank();
            let mut input_value:f64;
            loop{
                println!("choose action\n[exit,prepend,append,pop,shift,print,clear,insert,remove,find,replace,get]");
                input_choice=buf::input();
                match input_choice.as_str(){
                    "exit"=>{std::process::exit(0);}
                    ,"prepend"=>{
                        println!("[value]");
                        input_value=buf::input().parse::<f64>().unwrap();
                        head=bgd::Node::prepend(head,input_value.clone());
                    },"append"=>{
                        println!("[value]");
                        input_value=buf::input().parse::<f64>().unwrap();
                        head=bgd::Node::append(head,input_value.clone());
                    },"pop"=>{
                        head=bgd::Node::pop(head);
                    },"shift"=>{
                    },"print"=>{println!("{}",bgd::Node::display(head.clone()));}
                    ,"clear"=>{
                    },"insert"=>{
                    },"remove"=>{
                    },"find"=>{
                    },"replace"=>{
                    },"get"=>{
                    },_=>{println!("command unavailable");}
                }
            }
        },"string"=>{
            let mut head:bgd::node<String>=bgd::Node::blank();
            let mut input_value:String;
            loop{
                println!("choose action\n[exit,prepend,append,pop,shift,print,clear,insert,remove,find,replace,get]");
                input_choice=buf::input();
                match input_choice.as_str(){
                    "exit"=>{std::process::exit(0);}
                    ,"prepend"=>{
                        println!("[value]");
                        input_value=buf::input();
                        head=bgd::Node::prepend(head,input_value.clone());
                    },"append"=>{
                        println!("[value]");
                        input_value=buf::input();
                        head=bgd::Node::append(head,input_value.clone());
                    },"pop"=>{
                        println!("[value]");
                        input_value=buf::input();
                        head=bgd::Node::pop(head);
                    },"shift"=>{
                    },"print"=>{println!("{}",bgd::Node::display(head.clone()));}
                    ,"clear"=>{
                    },"insert"=>{
                    },"remove"=>{
                    },"find"=>{
                    },"replace"=>{
                    },"get"=>{
                    },_=>{println!("command unavailable");}
                }
            }
        },_=>{println!("datatype unsupported:[{:?}]",input_choice);}
    }
}