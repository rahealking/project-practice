pub use std;
pub mod buf{
    pub fn max<T:PartialOrd+Clone>(a:T,b:T)->T
    {return if a.clone()>b.clone(){a}else{b};}
    pub fn min<T:PartialOrd+Clone>(a:T,b:T)->T
    {return if a.clone()<b.clone(){a}else{b};}
    pub fn intput()->i32{
        let mut word:String=String::new();
        println!("[intput]");
        match std::io::stdin().read_line(&mut word){
            Ok(_)=>{
                word.pop();
                word=word.replace("\n","");
                word=word.replace("\r","");
                return word.parse().unwrap();
            },Err(e)=>{panic!("{}",e);}
        }
    }pub fn input()->String{
        let mut word:String=String::new();
        match std::io::stdin().read_line(&mut word){
            Ok(_)=>{
                word.pop();
                return word;
            },Err(e)=>{panic!("{}",e);}
        }
    }pub fn signum<T:PartialEq+PartialOrd+Clone>(a:T,b:T)->i32{
        if a.clone()==b.clone(){return 0;}
        else if a.clone()>b.clone(){return 1;}
        else if a.clone()<b.clone(){return -1;}
        else{panic!("unexpected case");}
    }
}pub mod bgd{// basic generic data-structures
    pub trait finites
    where Self:Default{
        const MAX:Self;
        const Min:Self;
    }impl finites for i32{
        const MAX:Self=i32::MAX;
        const Min:Self=i32::MIN;
    }#[derive(Debug,Clone,Copy)]
    pub struct Position{pub y:usize,pub x:usize,pub z:usize}
    impl Position{pub fn new(y:usize,x:usize,z:usize)->Position{return Position{y,x,z};}}
    #[derive(Debug)]
    pub struct Node<T>{
        pub value:T,
        pub prev:std::rc::Rc<Option<std::cell::RefCell<Node<T>>>>,
        pub next:std::rc::Rc<Option<std::cell::RefCell<Node<T>>>>
    }#[allow(non_camel_case_types)]
    pub type node<T>=std::rc::Rc<Option<std::cell::RefCell<Node<T>>>>;
    pub struct ListIter<T>{
        current:node<T>
    }impl<T>Iterator for ListIter<T>{
        type Item=node<T>;
        fn next(&mut self)->Option<Self::Item>{
            let temp:Self::Item=self.current.clone();
            match self.current.clone().as_ref(){
                Some(n)=>{
                    self.current=n.borrow().next.clone();
                    return Some(temp);
                },None=>{return None;}
            }
        }
    }impl<T>Node<T>{
        pub fn blank()->node<T>{return std::rc::Rc::new(None);}
        pub fn new(value:T)->node<T>
        {return std::rc::Rc::new(Some(std::cell::RefCell::new(Node{value,prev:Node::blank(),next:Node::blank()})));}
        pub fn push(head:node<T>,value:T)->node<T>{
            let temp:node<T>;
            let mut prev:node<T>=head.clone();
            let mut next:node<T>=head.clone();
            loop{
                match prev.clone().as_ref(){
                    Some(n)=>{
                        if std::rc::Rc::ptr_eq(&n.borrow().prev,&next){
                            match next.as_ref(){
                                Some(o)=>{
                                    temp=Node::new(value);
                                    match temp.as_ref(){
                                        Some(d)=>{
                                            d.borrow_mut().next=prev.clone();
                                            n.borrow_mut().prev=temp.clone();
                                            o.borrow_mut().next=temp.clone();
                                            d.borrow_mut().prev=next.clone();
                                            return head;
                                        },None=>{panic!("unexpected None");}
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }else{
                            if match n.borrow().prev.as_ref(){Some(_)=>{true},None=>{false}}
                            {prev=n.borrow().prev.clone();}else{prev=Node::blank();}
                        }
                    },None=>{}
                }match next.clone().as_ref(){
                    Some(n)=>{
                        if std::rc::Rc::ptr_eq(&n.borrow().next,&prev){
                            match prev.as_ref(){
                                Some(o)=>{
                                    temp=Node::new(value);
                                    match temp.as_ref(){
                                        Some(d)=>{
                                            d.borrow_mut().next=prev.clone();
                                            o.borrow_mut().prev=temp.clone();
                                            n.borrow_mut().next=temp.clone();
                                            d.borrow_mut().prev=next.clone();
                                            return head;
                                        },None=>{panic!("unexpected None");}
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }else{
                            if match n.borrow().prev.as_ref(){Some(_)=>{true},None=>{false}}
                            {next=n.borrow().next.clone();}else{
                                temp=Node::new(value);
                                match temp.as_ref(){
                                    Some(o)=>{
                                        o.borrow_mut().prev=next.clone();
                                        n.borrow_mut().next=temp.clone();
                                        return head;
                                    },None=>{panic!("unexpected None");}
                                }
                            }
                        }
                    },None=>{return Node::new(value);}
                }
            }
        }pub fn remove(head:node<T>,target:T)->node<T>where T:PartialEq{
            match head.as_ref(){
                Some(n)=>{
                    if n.borrow().value==target{return n.borrow().next.clone();}
                    else{
                        let temp:node<T>=Node::remove(n.borrow().next.clone(),target);
                        n.borrow_mut().next=temp.clone();
                        match temp.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().prev=head.clone();
                            },None=>{}
                        }return head;
                    }
                },None=>{return head;}
            }
        }pub fn iter(n:node<T>)->ListIter<T>{
            return ListIter{current:n};
        }pub fn display(mut temp:node<T>)->String where T:Clone+std::fmt::Display{
            let mut output:String=String::new();
            loop{
                match temp.clone().as_ref(){
                    Some(n)=>{
                        output.extend(format!("({})->",n.borrow().value.clone()).chars());
                        temp=n.borrow().next.clone();
                    },None=>{
                        output.pop();output.pop();
                        break;
                    }
                }
            }output.push(']');
            output.insert(0,'[');
            return output;
        }
    }#[derive(Debug,Clone)]
    pub struct Pair<K,V>{pub kye:K,pub value:V}
    impl<K,V>Pair<K,V>{pub fn new(k:K,v:V)->Pair<K,V>{return Pair{kye:k,value:v};}}
    impl<K:std::fmt::Display,V:std::fmt::Display>std::fmt::Display for Pair<K,V>{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return std::fmt::write(
                f,format_args!("{}:{}",self.kye,self.value)
            );
        }
    }
    pub mod map{
        use super::{Node,node,Pair};
        pub trait Hash{
            fn hashcode(&self)->usize;
            fn compressor(value:usize,bound:usize)->usize{
                return if value>bound{((value/bound+bound)/(value%bound))%bound}else{value};
            }
        }#[derive(Debug)]
        pub struct HashTable<K:Hash,V>{
            pub bucket:Vec<node<Pair<K,V>>>
        }pub struct TableIter<'a,K:Hash+Clone,V:Clone>{table:&'a HashTable<K,V>,i:usize,j:node<Pair<K,V>>}
        impl<'a,K:Hash+Clone,V:Clone>Iterator for TableIter<'a,K,V>{
            type Item=(K,V);
            fn next(&mut self)->Option<Self::Item>{
                let value:(K,V);
                match self.j.clone().as_ref(){
                    Some(n)=>{
                        value=(n.borrow().value.kye.clone(),n.borrow().value.value.clone());
                        self.j=n.borrow().next.clone();
                        return Some(value);
                    },None=>{
                        if self.i<self.table.bucket.len(){
                            match self.table.bucket[self.i].as_ref(){
                                Some(n)=>{
                                    value=(n.borrow().value.kye.clone(),n.borrow().value.value.clone());
                                    self.j=n.borrow().next.clone();
                                    self.i+=1;
                                    return Some(value);
                                },None=>{
                                    self.i+=1;
                                    return self.next();
                                }
                            }
                        }else{return None;}
                    }
                }
            }
        }impl<K:Hash,V>HashTable<K,V>{
            pub fn new()->HashTable<K,V>{return HashTable{bucket:(vec![Node::blank();100])};}
            pub fn get(&self,kye:K)->Option<V>where K:PartialEq,V:Clone{
                let hash:usize=<K as Hash>::compressor(kye.hashcode(),self.bucket.len());
                let mut temp:node<Pair<K,V>>=self.bucket.as_slice()[hash].clone();
                loop{
                    match temp.clone().as_ref(){
                        Some(n)=>{
                            if n.borrow().value.kye==kye{
                                return Some(n.borrow().value.value.clone());
                            }else{temp=n.borrow().next.clone();}
                        },None=>{return None;}
                    }
                }
            }pub fn remove(&mut self,kye:K)where K:PartialEq{
                let hash:usize=<K as Hash>::compressor(kye.hashcode(),self.bucket.len());
                let arr:&mut[node<Pair<K,V>>]=self.bucket.as_mut_slice();
                let mut temp:node<Pair<K,V>>=arr[hash].clone();
                let mut previous:node<Pair<K,V>>=temp.clone();
                loop{
                    match temp.clone().as_ref(){
                        Some(n)=>{
                            if n.borrow().value.kye==kye{
                                temp=n.borrow().next.clone();
                                match previous.as_ref(){
                                    Some(o)=>{
                                        o.borrow_mut().next=temp.clone();
                                        return;
                                    },None=>{panic!("unexpected None");}
                                }
                            }else{
                                previous=temp.clone();
                                temp=n.borrow().next.clone();
                            }
                        },None=>{return;}
                    }
                }
            }pub fn modify(&mut self,kye:K,value:V)where K:PartialEq{
                let hash:usize=<K as Hash>::compressor(kye.hashcode(),self.bucket.len());
                let mut temp:node<Pair<K,V>>=self.bucket.as_slice()[hash].clone();
                loop{
                    match temp.clone().as_ref(){
                        Some(n)=>{
                            if n.borrow().value.kye==kye{
                                n.borrow_mut().value.value=value;
                                return;
                            }else{temp=n.borrow().next.clone();}
                        },None=>{return;}
                    }
                }
            }pub fn insert(&mut self,kye:K,value:V)where K:PartialEq+Clone,V:Clone{
                match self.get(kye.clone()){
                    Some(_)=>{
                        self.modify(kye,value);
                    },None=>{
                        let hash:usize=<K as Hash>::compressor(kye.hashcode(),self.bucket.len());
                        let arr:&mut[node<Pair<K,V>>]=self.bucket.as_mut_slice();
                        arr[hash]=Node::push(arr[hash].clone(),Pair::new(kye,value));
                    }
                }
            }pub fn or_insert(&mut self,kye:K,value:V)where K:PartialEq+Clone,V:Clone{
                match self.get(kye.clone()){
                    Some(_)=>{return;},None=>{self.insert(kye,value);return;}
                }
            }pub fn iter<'a>(&'a self)->TableIter<'a,K,V>where K:Clone,V:Clone{
                return TableIter{table:self,i:0,j:Node::blank()};
            }
        }impl<K:Hash+std::fmt::Display,V:std::fmt::Display>std::fmt::Display for HashTable<K,V>{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut output=String::new();
                let mut i:usize=0;let mut temp:node<Pair<K,V>>;
                let arr:&[node<Pair<K,V>>]=self.bucket.as_slice();
                output.push_str("[map]");
                loop{
                    if i<arr.len(){
                        match arr[i].as_ref(){
                            Some(n)=>{
                                output.extend(
                                    format!(
                                        "\n[{}]-[({}:{})",
                                        i,n.borrow().value.kye,n.borrow().value.value
                                    ).chars()
                                );temp=n.borrow().next.clone();
                                loop{
                                    match temp.clone().as_ref(){
                                        Some(o)=>{
                                            output.extend(
                                                format!(
                                                    ",({}:{})",
                                                    o.borrow().value.value,o.borrow().value.kye
                                                ).chars()
                                            );temp=o.borrow().next.clone();
                                        },None=>{break;}
                                    }
                                }output.push(']');
                            },None=>{}
                        }i+=1;
                    }else{break;}
                }
                return std::fmt::write(f,format_args!("{}",output));
            }
        }
        #[derive(Debug,Clone)]
        pub struct Trie{
            value:char,terminal:bool,pub list:Vec<Option<Trie>>
        }impl Trie{
            pub fn new(c:char)->Trie{
                return Trie{
                    value:c,list:vec![None;26],terminal:false
                };
            }pub fn insert(&mut self,data:String){
                if data.len()>0{
                    let mut temp=self;let mut i:usize;
                    let mut a;
                    for ch in data.chars(){
                        i=ch as usize-97;
                        a = &mut temp.list[i];
                        match a{
                            Some(t)=>{
                                temp=t;
                            },None=>{
                                *a=Some(Trie::new(ch));
                                match a{
                                    Some(r)=>{
                                        temp=r;
                                    },None=>{panic!("unexpected None");}
                                }
                            }
                        }
                    }temp.terminal=true;
                }
            }pub fn remove(&mut self,target:String){
                if target.len()>0{
                    let mut temp:&mut Trie=self;
                    let mut i:usize;
                    for ch in target.chars(){
                        i=ch as usize-97;
                        match&mut temp.list[i]{
                            Some(t)=>{
                                temp=t;
                            },None=>{return;}
                        }
                    }temp.terminal=false;
                }
            }pub fn search(&self,target:String)->bool{
                let mut temp:&Trie=self;let mut i:usize;
                for ch in target.chars(){
                    i=ch as usize-97;match&temp.list[i]{
                        Some(t)=>{
                            temp=t;
                        },None=>{return false;}
                    }
                }return temp.terminal;
            }pub fn words(&self,mut prefix:String)->Vec<String>{
                let mut collection:Vec<String>=Vec::new();
                if self.terminal{collection.push(prefix.clone());}
                for i in 0..26{
                    match&self.list[i]{
                        Some(t)=>{
                            prefix.push(t.value);
                            collection.append(&mut t.words(prefix.clone()));
                            prefix.pop();
                        },None=>{}
                    }
                }return collection;
            }
        }
    }use map::{Hash,HashTable};
    pub struct Queue<T>{head:node<T>,tail:node<T>}
    impl<T>Queue<T>{
        pub fn new()->Queue<T>{return Queue{head:Node::blank(),tail:Node::blank()};}
        pub fn enqueue(&mut self,value:T){
            let v:node<T>=Node::new(value);
            match self.tail.clone().as_ref(){
                Some(n)=>{
                    n.borrow_mut().next=v.clone();
                    match v.as_ref(){
                        Some(o)=>{
                            o.borrow_mut().prev=self.tail.clone();
                        },None=>{panic!("unexpected None");}
                    }self.tail=v.clone();
                },None=>{
                    self.head=v.clone();
                    self.tail=self.head.clone();
                }
            }
        }pub fn dequeue(&mut self)->node<T>{
            let value:node<T>=self.head.clone();
            match self.head.clone().as_ref(){
                Some(n)=>{
                    self.head=n.borrow().next.clone();
                    n.borrow_mut().next=Node::blank();
                    match self.head.as_ref(){
                        Some(o)=>{
                            o.borrow_mut().prev=Node::blank();
                        },None=>{self.tail=self.head.clone();}
                    }return value;
                },None=>{return value;}
            }
        }
    }#[derive(Debug)]
    pub struct Stack<T>{top:node<T>}
    impl<T>Stack<T>{
        pub fn new()->Stack<T>{return Stack{top:Node::blank()};}
        pub fn push(&mut self,value:T){
            let v:node<T>=Node::new(value);
            match v.as_ref(){
                Some(n)=>{
                    n.borrow_mut().next=self.top.clone();
                    self.top=v.clone();
                },None=>{panic!("unexpected None");}
            }
        }pub fn pop(&mut self)->node<T>{
            let value:node<T>=self.top.clone();
            match self.top.clone().as_ref(){
                Some(n)=>{
                    self.top=n.borrow().next.clone();
                },None=>{}
            }return value;
        }pub fn query(&self,target:T)->bool where T:Clone+PartialEq{
            let mut temp:node<T>=self.top.clone();
            loop{
                match temp.clone().as_ref(){
                    Some(n)=>{
                        if n.borrow().value==target.clone()
                        {return true;}else
                        {temp=n.borrow().next.clone();}
                    },None=>{return false;}
                }
            }
        }
    }impl<T:std::fmt::Display+Clone>std::fmt::Display for Stack<T>{
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
            let mut output:String=String::new();
            let mut temp:node<T>=self.top.clone();
            output.push(' ');
            output.push('_');
            output.push('\n');
            loop{
                match temp.clone().as_ref(){
                    Some(n)=>{
                        output.extend(format!("|{}|\n",n.borrow().value.clone()).chars());
                        temp=n.borrow().next.clone();
                    },None=>{break;}
                }
            }
            output.push(' ');
            output.push('-');
            output.push('\n');
            return write!(f,"{}",output);
        }
    }pub struct AdjacencyList<V:Hash+Clone+PartialEq,W:Clone>
    {pub list:HashTable<V,node<Pair<V,W>>>}
    impl<V:Hash+Clone+PartialEq,W:Clone>std::fmt::Display for AdjacencyList<V,W>
    where V:std::fmt::Display,W:std::fmt::Display{
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
            let mut output:String=String::new();
            for(k,v)in self.list.iter(){
                output.extend(format!("[{}]:",k.clone()).chars());
                output.extend(Node::display(v.clone()).chars());
                output.push(';');
                output.push('\n');
            }output.pop();
            return write!(f,"{}",output);
        }
    }impl<V:Hash+Clone+PartialEq,W:Clone>AdjacencyList<V,W>{
        pub fn new()->AdjacencyList<V,W>{return AdjacencyList{list:HashTable::new()}}
        pub fn add_edge(&mut self,a:V,b:V,weight:W,directed:bool){
            match self.list.get(a.clone()){
                Some(vortex)=>{
                    let mut temp:node<Pair<V,W>>=vortex;
                    loop{
                        match temp.clone().as_ref(){
                            Some(n)=>{
                                if n.borrow().value.kye==b.clone(){
                                    n.borrow_mut().value.value=weight.clone();
                                    break;
                                }else{
                                    if match n.borrow().next.as_ref(){Some(_)=>{true},None=>{false}}
                                    {temp=n.borrow().next.clone();}else{
                                        n.borrow_mut().next=Node::new
                                        (Pair::new(b.clone(),weight.clone()));
                                        break;
                                    }
                                }
                            },None=>{
                                self.list.modify(a.clone(),Node::new(Pair::new(b.clone(),weight.clone())));
                                break;
                            }
                        }
                    }
                },None=>{self.list.insert(a.clone(),Node::new(Pair::new(b.clone(),weight.clone())));}
            }if!directed{
                match self.list.get(b.clone()){
                    Some(vortex)=>{
                        let mut temp:node<Pair<V,W>>=vortex;
                        loop{
                            match temp.clone().as_ref(){
                                Some(n)=>{
                                    if n.borrow().value.kye==a.clone(){
                                        n.borrow_mut().value.value=weight.clone();
                                        break;
                                    }else{
                                        if match n.borrow().next.as_ref(){Some(_)=>{true},None=>{false}}{
                                            temp=n.borrow().next.clone();
                                        }else{
                                            n.borrow_mut().next=Node::new
                                            (Pair::new(a.clone(),weight.clone()));
                                            break;
                                        }
                                    }
                                },None=>{
                                    self.list.modify(b.clone(),Node::new(Pair::new(a.clone(),weight.clone())));
                                    break;
                                }
                            }
                        }
                    },None=>{self.list.insert(b.clone(),Node::new(Pair::new(a.clone(),weight.clone())));}
                }
            }else{self.list.or_insert(b.clone(),Node::blank());}
        }pub fn remove_node(head:node<Pair<V,W>>,target:V)->node<Pair<V,W>>{
            match head.as_ref(){
                Some(n)=>{
                    let temp:node<Pair<V,W>>;
                    if n.borrow().value.kye.clone()==target.clone()
                    {return n.borrow().next.clone();}else{
                        temp=Self::remove_node(n.borrow().next.clone(),target);
                        n.borrow_mut().next=temp;return head;
                    }
                },None=>{return head;}
            }
        }pub fn remove_edge(&mut self,a:V,b:V,directed:bool){
            match self.list.get(a.clone()){
                Some(edges)=>{
                    self.list.modify(a.clone(),Self::remove_node(edges,b.clone()));
                },None=>{}
            }if!directed{
                match self.list.get(b.clone()){
                    Some(edges)=>{
                        self.list.modify(b.clone(),Self::remove_node(edges,a.clone()));
                    },None=>{}
                }
            }
        }
    }impl<V:Hash+Clone+PartialEq,W:Clone>AdjacencyList<V,W>{//
        pub fn topological_traversal(&self,mut syscall:Stack<V>,mut visited:HashTable<V,()>,index:V)
        ->(Stack<V>,HashTable<V,()>){
            match visited.get(index.clone()){
                Some(_)=>{return(syscall,visited);}
                ,None=>{
                    visited.insert(index.clone(),());
                    match self.list.get(index.clone()){
                        Some(edges)=>{
                            for edge in Node::iter(edges){
                                match edge.as_ref(){
                                    Some(n)=>{
                                        (syscall,visited)=self.topological_traversal
                                        (syscall,visited,n.borrow().value.kye.clone());
                                    },None=>{panic!("unexpected None");}
                                }
                            }
                        },None=>{}
                    }syscall.push(index);
                    return(syscall,visited);
                }
            }
        }pub fn shortest_path(&self,a:V,b:V)->W
        where W:finites+PartialEq+std::ops::Add<Output=W>+PartialOrd
        ,V:std::fmt::Display{
            if a.clone()==b.clone()
            {return<W as Default>::default();}
            else{
                match self.list.get(a.clone()){
                    Some(edges)=>{
                        let mut temp:W;let mut ans:W=<W as finites>::MAX;
                        for edge in Node::iter(edges){
                            match edge.as_ref(){
                                Some(n)=>{
                                    temp=self.shortest_path
                                    (n.borrow().value.kye.clone(),b.clone());if temp<
                                    <W as finites>::MAX&&temp.clone()+n.borrow().value.value.clone()<ans
                                    .clone(){ans=temp+n.borrow().value.value.clone();}
                                },None=>{panic!("unexpected None");}
                            }
                        }return ans;
                    },None=>{return<W as finites>::MAX;}
                }
            }
        }pub fn shortest_paths(&self,a:V)->HashTable<V,W>where W:finites
        +PartialEq+std::ops::Add<Output=W>+PartialOrd
        +std::fmt::Display,V:std::fmt::Display{
            let mut order:Stack<V>;(order,_)=self.topological_traversal
            (Stack::new(),HashTable::new(),a.clone());
            let mut distance:HashTable<V,W>=HashTable::new();
            for(vortex,_)in self.list.iter(){
                distance.insert(
                    vortex.clone(),
                    self.shortest_path(a.clone(),vortex.clone())
                );
            }
            loop{
                // match order.pop().as_ref(){
                //     Some(n)=>{
                //         distance.insert(
                //             n.borrow().value.clone(),
                //             self.shortest_path(a.clone(),n.borrow().value.clone())
                //         );
                //     },None=>{break;}
                // }
                break;
            }return distance;
        }
    }
}
// ---map---
// --backtracing--
pub mod backtracing{
    use super::bgd::{Node,node,Position};
    use super::buf;
    pub fn rat_in_maze(
        maze:node<&[&[u8]]>,map:node<&mut[&mut[u8]]>,// single node (not list);
        index:Position,mut path:String
    )->Vec<String>{
        // [d][l][r][u];
        //  v << >>  ^ ;
        match maze.as_ref(){
            Some(n)=>{
                match map.as_ref(){
                    Some(o)=>{
                        if n.borrow().value.len()==o.borrow().value.len()&&
                        n.borrow().value[0].len()==o.borrow().value[0].len()
                        &&n.borrow().value.len()>1&&o.borrow().value[0].len()>1{
                            let mut ans:Vec<String>=Vec::new();
                            let visited:char;
                            if!(n.borrow().value.len()-1==index.y&&o.borrow().value[0].len()-1==index.x){
                                // no path
                                o.borrow_mut().value[index.y][index.x]=1;
                                if index.y<n.borrow().value.len()-1&&
                                n.borrow().value[index.y+1][index.x]==1
                                &&o.borrow().value[index.y+1][index.x]==0{// v/d
                                    path.push('d');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y+1,index.x,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                    visited='d';
                                }else if index.x>0&&
                                n.borrow().value[index.y][index.x-1]==1
                                &&o.borrow().value[index.y][index.x-1]==0{// <</l
                                    path.push('l');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y,index.x-1,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                    visited='l';
                                }else if index.x<n.borrow().value[0].len()-1
                                &&n.borrow().value[index.y][index.x+1]==1
                                &&o.borrow().value[index.y][index.x+1]==0{// >>/r
                                    path.push('r');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y,index.x+1,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                    visited='r';
                                }else if index.y>0&&
                                n.borrow().value[index.y-1][index.x]==1
                                &&o.borrow().value[index.y-1][index.x]==0{// ^/u
                                    path.push('u');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y-1,index.x,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                    visited='u';
                                }else{
                                    o.borrow_mut().value[index.y][index.x]=0;
                                    return ans;
                                }// post recursion block
                                if index.y<n.borrow().value.len()-1&&
                                n.borrow().value[index.y+1][index.x]==1
                                &&o.borrow().value[index.y+1][index.x]==0
                                &&visited!='d'{// v/d
                                    path.push('d');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y+1,index.x,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                }else if index.x>0&&
                                n.borrow().value[index.y][index.x-1]==1
                                &&o.borrow().value[index.y][index.x-1]==0
                                &&visited!='l'{// <</l
                                    path.push('l');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y,index.x-1,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                }else if index.x<n.borrow().value[0].len()-1
                                &&n.borrow().value[index.y][index.x+1]==1
                                &&o.borrow().value[index.y][index.x+1]==0
                                &&visited!='r'{// >>/r
                                    path.push('r');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y,index.x+1,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                }else if index.y>0&&
                                n.borrow().value[index.y-1][index.x]==1
                                &&o.borrow().value[index.y-1][index.x]==0
                                &&visited!='u'{// ^/u
                                    path.push('u');
                                    ans.append(
                                        &mut rat_in_maze(
                                            maze.clone(),
                                            map.clone(),
                                            Position::new(index.y-1,index.x,0),
                                            path.clone()
                                        )
                                    );
                                    path.pop();
                                }o.borrow_mut().value[index.y][index.x]=0;
                                return ans;
                            }else{
                                // at destination
                                ans.push(path.clone());
                                return ans;
                            }
                        }else{return Vec::new();}
                    },None=>{return Vec::new();}
                }
            },None=>{return Vec::new();}
        }
    }pub fn n_queen(board:Vec<Vec<char>>)->Vec<Vec<char>>{
        if board.len()>1&&board[0].len()==board.len(){
            let pieces=|map:Vec<Vec<char>>|->usize{
                let mut  count:usize=0;
                for i in 0..map.len(){
                    for j in 0..map[1].len(){
                        if map[i][j]=='q'{count+=1;}
                    }
                };return count;
            };if pieces(board.clone())<board[0].len(){
                let mut map:Vec<Vec<char>>;
                for i in 0..board.len(){
                    for j in 0..board[0].len(){
                        if board[i][j]=='o'{
                            map=board.clone();
                            map[i][j]='q';
                            // marking paths
                            for l in 0..map.len()-(map.len()-i){// ^
                                if map[l][j]!='q'{
                                    map[l][j]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in 0..buf::min
                            (i as i32,(map.len()-1-j)as i32)
                            as usize{// ^>>
                                if map[i-l-1][j+l+1]!='q'{
                                    map[i-l-1][j+l+1]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in j+1..map.len(){// >>
                                if map[i][l]!='q'{
                                    map[i][l]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in 0..buf::min
                            ((map.len()-i-1)as i32,(map.len()-1-j)as i32)
                            as usize{// >>v
                                if map[i+l+1][j+l+1]!='q'{
                                    map[i+l+1][j+l+1]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in i+1..map.len(){// v
                                if map[l][j]!='q'{
                                    map[l][j]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in 0..buf::min(
                                j as i32,
                                (map.len()-i-1)as i32
                            ) as usize{// v<<
                                if map[i+l+1][j-l-1]!='q'{
                                    map[i+l+1][j-l-1]='x';
                                }else{
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in 0..j{
                                if map[i][l]!='q'{// <<
                                    map[i][l]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }for l in 0..buf::min(i as i32,j as i32) as usize{// <<^
                                if map[i-l-1][j-l-1]!='q'{
                                    map[i-l-1][j-l-1]='x';
                                }else{
                                    println!("position:({}x{})",i,j);
                                    for l in 0..map.len(){
                                        println!("{:?}",map[l]);
                                    }
                                    panic!("unexpected position");
                                }
                            }map=n_queen(map);
                            if!(pieces(map.clone())<board[0].len()){return map;}
                        }
                    }
                }
            }return board;
        }else{return Vec::new();}
    }pub fn sudoku(board:[[u8;9];9],c:node<i128>)->[[u8;9];9]{
        let mut vertical:node<u8>;
        let mut horizontal:node<u8>;
        let mut block:node<u8>;
        let mut map:[[u8;9];9]=board.clone();
        let commons
        =|mut l1:node<u8>,mut l2:node<u8>,mut l3:node<u8>|->node<u8>{
            let mut ans:node<u8>=Node::blank();loop{
                match l1.clone().as_ref(){
                    Some(n)=>{
                        match l2.clone().as_ref(){
                            Some(o)=>{
                                match l3.clone().as_ref(){
                                    Some(d)=>{
                                        if n.borrow().value==o.borrow().value&&o.borrow().value==d.borrow().value{
                                            ans=Node::push(ans,n.borrow().value);
                                        }if o.borrow().value<d.borrow().value{
                                            if o.borrow().value<n.borrow().value{
                                                l2=o.borrow().next.clone();
                                            }else{
                                                l1=n.borrow().next.clone();
                                            }
                                        }else{
                                            if d.borrow().value<n.borrow().value{
                                                l3=d.borrow().next.clone();
                                            }else{
                                                l1=n.borrow().next.clone();
                                            }
                                        }
                                    },None=>{return ans;}
                                }
                            },None=>{return ans;}
                        }
                    },None=>{return ans;}
                }
            }
        };for i in 0..9{
            for j in 0..9{
                if board[i][j]==0{
                    block=Node::blank();
                    vertical=Node::blank();
                    horizontal=Node::blank();
                    for v in 1..10{
                        block=Node::push(block.clone(),v);
                        vertical=Node::push(vertical.clone(),v);
                        horizontal=Node::push(horizontal.clone(),v);
                    }// vertical
                    for l in 0..i{vertical=Node::remove(vertical.clone(),board[l][j]);}
                    for l in i+1..9{vertical=Node::remove(vertical.clone(),board[l][j]);}
                    // horizontal
                    for l in 0..j{horizontal=Node::remove(horizontal.clone(),board[i][l]);}
                    for l in j+1..9{horizontal=Node::remove(horizontal.clone(),board[i][l]);}
                    // block
                    if i<3{
                        if j<3{
                            for x in 0..3{
                                for y in 0..3{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<6{
                            for x in 0..3{
                                for y in 3..6{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<9{
                            for x in 0..3{
                                for y in 6..9{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else{panic!("unexpected position");}
                    }else if i<6{
                        if j<3{
                            for x in 3..6{
                                for y in 0..3{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<6{
                            for x in 3..6{
                                for y in 3..6{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<9{
                            for x in 3..6{
                                for y in 6..9{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else{panic!("unexpected position");}
                    }else if i<9{
                        if j<3{
                            for x in 6..9{
                                for y in 0..3{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<6{
                            for x in 6..9{
                                for y in 3..6{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else if j<9{
                            for x in 6..9{
                                for y in 6..9{block=Node::remove(block.clone(),board[x][y]);}
                            }
                        }else{panic!("unexpected position");}
                    }else{
                        panic!("unexpected position");
                    }for n in Node::iter
                    (commons(vertical.clone(),horizontal.clone(),block.clone())){
                        match n.as_ref(){
                            Some(o)=>{
                                map=board.clone();
                                map[i][j]=o.borrow().value;
                                map=sudoku(map,c.clone());
                                if ||->bool{
                                    for x in 0..9{
                                        for y in 0..9{
                                            if&map[x][y]==&0{return false;}
                                        }
                                    }return true;
                                }(){return map;}
                            },None=>{panic!("unexpected None");}
                        }
                    }return map;
                }
            }
        }return board;
    }
}
// --graph --
