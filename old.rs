pub fn sudoku(board:&mut[[u8;9];9]){
    let mut vertical:super::map::node<u8>;
    let mut horizontal:super::map::node<u8>;
    let mut block:super::map::node<u8>;
    let commons
    =|mut l1:super::map::node<u8>,mut l2:super::map::node<u8>,mut l3:super::map::node<u8>|->super::map::node<u8>{
        let mut ans:super::map::node<u8>=super::map::Node::blank();loop{
            match l1.clone().as_ref(){
                Some(n)=>{
                    match l2.clone().as_ref(){
                        Some(o)=>{
                            match l3.clone().as_ref(){
                                Some(d)=>{
                                    if n.borrow().value==o.borrow().value&&o.borrow().value==d.borrow().value{
                                        ans=super::map::Node::push(ans,n.borrow().value);
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
                block=super::map::Node::blank();
                vertical=super::map::Node::blank();
                horizontal=super::map::Node::blank();
                for v in 1..10{
                    block=super::map::Node::push(block.clone(),v);
                    vertical=super::map::Node::push(vertical.clone(),v);
                    horizontal=super::map::Node::push(horizontal.clone(),v);
                }// vertical
                for l in 0..i{vertical=super::map::Node::remove(vertical.clone(),board[l][j]);}
                for l in i+1..9{vertical=super::map::Node::remove(vertical.clone(),board[l][j]);}
                // horizontal
                for l in 0..j{horizontal=super::map::Node::remove(horizontal.clone(),board[i][l]);}
                for l in j+1..9{horizontal=super::map::Node::remove(horizontal.clone(),board[i][l]);}
                // block
                if i<3{
                    if j<3{
                        for x in 0..3{
                            for y in 0..3{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<6{
                        for x in 0..3{
                            for y in 3..6{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<9{
                        for x in 0..3{
                            for y in 6..9{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else{panic!("unexpected position");}
                }else if i<6{
                    if j<3{
                        for x in 3..6{
                            for y in 0..3{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<6{
                        for x in 3..6{
                            for y in 3..6{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<9{
                        for x in 3..6{
                            for y in 6..9{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else{panic!("unexpected position");}
                }else if i<9{
                    if j<3{
                        for x in 6..9{
                            for y in 0..3{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<6{
                        for x in 6..9{
                            for y in 3..6{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else if j<9{
                        for x in 6..9{
                            for y in 6..9{block=super::map::Node::remove(block.clone(),board[x][y]);}
                        }
                    }else{panic!("unexpected position");}
                }else{
                    panic!("unexpected position");
                }match commons(vertical.clone(),horizontal.clone(),block.clone()).as_ref(){
                    Some(n)=>{
                        board[i][j]=n.borrow().value;
                    },None=>{
                        println!("{},{}",i,j);
                        return;
                    }
                }
            }
        }
    }
}
// --
pub use std;
pub fn max(a:i32,b:i32)->i32{return if a>b{a}else{b};}
pub fn min(a:i32,b:i32)->i32{return if a<b{a}else{b};}
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
}pub fn signum(a:i32,b:i32)->i32{
    if a==b{return 0;}
    if a>b{return 1;}
    if a<b{return -1;}
    else{panic!("unexpected case");}
}
#[derive(Debug)]
pub struct Node{
    pub height:i32,pub value:i32,
    pub prev:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
    pub next:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
    pub parent:std::rc::Rc<Option<std::cell::RefCell<Node>>>
}impl Node{
    pub fn new(value:i32)->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        return std::rc::Rc::new(
            Some(
                std::cell::RefCell::new(
                    Node{height:0,value,
                        prev:std::rc::Rc::new(None),
                        next:std::rc::Rc::new(None),
                        parent:std::rc::Rc::new(None)
                    }
                )
            )
        );
    }pub fn append(
        n:std::rc::Rc<Option<std::cell::RefCell<Node>>>,value:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match n.as_ref(){
            Some(o)=>{
                let temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>
                =Node::append(o.borrow().next.clone(),value);
                o.borrow_mut().next=temp;
                return n;
            },None=>{
                return Node::new(value);
            }
        }
    }pub fn print(n:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match n.as_ref(){
            Some(o)=>{
                print!("[{}]",o.borrow().value);
                Node::print(o.borrow().next.clone());
            },None=>{return;}
        }
    }pub fn mid(
        head:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match head.as_ref(){
            Some(_)=>{
                let mut slow:std::rc::Rc<Option<std::cell::RefCell<Node>>>=head.clone();
                let mut fast:std::rc::Rc<Option<std::cell::RefCell<Node>>>=slow.clone();
                loop{
                    match slow.clone().as_ref(){
                        Some(n)=>{
                            match n.borrow().next.as_ref(){
                                Some(_)=>{
                                    slow=n.borrow().next.clone();
                                },None=>{break;}
                            }
                        },None=>{panic!("unexpected None");}
                    }match fast.clone().as_ref(){
                        Some(n)=>{
                            match n.borrow().next.as_ref(){
                                Some(_)=>{
                                    fast=n.borrow().next.clone();
                                },None=>{break;}
                            }
                        },None=>{panic!("unexpected None");}
                    }match fast.clone().as_ref(){
                        Some(n)=>{
                            match n.borrow().next.as_ref(){
                                Some(_)=>{
                                    fast=n.borrow().next.clone();
                                },None=>{break;}
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }println!("finished list_mid");
                return slow;
            },None=>{return head;}
        }
    }pub fn head(
        tail:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match tail.as_ref(){
            Some(_)=>{
                let mut head:std::rc::Rc<Option<std::cell::RefCell<Node>>>=tail.clone();
                loop{
                    match head.clone().as_ref(){
                        Some(n)=>{
                            match n.borrow().prev.as_ref(){
                                Some(_)=>{
                                    head=n.borrow().prev.clone();
                                },None=>{break;}
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }return head;
            },None=>{return tail;}
        }
    }pub fn collect(
        head:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->Vec<i32>{
        match head.as_ref(){
            Some(n)=>{
                let mut v:Vec<i32>=Vec::new();
                v.push(n.borrow().value);
                v.extend(Node::collect(n.borrow().next.clone()).iter());
                return v;
            },None=>{return Vec::new();}
        }
    }
}
#[derive(Debug)]
pub struct Shell{
    pub data:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
    pub prev:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
    pub next:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
    pub value:i32
}impl Shell{
    pub fn new(
        data:i32,value:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        return std::rc::Rc::new(
            Some(
                std::cell::RefCell::new(
                    Shell {
                        data:value,value:data,
                        next:std::rc::Rc::new(None),
                        prev:std::rc::Rc::new(None)
                    }
                )
            )
        );
    }pub fn insert(
        s:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
        position:i32,
        value:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        match s.as_ref(){
            Some(h)=>{
                let temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>;
                if position>0{
                    temp=Shell::insert(h.borrow().next.clone(),position-1,value);
                    match temp.as_ref(){
                        Some(e)=>{
                            e.borrow_mut().prev=s.clone();
                        },None=>{panic!("unexpected None");}
                    }h.borrow_mut().next=temp;
                    return s;
                }else if position<0{
                    temp=Shell::insert(h.borrow().prev.clone(),position+1,value);
                    match temp.as_ref(){
                        Some(e)=>{
                            e.borrow_mut().next=s.clone();
                        },None=>{panic!("unexpected None");}
                    }h.borrow_mut().prev=temp;
                    return s;
                }else{
                    match value.as_ref(){
                        Some(n)=>{
                            Node::append(h.borrow().data.clone(),n.borrow().value);
                        },None=>{panic!("unexpected None");}
                    }
                    return s;
                }
            },None=>{return Shell::new(position,value);}
        }
    }pub fn print(s:std::rc::Rc<Option<std::cell::RefCell<Shell>>>){
        match s.as_ref(){
            Some(h)=>{
                Node::print(h.borrow().data.clone());
                println!("");
                Shell::print(h.borrow().next.clone());
            },None=>{return;}
        }
    }pub fn root(
        s:std::rc::Rc<Option<std::cell::RefCell<Shell>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        match s.as_ref(){
            Some(h)=>{
                match h.borrow().prev.as_ref(){
                    Some(_)=>{
                        return Shell::root(h.borrow().prev.clone());
                    },None=>{return s.clone();}
                }
            },None=>{return s;}
        }
    }
}pub struct Queue{
    front:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
    rare:std::rc::Rc<Option<std::cell::RefCell<Shell>>>
}impl Queue{
    pub fn new()->Queue{
        return Queue{
            front:std::rc::Rc::new(None),
            rare:std::rc::Rc::new(None)
        }
    }pub fn enqueue(&mut self,val:i32,value:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        let temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>;
        match self.rare.as_ref(){
            Some(s)=>{
                s.borrow_mut().next=Shell::new(val,value);
                temp=s.borrow().next.clone();
            },None=>{
                self.rare=Shell::new(val,value);
                self.front=self.rare.clone();
                return;
            }
        }self.rare=temp;
    }pub fn dequeue(&mut self)->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        let temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>;
        let value:std::rc::Rc<Option<std::cell::RefCell<Shell>>>=self.front.clone();
        match self.front.as_ref(){
            Some(s)=>{
                temp=s.borrow().next.clone();
            },None=>{return std::rc::Rc::new(None);}
        }self.front=temp;match self.front.as_ref(){
            Some(_)=>{},None=>{
                self.rare=self.front.clone();
            }
        }return value;
    }pub fn empty(&self)->bool{
        return match self.front.as_ref(){
            Some(_)=>{false},None=>{true}
        }
    }
}pub struct Stack{
    pub top:std::rc::Rc<Option<std::cell::RefCell<Shell>>>
}impl Stack{
    pub fn new()->Stack{
        return Stack{top:std::rc::Rc::new(None)};
    }
    pub fn push(&mut self,n:std::rc::Rc<Option<std::cell::RefCell<Node>>>,v:i32){
        let temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>=Shell::new(v,n);
        match temp.as_ref(){
            Some(s)=>{
                s.borrow_mut().next=self.top.clone();
            },None=>{panic!("unexpected None");}
        }match self.top.as_ref(){
            Some(s)=>{
                s.borrow_mut().prev=temp.clone();
            },None=>{}
        }self.top=temp;
        return;
    }pub fn pop(&mut self)->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        let temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>=self.top.clone();
        match self.top.clone().as_ref(){
            Some(s)=>{
                self.top=s.borrow().next.clone();
                match self.top.as_ref(){
                    Some(h)=>{
                        h.borrow_mut().prev=std::rc::Rc::new(None);
                    },None=>{}
                }
            },None=>{}
        }return temp;
    }pub fn find(&self,n:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        let mut temp:std::rc::Rc<Option<std::cell::RefCell<Shell>>>=self.top.clone();
        loop{
            match temp.clone().as_ref(){
                Some(s)=>{
                    if std::rc::Rc::ptr_eq(&s.borrow().data,&n){
                        break;
                    }else{
                        temp=s.borrow().next.clone();
                    }
                },None=>{break;}
            }
        }return temp;
    }
}pub struct TreeInfo{pub bst:bool,pub height:i32,pub min:i32,pub max:i32}
impl TreeInfo{
    pub fn new(bst:bool,height:i32,min:i32,max:i32)
    ->TreeInfo{return TreeInfo{bst,height,min,max}}
}pub struct Tree;impl Tree{
    pub fn new()->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        return std::rc::Rc::new(None);
    }pub fn in_order_traversal(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,)->Vec<i32>{
        let mut list:Vec<i32>=Vec::new();
        match root.as_ref(){
            Some(n)=>{
                list.extend(Tree::in_order_traversal(n.borrow().prev.clone()).iter());
                list.push(n.borrow().value);
                list.extend(Tree::in_order_traversal(n.borrow().next.clone()).iter());
                return list;
            },None=>{return list;}
        }
    }pub fn pre_order_traversal(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,)->Vec<i32>{
        let mut list:Vec<i32>=Vec::new();
        match root.as_ref(){
            Some(n)=>{
                list.push(n.borrow().value);
                list.extend(Tree::pre_order_traversal(n.borrow().prev.clone()).iter());
                list.extend(Tree::pre_order_traversal(n.borrow().next.clone()).iter());
                return list;
            },None=>{return list;}
        }
    }pub fn show(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        println!(
            "[Tree]:{:#?}",
            Tree::custom_constructor(
                Tree::in_order_traversal(root.clone()).as_slice(),
                Tree::pre_order_traversal(root.clone()).as_slice()
            )
        );
    }pub fn height(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->i32{
        match root.as_ref(){
            Some(n)=>{
                return max(
                    Tree::height(n.borrow().prev.clone()),
                    Tree::height(n.borrow().next.clone())
                )+1;
            },None=>{return -1;}
        }
    }pub fn rotation_l(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                let temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>=n.borrow().prev.clone();
                match temp.as_ref(){
                    Some(o)=>{
                        o.borrow_mut().parent=std::rc::Rc::new(None);
                        let mut val:i32;
                        n.borrow_mut().prev=o.borrow().next.clone();
                        match o.borrow().next.as_ref(){
                            Some(d)=>{
                                d.borrow_mut().parent=root.clone();
                            },None=>{}
                        }o.borrow_mut().next=root.clone();
                        n.borrow_mut().parent=temp.clone();
                        val=max(match n.borrow().prev.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        },match n.borrow().next.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        });n.borrow_mut().height=val+1;
                        val=max(match o.borrow().prev.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        },match o.borrow().next.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        });o.borrow_mut().height=val+1;
                        return temp;
                    },None=>{return root;}
                }
            },None=>{return root;}
        }
    }pub fn rotation_r(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                n.borrow_mut().parent=std::rc::Rc::new(None);
                let temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>=n.borrow().next.clone();
                match temp.as_ref(){
                    Some(o)=>{
                        o.borrow_mut().parent=std::rc::Rc::new(None);
                        let mut val:i32;
                        n.borrow_mut().next=o.borrow().prev.clone();
                        match o.borrow().prev.as_ref(){                            
                            Some(d)=>{
                                d.borrow_mut().parent=root.clone();
                            },None=>{}
                        }
                        o.borrow_mut().prev=root.clone();
                        n.borrow_mut().parent=temp.clone();
                        val=max(match n.borrow().prev.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        },match n.borrow().next.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        });n.borrow_mut().height=val+1;
                        val=max(match o.borrow().prev.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        },match o.borrow().next.as_ref(){
                            Some(d)=>{
                                d.borrow().height
                            },None=>{-1}
                        });o.borrow_mut().height=val+1;
                        return temp;
                    },None=>{return root;}
                }
            },None=>{return root;}
        }
    }pub fn insert(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,value:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                n.borrow_mut().parent=std::rc::Rc::new(None);
                let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                let mut val:i32;
                if value>n.borrow().value{
                    temp=Tree::insert(n.borrow().next.clone(),value);
                    n.borrow_mut().next=temp.clone();
                    val=max(match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    },match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    });n.borrow_mut().height=val+1;
                    val=match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }-match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    };if -1>val||1<val{
                        match n.borrow().next.as_ref(){
                            Some(o)=>{
                                if value<o.borrow().value{
                                    temp=Tree::rotation_l(n.borrow().next.clone());
                                }else{temp=n.borrow().next.clone();}
                            },None=>{panic!("unexpected None");}
                        }match temp.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }n.borrow_mut().next=temp;
                        return Tree::rotation_r(root);
                    }else{
                        match n.borrow().next.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }return root;
                    }
                }else if value<n.borrow().value{
                    temp=Tree::insert(n.borrow().prev.clone(),value);
                    n.borrow_mut().prev=temp.clone();
                    val=max(match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    },match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    });n.borrow_mut().height=val+1;
                    val=match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }-match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    };if 1<val||-1>val{
                        match n.borrow().prev.as_ref(){
                            Some(o)=>{
                                if value>o.borrow().value{
                                    temp=Tree::rotation_r(n.borrow().prev.clone());
                                }else{temp=n.borrow().prev.clone();}
                            },None=>{panic!("unexpected None");}
                        }match temp.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }n.borrow_mut().prev=temp;
                        return Tree::rotation_l(root);
                    }else{
                        match n.borrow().prev.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }return root;
                    }
                }else{
                    return root;
                }
            },None=>{return Node::new(value);}
        }
    }pub fn delete(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,value:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                // variable initialization
                let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                let mut val:i32;
                if value>n.borrow().value{
                    temp=Tree::delete(n.borrow().next.clone(),value);
                    n.borrow_mut().next=temp.clone();
                    val=max(match n.borrow().prev.as_ref(){// setting height;
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    },match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    })+1;n.borrow_mut().height=val;
                    val=match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }-match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    };if -1>val||1<val{// balance factor
                        match n.borrow().prev.as_ref(){
                            Some(o)=>{
                                val=match o.borrow().prev.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                }-match o.borrow().next.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                };
                            },None=>{val=0;}
                        }if val>-1{
                            return Tree::rotation_l(root);
                        }else{
                            temp=Tree::rotation_r(n.borrow().prev.clone());
                            n.borrow_mut().prev=temp;
                            val=max(match n.borrow().prev.as_ref(){// setting height;
                                Some(o)=>{
                                    o.borrow().height
                                },None=>{-1}
                            },match n.borrow().next.as_ref(){
                                Some(o)=>{
                                    o.borrow().height
                                },None=>{-1}
                            })+1;n.borrow_mut().height=val;
                            return Tree::rotation_l(root);
                        }
                    }else{
                        n.borrow_mut().parent=std::rc::Rc::new(None);
                        match n.borrow().next.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }return root;
                    }
                }else if value<n.borrow().value{
                    temp=Tree::delete(n.borrow().prev.clone(),value);
                    n.borrow_mut().prev=temp.clone();
                    val=max(match n.borrow().prev.as_ref(){// setting height;
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    },match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    })+1;n.borrow_mut().height=val;
                    val=match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }-match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    };if -1>val||1<val{// balance factor
                        match n.borrow().next.as_ref(){
                            Some(o)=>{
                                val=match o.borrow().prev.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                }-match o.borrow().next.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                };
                            },None=>{val=0;}
                        }if val<1{
                            return Tree::rotation_r(root);
                        }else{
                            temp=Tree::rotation_l(n.borrow().next.clone());
                            n.borrow_mut().next=temp;
                            val=max(match n.borrow().prev.as_ref(){// setting height;
                                Some(o)=>{
                                    o.borrow().height
                                },None=>{-1}
                            },match n.borrow().next.as_ref(){
                                Some(o)=>{
                                    o.borrow().height
                                },None=>{-1}
                            })+1;n.borrow_mut().height=val;
                            return Tree::rotation_r(root);
                        }
                    }else{
                        n.borrow_mut().parent=std::rc::Rc::new(None);
                        match n.borrow().prev.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=root.clone();
                            },None=>{}
                        }return root;
                    }
                }else{
                    let mut prev:std::rc::Rc<Option<std::cell::RefCell<Node>>>
                    =std::rc::Rc::new(None);
                    let mut tmp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                    if match n.borrow().prev.as_ref(){Some(_)=>{true},None=>{false}}{
                        temp=n.borrow().prev.clone();loop{
                            match temp.as_ref(){
                                Some(o)=>{
                                    match o.borrow().next.as_ref(){
                                        Some(_)=>{
                                            prev=temp.clone();
                                        },None=>{break;}
                                    }
                                },None=>{panic!("unexpected None");}
                            }match prev.as_ref(){
                                Some(o)=>{
                                    temp=o.borrow().next.clone()
                                },None=>{panic!("unexpected None");}
                            }
                        }match temp.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().parent=std::rc::Rc::new(None);
                                match prev.as_ref(){
                                    Some(d)=>{
                                        d.borrow_mut().next=o.borrow().prev.clone();
                                        o.borrow_mut().prev=n.borrow().prev.clone();
                                        o.borrow_mut().next=n.borrow().next.clone();
                                        val=max(match d.borrow().prev.as_ref(){// setting height
                                            Some(e)=>{
                                                e.borrow().height
                                            },None=>{-1}
                                        },match d.borrow().next.as_ref(){
                                            Some(e)=>{
                                                e.borrow_mut().parent=prev.clone();
                                                e.borrow().height
                                            },None=>{-1}
                                        })+1;d.borrow_mut().height=val;
                                        val=match d.borrow().prev.as_ref(){
                                            Some(e)=>{
                                                e.borrow().height
                                            },None=>{-1}
                                        }-match d.borrow().next.as_ref(){
                                            Some(e)=>{
                                                e.borrow().height
                                            },None=>{-1}
                                        };if -1>val||1<val{// balance factor
                                            match d.borrow().prev.as_ref(){
                                                Some(e)=>{
                                                    val=match e.borrow().prev.as_ref(){
                                                        Some(x)=>{
                                                            x.borrow().height
                                                        },None=>{-1}
                                                    }-match e.borrow().next.as_ref(){
                                                        Some(x)=>{
                                                            x.borrow().height
                                                        },None=>{-1}
                                                    };
                                                },None=>{val=0}
                                            }if val>-1{
                                                tmp=d.borrow().parent.clone();
                                                match tmp.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow_mut().next=Tree::rotation_l(prev.clone());
                                                        val=max(match e.borrow().prev.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow().height
                                                            },None=>{-1}
                                                        },match e.borrow().next.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow_mut().parent=tmp.clone();
                                                                x.borrow().height
                                                            },None=>{panic!("unexpected None");}
                                                        })+1;e.borrow_mut().height=val;
                                                    },None=>{panic!("unexpected None");}
                                                }
                                            }else{
                                                tmp=Tree::rotation_r(d.borrow().prev.clone());
                                                d.borrow_mut().prev=tmp.clone();
                                                val=max(match d.borrow().prev.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow_mut().parent=prev.clone();
                                                        e.borrow().height
                                                    },None=>{panic!("unexpected None");}
                                                },match d.borrow().next.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow().height
                                                    },None=>{-1}
                                                })+1;d.borrow_mut().height=val;
                                                tmp=d.borrow().parent.clone();
                                                match tmp.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow_mut().next=Tree::rotation_l(prev.clone());
                                                        val=max(match e.borrow().prev.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow().height
                                                            },None=>{-1}
                                                        },match e.borrow().next.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow_mut().parent=tmp.clone();
                                                                x.borrow().height
                                                            },None=>{panic!("unexpected None");}
                                                        })+1;e.borrow_mut().height=val;
                                                    },None=>{panic!("unexpected None");}
                                                }
                                            }
                                        }
                                    },None=>{
                                        o.borrow_mut().next=n.borrow().next.clone();
                                    }
                                }val=max(match o.borrow().prev.as_ref(){// setting height
                                    Some(d)=>{
                                        d.borrow_mut().parent=temp.clone();
                                        d.borrow().height
                                    },None=>{-1}
                                },match o.borrow().next.as_ref(){
                                    Some(d)=>{
                                        d.borrow_mut().parent=temp.clone();
                                        d.borrow().height
                                    },None=>{-1}
                                })+1;o.borrow_mut().height=val;
                                val=match o.borrow().prev.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                }-match o.borrow().next.as_ref(){
                                    Some(d)=>{
                                        d.borrow().height
                                    },None=>{-1}
                                };if -1>val||1<val{
                                    match o.borrow().next.as_ref(){
                                        Some(d)=>{
                                            val=match d.borrow().prev.as_ref(){
                                                Some(e)=>{
                                                    e.borrow().height
                                                },None=>{-1}
                                            }-match d.borrow().next.as_ref(){
                                                Some(e)=>{
                                                    e.borrow().height
                                                },None=>{-1}
                                            };
                                        },None=>{val=0;}
                                    }if val<1{
                                        return Tree::rotation_r(temp);
                                    }else{
                                        tmp=Tree::rotation_l(o.borrow().next.clone());
                                        o.borrow_mut().next=tmp;
                                        val=max(match o.borrow().prev.as_ref(){
                                            Some(d)=>{
                                                d.borrow().height
                                            },None=>{-1}
                                        },match o.borrow().next.as_ref(){
                                            Some(d)=>{
                                                d.borrow_mut().parent=temp.clone();
                                                d.borrow().height
                                            },None=>{panic!("unexpected None");}
                                        })+1;o.borrow_mut().height=val;
                                        return Tree::rotation_r(temp.clone());
                                    }
                                }else{
                                    return temp;
                                }
                            },None=>{panic!("unexpected None");}
                        }
                    }else{
                        if match n.borrow().next.as_ref(){Some(_)=>{true},None=>{false}}{
                            temp=n.borrow().next.clone();loop{
                                match temp.as_ref(){
                                    Some(o)=>{
                                        match o.borrow().prev.as_ref(){
                                            Some(_)=>{
                                                prev=temp.clone();
                                            },None=>{break;}
                                        }
                                    },None=>{panic!("unexpected None");}
                                }match prev.as_ref(){
                                    Some(o)=>{
                                        temp=o.borrow().prev.clone();
                                    },None=>{panic!("unexpected None");}
                                }
                            }match temp.as_ref(){
                                Some(o)=>{
                                    o.borrow_mut().parent=std::rc::Rc::new(None);
                                    match prev.as_ref(){
                                        Some(d)=>{
                                            d.borrow_mut().prev=o.borrow().next.clone();
                                            o.borrow_mut().prev=n.borrow().prev.clone();
                                            o.borrow_mut().next=n.borrow().next.clone();
                                            val=max(match d.borrow().prev.as_ref(){// setting height
                                                Some(e)=>{
                                                    e.borrow_mut().parent=prev.clone();
                                                    e.borrow().height
                                                },None=>{-1}
                                            },match d.borrow().next.as_ref(){
                                                Some(e)=>{
                                                    e.borrow().height
                                                },None=>{-1}
                                            })+1;d.borrow_mut().height=val;
                                            val=match d.borrow().prev.as_ref(){
                                                Some(e)=>{
                                                    e.borrow().height
                                                },None=>{-1}
                                            }-match d.borrow().next.as_ref(){
                                                Some(e)=>{
                                                    e.borrow().height
                                                },None=>{-1}
                                            };if -1>val||1<val{
                                                match d.borrow().next.as_ref(){
                                                    Some(e)=>{
                                                        val=match e.borrow().prev.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow().height
                                                            },None=>{-1}
                                                        }-match e.borrow().next.as_ref(){
                                                            Some(x)=>{
                                                                x.borrow().height
                                                            },None=>{-1}
                                                        };
                                                    },None=>{val=0;}
                                                }if val<1{
                                                    tmp=d.borrow().parent.clone();
                                                    match tmp.as_ref(){
                                                        Some(e)=>{
                                                            e.borrow_mut().prev=Tree::rotation_r(prev.clone());
                                                            val=max(match e.borrow().prev.as_ref(){
                                                                Some(x)=>{
                                                                    x.borrow_mut().parent=tmp.clone();
                                                                    x.borrow().height
                                                                },None=>{panic!("unexpected None");}
                                                            },match e.borrow().next.as_ref(){
                                                                Some(x)=>{
                                                                    x.borrow().height
                                                                },None=>{-1}
                                                            })+1;e.borrow_mut().height=val;
                                                        },None=>{panic!("unexpected None");}
                                                    }
                                                }else{
                                                    tmp=Tree::rotation_l(d.borrow().next.clone());
                                                    d.borrow_mut().next=tmp.clone();
                                                    val=max(match d.borrow().prev.as_ref(){
                                                        Some(e)=>{
                                                            e.borrow().height
                                                        },None=>{-1}
                                                    },match d.borrow().next.as_ref(){
                                                        Some(e)=>{
                                                            e.borrow().height
                                                        },None=>{panic!("unexpected None");}
                                                    })+1;d.borrow_mut().height=val;
                                                    tmp=d.borrow().parent.clone();
                                                    match tmp.as_ref(){
                                                        Some(e)=>{
                                                            e.borrow_mut().prev=Tree::rotation_r(prev.clone());
                                                            val=max(match e.borrow().prev.as_ref(){
                                                                Some(x)=>{
                                                                    x.borrow_mut().parent=tmp.clone();
                                                                    x.borrow().height
                                                                },None=>{panic!("unexpected None");}
                                                            },match e.borrow().next.as_ref(){
                                                                Some(x)=>{
                                                                    x.borrow().height
                                                                },None=>{-1}
                                                            })+1;e.borrow_mut().height=val;
                                                        },None=>{panic!("unexpected None");}
                                                    }
                                                }
                                            }
                                        },None=>{
                                            o.borrow_mut().prev=n.borrow().prev.clone();
                                        }
                                    }val=max(match o.borrow().prev.as_ref(){// setting height
                                        Some(d)=>{
                                            d.borrow_mut().parent=temp.clone();
                                            d.borrow().height
                                        },None=>{-1}
                                    },match o.borrow().next.as_ref(){
                                        Some(d)=>{
                                            d.borrow_mut().parent=temp.clone();
                                            d.borrow().height
                                        },None=>{-1}
                                    })+1;o.borrow_mut().height=val;
                                    val=match o.borrow().prev.as_ref(){
                                        Some(d)=>{
                                            d.borrow().height
                                        },None=>{-1}
                                    }-match o.borrow().next.as_ref(){
                                        Some(d)=>{
                                            d.borrow().height
                                        },None=>{-1}
                                    };if -1>val||1<val{
                                        match o.borrow().prev.as_ref(){
                                            Some(d)=>{
                                                val=match d.borrow().prev.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow().height
                                                    },None=>{-1}
                                                }-match d.borrow().next.as_ref(){
                                                    Some(e)=>{
                                                        e.borrow().height
                                                    },None=>{-1}
                                                };
                                            },None=>{val=0;}
                                        }if val>-1{
                                            return Tree::rotation_l(temp.clone());
                                        }else{
                                            tmp=Tree::rotation_r(o.borrow().prev.clone());
                                            o.borrow_mut().prev=tmp;
                                            val=max(match o.borrow().prev.as_ref(){
                                                Some(d)=>{
                                                    d.borrow_mut().parent=temp.clone();
                                                    d.borrow().height
                                                },None=>{panic!("unexpected None");}
                                            },match o.borrow().next.as_ref(){
                                                Some(d)=>{
                                                    d.borrow().height
                                                },None=>{-1}
                                            })+1;o.borrow_mut().height=val;
                                            return Tree::rotation_l(temp.clone());
                                        }
                                    }else{
                                        return temp;
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }else{
                            return std::rc::Rc::new(None);
                        }
                    }
                }
            },None=>{return root;}
        }
    }pub fn display(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(_)=>{
                let mut collection:Queue=Queue::new();
                collection.enqueue(0,root);
                collection.enqueue(0,std::rc::Rc::new(None));
                loop{
                    match collection.dequeue().as_ref(){
                        Some(s)=>{
                            match s.borrow().data.as_ref(){
                                Some(n)=>{
                                    print!("({})",n.borrow().value);
                                    match n.borrow().prev.as_ref(){
                                        Some(_)=>{
                                            collection.enqueue(
                                                0,n.borrow().prev.clone()
                                            );
                                        },None=>{}
                                    }match n.borrow().next.as_ref(){
                                        Some(_)=>{
                                            collection.enqueue(
                                                0,n.borrow().next.clone()
                                            );
                                        },None=>{}
                                    }
                                },None=>{
                                    if !collection.empty(){
                                        collection.enqueue(0,std::rc::Rc::new(None));
                                    }
                                    print!("\n");
                                }
                            }
                        },None=>{break;}
                    }
                }
            },None=>{return;}
        }
    }pub fn diameter(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->i32{
        match root.as_ref(){
            Some(n)=>{
                return max(
                    match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }+match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }+1,max(
                        Tree::diameter(n.borrow().prev.clone()),
                        Tree::diameter(n.borrow().next.clone())
                    )
                );
            },None=>{return 0;}
        }
    }pub fn identical(
        a:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        b:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->bool{
        match a.as_ref(){
            Some(n)=>{
                return match b.as_ref(){
                    Some(o)=>{
                        if Tree::identical(
                            n.borrow().prev.clone(),
                            o.borrow().prev.clone()
                        )&&Tree::identical(
                            n.borrow().next.clone(),
                            o.borrow().next.clone()
                        )&&n.borrow().value==o.borrow().value
                        {true}else{false}
                    },None=>{return false;}
                }
            },None=>{
                match b.as_ref(){
                    Some(_)=>{return false;},
                    None=>{return true;}
                }
            }
        }
    }pub fn zigzag_print(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        let mut collection:Queue=Queue::new();
        collection.enqueue(0,root.clone());
        loop{
            collection.enqueue(0,std::rc::Rc::new(None));
            loop{
                match collection.dequeue().as_ref(){
                    Some(s)=>{
                        match s.borrow().data.as_ref(){
                            Some(n)=>{
                                print!("<{}>",n.borrow().value);
                                match n.borrow().prev.as_ref(){
                                    Some(_)=>{
                                        collection.enqueue(0,n.borrow().prev.clone());
                                    },None=>{}
                                }match n.borrow().next.as_ref(){
                                    Some(_)=>{
                                        collection.enqueue(0,n.borrow().next.clone());
                                    },None=>{}
                                }
                            },None=>{
                                println!("|");
                                match collection.front.as_ref(){
                                    Some(_)=>{},None=>{return;}
                                }break;
                            }
                        }
                    },None=>{panic!("unexpected None");}
                }
            }
            collection.enqueue(0,std::rc::Rc::new(None));
            loop{
                match collection.dequeue().as_ref(){
                    Some(s)=>{
                        match s.borrow().data.as_ref(){
                            Some(n)=>{
                                print!("<{}>",n.borrow().value);
                                match n.borrow().next.as_ref(){
                                    Some(_)=>{
                                        collection.enqueue(0,n.borrow().next.clone());
                                    },None=>{}
                                }match n.borrow().prev.as_ref(){
                                    Some(_)=>{
                                        collection.enqueue(0,n.borrow().prev.clone());
                                    },None=>{}
                                }
                            },None=>{
                                println!("||");
                                match collection.front.as_ref(){
                                    Some(_)=>{},None=>{return;}
                                }break;
                            }
                        }
                    },None=>{panic!("unexpected None");}
                }
            }
        }
    }pub fn vertical_print(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(_)=>{
                let mut line:std::rc::Rc<Option<std::cell::RefCell<Shell>>>;
                let mut collection:Queue=Queue::new();
                line=std::rc::Rc::new(None);
                collection.enqueue(0,root);
                loop{
                    match collection.dequeue().as_ref(){
                        Some(s)=>{
                            match s.borrow().data.as_ref(){
                                Some(n)=>{
                                    line=Shell::insert(
                                        line,
                                        s.borrow().value,
                                        Node::new(n.borrow().value)
                                    );match n.borrow().prev.as_ref(){
                                        Some(_)=>{
                                            collection.enqueue(
                                                s.borrow().value-1,
                                                n.borrow().prev.clone()
                                            );
                                        },None=>{}
                                    }match n.borrow().next.as_ref(){
                                        Some(_)=>{
                                            collection.enqueue(
                                                s.borrow().value+1,
                                                n.borrow().next.clone()
                                            );
                                        },None=>{}
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        },None=>{break;}
                    }
                }Shell::print(Shell::root(line));
            },None=>{println!("");}
        }
    }pub fn diagonal_print(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(_)=>{
                let mut collection:Queue=Queue::new();
                let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                collection.enqueue(0,root);
                loop{
                    match collection.dequeue().as_ref(){
                        Some(s)=>{
                            temp=s.borrow().data.clone();
                            loop{
                                match temp.clone().as_ref(){
                                    Some(n)=>{
                                        print!("[{}]",n.borrow().value);
                                        match n.borrow().prev.as_ref(){
                                            Some(_)=>{
                                                collection.enqueue(
                                                    0,
                                                    n.borrow().prev.clone()
                                                );
                                            },None=>{}
                                        }
                                        temp=n.borrow().next.clone();
                                    },None=>{break;}
                                }
                            }
                        },None=>{break;}
                    }
                }
            },None=>{println!("");}
        }
    }pub fn long_root_sum(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->i32{
        match root.as_ref(){
            Some(n)=>{
                return n.borrow().value+Tree::long_root_sum(
                    if match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }>match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }{n.borrow().prev.clone()}
                    else{n.borrow().next.clone()}
                );
            },None=>{return 0;}
        }
    }pub fn lca(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        a:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        b:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let mut temp_a:std::rc::Rc<Option<std::cell::RefCell<Node>>>
        =std::rc::Rc::new(None);
        let mut temp_b:std::rc::Rc<Option<std::cell::RefCell<Node>>>
        =std::rc::Rc::new(None);
        if std::rc::Rc::ptr_eq(&root,&a){
            temp_a=root.clone();
        }if std::rc::Rc::ptr_eq(&root,&b){
            temp_b=root.clone();
        }match root.as_ref(){
            Some(n)=>{
                if match temp_a.clone().as_ref(){
                    Some(_)=>{false},None=>{true}
                }{
                    temp_a=Tree::lca(n.borrow().prev.clone(),a.clone(),b.clone());
                }if match temp_b.clone().as_ref(){
                    Some(_)=>{false},None=>{true}
                }{
                    temp_b=Tree::lca(n.borrow().next.clone(),a.clone(),b.clone());
                }if match temp_a.as_ref(){
                    Some(_)=>{true},None=>{false}
                }&&match temp_b.as_ref(){
                    Some(_)=>{true},None=>{false}
                }{return root;}else{
                    if match temp_a.as_ref(){
                        Some(_)=>{true},None=>{false}
                    }{
                        return temp_a;
                    }else if match temp_b.as_ref(){
                        Some(_)=>{true},None=>{false}
                    }{
                        return temp_b;
                    }else{
                        return std::rc::Rc::new(None);
                    }
                }
            },None=>{
                return root;
            }
        }
    }pub fn kth_ancestor(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        target:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        mut k:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Shell>>>{
        if std::rc::Rc::ptr_eq(&root,&target){
            return Shell::new(k-1,root);
        }else{
            match root.as_ref(){
                Some(n)=>{
                    let temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                    match Tree::kth_ancestor(
                        n.borrow().prev.clone(),
                        target.clone(),k
                    ).as_ref(){
                        Some(s)=>{
                            k=s.borrow().value;
                            temp=s.borrow().data.clone();
                        },None=>{
                            match Tree::kth_ancestor(
                                n.borrow().next.clone(),
                                target.clone(),k
                            ).as_ref(){
                                Some(s)=>{
                                    k=s.borrow().value;
                                    temp=s.borrow().data.clone();
                                },None=>{temp=std::rc::Rc::new(None)}
                            }
                        }
                    }if k==0{
                        return Shell::new(-1,root);
                    }else{
                        match temp.as_ref(){
                            Some(_)=>{
                                return Shell::new(k-1,temp);
                            },None=>{return std::rc::Rc::new(None);}
                        }
                    }
                },None=>{return std::rc::Rc::new(None);}
            }
        }
    }pub fn find(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        target:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                if target>n.borrow().value{
                    return Tree::find(n.borrow().next.clone(),target);
                }else if target<n.borrow().value{
                    return Tree::find(n.borrow().prev.clone(),target);
                }else{
                    return root;
                }
            },None=>{return root;}
        }
    }pub fn max_none_adjacent_sum(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->(i32,i32){
        match root.as_ref(){
            Some(n)=>{
                let prev:(i32,i32);let next:(i32,i32);
                prev=Tree::max_none_adjacent_sum(n.borrow().prev.clone());
                next=Tree::max_none_adjacent_sum(n.borrow().next.clone());
                return (
                    n.borrow().value+prev.1+next.1,
                    max(prev.1,prev.0)+max(next.0,next.1)
                );
            },None=>{
                return (0,0);
            }
        }
    }pub fn custom_constructor(
        ino:&[i32],pre:&[i32]
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        if ino.len()<1&&pre.len()<1{
            return std::rc::Rc::new(None);
        }else{
            let root:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Node::new(pre[0]);
            let mut i:usize=0;loop{if ino[i]==pre[0]||!i<ino.len(){break;}else{i+=1;}}
            if i<ino.len(){
                match root.as_ref(){
                    Some(n)=>{
                        n.borrow_mut().prev=Tree::custom_constructor(
                            &ino[0..i],&pre[1..i+1]
                        );n.borrow_mut().next=Tree::custom_constructor(
                            &ino[i+1..],&pre[i+1..]
                        );return root;
                    },None=>{panic!("unexpected None");}
                }
            }else{panic!("uncommon value found");}
        }
    }/*pub fn min_burn_time(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        target:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->i32{
        // [unfinished!];
        return 0;
    }*/pub fn morris_traversal(mut current:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        loop{
            match current.clone().as_ref(){
                Some(n)=>{
                    match n.borrow().prev.as_ref(){
                        Some(_)=>{
                            temp=n.borrow().prev.clone();
                            loop{
                                match temp.clone().as_ref(){
                                    Some(o)=>{
                                        if std::rc::Rc::ptr_eq(&o.borrow().next,&current){
                                            o.borrow_mut().next=std::rc::Rc::new(None);
                                            temp=o.borrow_mut().next.clone();
                                        }else{
                                            match o.borrow().next.as_ref(){
                                                Some(_)=>{
                                                    temp=o.borrow().next.clone();
                                                },None=>{break;}
                                            }
                                        }
                                    },None=>{break;}
                                }
                            }match temp.as_ref(){
                                Some(o)=>{
                                    o.borrow_mut().next=current.clone();
                                    current=n.borrow().prev.clone();
                                },None=>{
                                    print!("<{}>",n.borrow().value);
                                    current=n.borrow().next.clone();
                                }
                            }
                        },None=>{
                            print!("<{}>",n.borrow().value);
                            current=n.borrow().next.clone();
                        }
                    }
                },None=>{break;}
            }
        }println!("");
        return;
    }pub fn flatten_binary_tree(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        let mut prev:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        let mut current:std::rc::Rc<Option<std::cell::RefCell<Node>>>=root.clone();
        loop{// root=LinkedList head [pre order traversal];
            match current.clone().as_ref(){
                Some(n)=>{
                    if match n.borrow().prev.as_ref()
                    {Some(_)=>{true},None=>{false}}{
                        prev=n.borrow().prev.clone();
                        loop{
                            match prev.clone().as_ref(){
                                Some(o)=>{
                                    match o.borrow().next.as_ref(){
                                        Some(_)=>{
                                            prev=o.borrow().next.clone();
                                        },None=>{break;}
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }match prev.as_ref(){
                            Some(o)=>{
                                o.borrow_mut().next=n.borrow().next.clone();
                                temp=n.borrow().prev.clone();
                                n.borrow_mut().next=temp;
                                n.borrow_mut().prev=std::rc::Rc::new(None);
                                current=n.borrow().next.clone();
                            },None=>{panic!("unexpected None");}
                        }
                    }else{current=n.borrow().next.clone();}
                },None=>{break;}
            }
        }
    }pub fn flatten_binary_search_tree(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(_)=>{// variables initialized
                let head:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>=root.clone();
                let mut current:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                let mut collection:Stack=Stack::new();
                loop{// find head
                    match temp.clone().as_ref(){
                        Some(n)=>{
                            match n.borrow().prev.as_ref(){
                                Some(_)=>{
                                    temp=n.borrow().prev.clone();
                                },None=>{
                                    head=temp.clone();
                                    break;
                                }
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }current=root;
                loop{// flattening list
                    match current.clone().as_ref(){
                        Some(n)=>{
                            match collection.find(current.clone()).as_ref(){
                                Some(s)=>{
                                    if s.borrow().value==0{
                                        s.borrow_mut().value=1;
                                        current=n.borrow().next.clone();
                                    }else if s.borrow().value==1{
                                        collection.pop();
                                        match current.as_ref(){
                                            Some(o)=>{
                                                o.borrow_mut().parent=std::rc::Rc::new(None);
                                                o.borrow_mut().height=0;
                                                temp=o.borrow().next.clone();
                                                loop{
                                                    match temp.clone().as_ref(){
                                                        Some(d)=>{
                                                            match d.borrow().prev.as_ref(){
                                                                Some(_)=>{
                                                                    temp=d.borrow().prev.clone();
                                                                },None=>{break;}
                                                            }
                                                        },None=>{break;}
                                                    }
                                                }match temp.as_ref(){
                                                    Some(d)=>{
                                                        d.borrow_mut().prev=current.clone();
                                                        o.borrow_mut().next=temp.clone();
                                                    },None=>{}
                                                }
                                            },None=>{panic!("unexpected None");}
                                        }match current.as_ref(){
                                            Some(o)=>{
                                                temp=o.borrow().prev.clone();
                                                loop{
                                                    match temp.clone().as_ref(){
                                                        Some(d)=>{
                                                            match d.borrow().next.as_ref(){
                                                                Some(_)=>{
                                                                    temp=d.borrow().next.clone();
                                                                },None=>{break;}
                                                            }
                                                        },None=>{break;}
                                                    }
                                                }match temp.as_ref(){
                                                    Some(d)=>{
                                                        d.borrow_mut().next=current.clone();
                                                        o.borrow_mut().prev=temp.clone();
                                                    },None=>{}
                                                }
                                            },None=>{panic!("unexpected None");}
                                        }match collection.top.as_ref(){
                                            Some(h)=>{
                                                current=h.borrow().data.clone();
                                            },None=>{
                                                return head;
                                            }
                                        }
                                    }
                                },None=>{
                                    collection.push(current.clone(),0);
                                    current=n.borrow().prev.clone();
                                }
                            }
                        },None=>{
                            match collection.top.as_ref(){
                                Some(s)=>{
                                    current=s.borrow().data.clone();
                                },None=>{
                                    return head;
                                }
                            }
                        }
                    }
                }
            },None=>{return root;}
        }
    }pub fn flatten_binary_search_tree_in_place(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let mut current:std::rc::Rc<Option<std::cell::RefCell<Node>>>=root.clone();
        let mut previous:std::rc::Rc<Option<std::cell::RefCell<Node>>>
        =std::rc::Rc::new(None);
        let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        loop{
            match current.clone().as_ref(){
                Some(n)=>{
                    if match n.borrow().next.as_ref()
                    {Some(_)=>{true},None=>{false}}{
                        temp=n.borrow().next.clone();
                        loop{
                            match temp.clone().as_ref(){
                                Some(o)=>{
                                    match o.borrow().prev.as_ref(){
                                        Some(_)=>{
                                            if !std::rc::Rc::ptr_eq
                                            (&current,&o.borrow().prev)
                                            {temp=o.borrow().prev.clone();}
                                            else{break;}
                                        },None=>{break;}
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }match temp.as_ref(){
                            Some(o)=>{
                                if !std::rc::Rc::ptr_eq
                                (&current,&o.borrow().prev){
                                    o.borrow_mut().prev=current.clone();
                                    current=n.borrow().next.clone();
                                }else{
                                    o.borrow_mut().prev=current.clone();
                                    n.borrow_mut().next=temp.clone();
                                    previous=current.clone();
                                    current=n.borrow().prev.clone();
                                }
                            },None=>{panic!("unexpected None");}
                        }
                    }else{
                        match previous.as_ref(){
                            Some(o)=>{
                                n.borrow_mut().next=previous.clone();
                                o.borrow_mut().prev=current.clone();
                            },None=>{}
                        }previous=current.clone();
                        current=n.borrow().prev.clone();
                    }
                },None=>{return previous;}
            }
        }
    }pub fn bst_lca(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        a:i32,b:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                if n.borrow().value<a&&b>n.borrow().value{
                    return Tree::bst_lca(n.borrow().next.clone(),a,b);
                }else if n.borrow().value>a&&b<n.borrow().value{
                    return Tree::bst_lca(n.borrow().prev.clone(),a,b);
                }else{return root;}
            },None=>{return root;}
        }
    }pub fn balanced_bst_constructor(
        arr:&[i32]
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        if arr.len()>0{
            if arr.len()>1{
                let mut i:usize=0;let mut j:usize=arr.len()-1;
                loop{if i<j{i+=1;j-=1;}else{break;}}
                let root:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Node::new(arr[j]);
                match root.as_ref(){
                    Some(n)=>{
                        n.borrow_mut().prev=Tree::balanced_bst_constructor(&arr[..j]);
                        n.borrow_mut().next=Tree::balanced_bst_constructor(&arr[j+1..]);
                        return root;
                    },None=>{panic!("unexpected None");}
                }
            }else{return Node::new(arr[0]);}
        }else{return std::rc::Rc::new(None);}
    }pub fn bst_constructor_from_pre_order(arr:&[i32])
    ->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        if arr.len()>0{
            if arr.len()>1{
                let mut i:usize=0;
                let root:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Node::new(arr[0]);
                match root.as_ref(){
                    Some(n)=>{
                        loop{
                            if i<arr.len(){
                                if arr[i]>arr[0]
                                {break;}else{i+=1;}
                            }else{break;}
                        }n.borrow_mut().prev=Tree::bst_constructor_from_pre_order(&arr[1..i]);
                        n.borrow_mut().next=Tree::bst_constructor_from_pre_order(&arr[i..]);
                        return root;
                    },None=>{panic!("unexpected None");}
                }
            }else{return Node::new(arr[0]);}
        }else{return std::rc::Rc::new(None);}
    }pub fn merge_to_list(
        mut a:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        mut b:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let mut previous:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        match a.clone().as_ref(){
            Some(n)=>{
                match b.clone().as_ref(){
                    Some(o)=>{
                        if n.borrow().value>o.borrow().value{
                            previous=a.clone();
                            a=b.clone();
                            b=previous.clone();
                            n.borrow_mut().prev=Tree::new();
                            o.borrow_mut().prev=Tree::new();
                        }
                    },None=>{return a;}
                }
            },None=>{return b;}
        }let head:std::rc::Rc<Option<std::cell::RefCell<Node>>>=a.clone();loop{
            match a.clone().as_ref(){
                Some(n)=>{
                    match b.clone().as_ref(){
                        Some(o)=>{
                            if n.borrow().value>o.borrow().value{
                                match previous.clone().as_ref(){
                                    Some(d)=>{
                                        d.borrow_mut().next=b.clone();
                                        n.borrow_mut().prev=b.clone();
                                        b=o.borrow().next.clone();
                                        o.borrow_mut().prev=previous.clone();
                                        o.borrow_mut().next=a.clone();
                                        previous=d.borrow().next.clone();
                                    },None=>{panic!("unexpected None");}
                                }
                            }else{
                                previous=a.clone();
                                a=n.borrow().next.clone();
                            }
                        },None=>{break;}
                    }
                },None=>{break;}
            }
        }if match b.as_ref(){Some(_)=>{true},None=>{false}}{
            match previous.as_ref(){
                Some(n)=>{
                    n.borrow_mut().next=b.clone();
                },None=>{panic!("unexpected None");}
            }
        }return head;
    }pub fn merge_two_bst(
        mut a:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        mut b:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        a=Tree::flatten_binary_search_tree_in_place(a.clone());
        b=Tree::flatten_binary_search_tree_in_place(b.clone());
        return Tree::balanced_bst_constructor(Node::collect(Tree::merge_to_list(a,b)).as_slice());
    }pub fn tallest_bst_height(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->Option<TreeInfo>{
        match root.as_ref(){
            Some(n)=>{
                let left:Option<TreeInfo>=Tree::tallest_bst_height(n.borrow().prev.clone());
                let right:Option<TreeInfo>=Tree::tallest_bst_height(n.borrow().next.clone());
                let mut info:Option<TreeInfo>=Some(TreeInfo::new(
                    true,0,n.borrow().value,n.borrow().value
                ));match &mut info{
                    Some(i)=>{
                        if match &left{Some(ti)=>{ti.bst},None=>{true}}
                        &&match &right{Some(ti)=>{ti.bst},None=>{true}}
                        {}else{i.bst=false;}match &left{
                            Some(ti)=>{
                                if n.borrow().value>ti.max{
                                    i.min=ti.min;
                                }else{i.bst=false;}
                            },None=>{}
                        }match &right{
                            Some(ti)=>{
                                if ti.min>n.borrow().value{
                                    i.max=ti.max;
                                }else{i.bst=false;}
                            },None=>{}
                        }if i.bst{
                            i.height=max(
                                match &left{Some(ti)=>{ti.height},None=>{-1}},
                                match &right{Some(ti)=>{ti.height},None=>{-1}}
                            )+1;return info;
                        }else{
                            match &left{
                                Some(l)=>{
                                    match &right{
                                        Some(r)=>{
                                            if l.height>r.height{
                                                return left;
                                            }else{return right;}
                                        },None=>{return left;}
                                    }
                                },None=>{return right;}
                            }
                        }
                    },None=>{panic!("unexpected None");}
                }
            },None=>{return None;}
        }
    }
}impl Tree{// Debug implementation
    pub fn parent_check(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(n)=>{
                Tree::parent_check(n.borrow().prev.clone());
                println!(
                    "[n:({}),p:({})]",
                    n.borrow().value.to_string(),
                    match n.borrow().parent.as_ref(){
                        Some(o)=>{o.borrow().value.to_string()},
                        None=>{"None".to_string()}
                    }
                );Tree::parent_check(n.borrow().next.clone());
            },None=>{}
        }
    }pub fn info(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){// info(in order);
        match root.as_ref(){
            Some(n)=>{
                Tree::info(n.borrow().prev.clone());
                println!("[value]:[{}],[height]:[{}],[prev]:[{}],[next]:[{}],[parent]:[{}]",
                    n.borrow().value,
                    n.borrow().height,
                    match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().value.to_string()
                        },None=>{"x".to_string()}
                    },match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().value.to_string()
                        },None=>{"x".to_string()}
                    },match n.borrow().parent.as_ref(){
                        Some(o)=>{
                            o.borrow().value.to_string()
                        },None=>{"x".to_string()}
                    }
                );Tree::info(n.borrow().next.clone());
            },None=>{println!("[None]");}
        }
    }pub fn list_checker(head:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->bool{
        let tail:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        let mut temp:std::rc::Rc<Option<std::cell::RefCell<Node>>>=head.clone();
        print!("[ ");
        loop{
            match temp.clone().as_ref(){
                Some(n)=>{
                    print!("{:#?} ",n.borrow().value);
                    match n.borrow().next.as_ref(){
                        Some(_)=>{
                            temp=n.borrow().next.clone();
                        },None=>{break;}
                    }
                },None=>{
                    println!("empty list ]");
                    return false;
                }
            }
        }print!("]\n[ ");
        tail=temp.clone();
        loop{
            match temp.clone().as_ref(){
                Some(n)=>{
                    print!("{:#?} ",n.borrow().value);
                    match n.borrow().prev.as_ref(){
                        Some(_)=>{
                            temp=n.borrow().prev.clone();
                        },None=>{break;}
                    }
                },None=>{
                    println!("empty list ]");
                    return false;
                }
            }
        }print!("]\n");
        println!(
            "head={},tail={}",
            match head.as_ref(){
                Some(n)=>{
                    n.borrow().value
                },None=>{-1}
            },match tail.as_ref(){
                Some(n)=>{
                    n.borrow().value
                },None=>{-1}
            }
        );
        if std::rc::Rc::ptr_eq(&head,&temp){
            println!("list");
            return true;
        }else{
            println!("not list");
            return false;
        }
    }
}// --heap--;
pub mod arr_impl{
    use super::max;
    #[derive(Debug)]
    pub struct Heap{
        pub tree:Vec<i32>,pub end:usize
    }impl Heap{
        pub fn new(size:usize)->Heap{
            return Heap{
                tree:vec![0;size+1],end:0
            };
        }pub fn insert(&mut self,value:i32)->bool{
            if self.end<self.tree.len()-1{
                self.end+=1;
                let mut i:usize=self.end;
                let mut temp:i32;
                self.tree[self.end]=value;
                loop{
                    if i>1{
                        if self.tree[i]>self.tree[i/2]{
                            temp=self.tree[i/2];
                            self.tree[i/2]=self.tree[i];
                            self.tree[i]=temp;i=i/2;
                        }else{i=i/2;}
                    }else{break;}
                }return true;
            }else{return false;}
        }pub fn pop(&mut self)->Option<i32>{
            if self.end>0{
                let mut i:usize=1;
                let mut temp:i32;
                let value:i32=self.tree[1];
                self.tree[1]=self.tree[self.end];
                self.tree[self.end]=-1;
                self.end-=1;loop{
                    if i<self.end/2+1{
                        if max(self.tree[i*2],self.tree[i*2+1])==self.tree[i*2+1]{
                            if self.tree[i*2+1]>self.tree[i]{
                                temp=self.tree[i*2+1];
                                self.tree[i*2+1]=self.tree[i];
                                self.tree[i]=temp;i=i*2+1;
                            }else{break;}
                        }else{
                            if self.tree[i*2]>self.tree[i]{
                                temp=self.tree[i*2];
                                self.tree[i*2]=self.tree[i];
                                self.tree[i]=temp;i=i*2;
                            }else{break;}
                        }
                    }else{break;}
                }return Some(value);
            }else{return None;}
        }pub fn heapify(&mut self){
            if self.end>1{
                let mut i:usize=self.end/2;
                let mut j:usize;// [_!];
                let mut temp:i32;loop{
                    if i>0{
                        if max(self.tree[i*2],self.tree[i*2+1])==self.tree[i*2+1]{
                            if self.tree[i*2+1]>self.tree[i]{
                                temp=self.tree[i*2+1];
                                self.tree[i*2+1]=self.tree[i];
                                self.tree[i]=temp;j=i;i=i*2+1;
                                loop{
                                    if i<self.end/2+1{
                                        if max(self.tree[i*2],self.tree[i*2+1])==self.tree[i*2+1]{
                                            if self.tree[i*2+1]>self.tree[i]{
                                                temp=self.tree[i*2+1];
                                                self.tree[i*2+1]=self.tree[i];
                                                self.tree[i]=temp;i=i*2+1;
                                            }else{break;}
                                        }else{
                                            if self.tree[i*2]>self.tree[i]{
                                                temp=self.tree[i*2];
                                                self.tree[i*2]=self.tree[i];
                                                self.tree[i]=temp;i=i*2;
                                            }else{break;}
                                        }
                                    }else{break;}
                                }i=j;
                            }else{}
                        }else{
                            if self.tree[i*2]>self.tree[i]{
                                temp=self.tree[i*2];
                                self.tree[i*2]=self.tree[i];
                                self.tree[i]=temp;j=i;i=i*2;
                                loop{
                                    if i<self.end/2+1{
                                        if max(self.tree[i*2],self.tree[i*2+1])==self.tree[i*2+1]{
                                            if self.tree[i*2+1]>self.tree[i]{
                                                temp=self.tree[i*2+1];
                                                self.tree[i*2+1]=self.tree[i];
                                                self.tree[i]=temp;i=i*2+1;
                                            }else{break;}
                                        }else{
                                            if self.tree[i*2]>self.tree[i]{
                                                temp=self.tree[i*2];
                                                self.tree[i*2]=self.tree[i];
                                                self.tree[i]=temp;i=i*2;
                                            }else{break;}
                                        }
                                    }else{break;}
                                }i=j;
                            }else{}
                        }i-=1;
                    }else{break;}
                }println!("arr:{:?}",self.tree);
                return;
            }else{return;}
        }pub fn kth_min_int(arr:&[i32],k:usize)->i32{
            if !k>arr.len(){
                let mut collection:Heap=Heap::new(k);
                let mut i:usize=0;loop{
                    if i<k{
                        collection.insert(arr[i]);
                        i+=1;
                    }else{break;}
                }loop{
                    if i<arr.len(){
                        if arr[i]<collection.tree[1]{
                            collection.pop();
                            collection.insert(arr[i]);
                        }i+=1;
                    }else{break;}
                }return collection.tree[1];
            }else{panic!("array index out of bound");}
        }
    }
}// mod_arr_impl;
#[derive(Debug)]
pub struct Heap;
impl Heap{
    pub fn heapify(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(n)=>{
                let temp:i32;
                Heap::heapify(n.borrow().prev.clone());
                Heap::heapify(n.borrow().next.clone());
                let left:std::rc::Rc<Option<std::cell::RefCell<Node>>>=n.borrow().prev.clone();
                let right:std::rc::Rc<Option<std::cell::RefCell<Node>>>=n.borrow().next.clone();
                match left.as_ref(){
                    Some(o)=>{
                        match right.as_ref(){
                            Some(d)=>{
                                if max(o.borrow().value,d.borrow().value)==o.borrow().value{
                                    if o.borrow().value>n.borrow().value{
                                        temp=n.borrow().value;
                                        n.borrow_mut().value=o.borrow().value;
                                        o.borrow_mut().value=temp;
                                    }else{return;}
                                }else{
                                    if d.borrow().value>n.borrow().value{
                                        temp=n.borrow().value;
                                        n.borrow_mut().value=d.borrow().value;
                                        d.borrow_mut().value=temp;
                                    }else{return;}
                                }
                            },None=>{
                                if o.borrow().value>n.borrow().value{
                                    temp=n.borrow().value;
                                    n.borrow_mut().value=o.borrow().value;
                                    o.borrow_mut().value=temp;
                                }else{return;}
                            }
                        }
                    },None=>{
                        match right.as_ref(){
                            Some(o)=>{
                                if o.borrow().value>n.borrow().value{
                                    temp=n.borrow().value;
                                    n.borrow_mut().value=o.borrow().value;
                                    o.borrow_mut().value=temp;
                                }else{return;}
                            },None=>{return;}
                        }
                    }
                }
            },None=>{return;}
        }
    }pub fn insert(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,value:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let mut collection:Queue=Queue::new();
        collection.enqueue(0,root.clone());
        loop{
            match collection.dequeue().as_ref(){
                Some(s)=>{
                    match s.borrow().data.as_ref(){
                        Some(n)=>{
                            if match n.borrow().prev.clone().as_ref()
                            {Some(_)=>{true},None=>{false}}{
                                if match n.borrow().next.as_ref(){Some(_)=>{true},None=>{false}}{
                                    collection.enqueue(0,n.borrow().prev.clone());
                                    collection.enqueue(0,n.borrow().next.clone());
                                }else{
                                    n.borrow_mut().next=Node::new(value);
                                    break;
                                }
                            }else{
                                n.borrow_mut().prev=Node::new(value);
                                break;
                            }
                        },None=>{return Node::new(value);}
                    }
                },None=>{panic!("unexpected None");}
            }
        }Heap::heapify(root.clone());
        return root;
    }pub fn pop(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                let temp:i32;
                let mut collection:Queue=Queue::new();
                let mut previous:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
                collection.enqueue(0,root.clone());
                loop{
                    match collection.dequeue().as_ref(){
                        Some(s)=>{
                            match s.borrow().data.as_ref(){
                                Some(o)=>{
                                    if match o.borrow().prev.as_ref()
                                    {Some(_)=>{true},None=>{false}}{
                                        collection.enqueue(0,o.borrow().prev.clone());
                                        if match o.borrow().next.as_ref()
                                        {Some(_)=>{true},None=>{false}}{
                                            collection.enqueue(0,o.borrow().next.clone());
                                            previous=s.borrow().data.clone();
                                        }else{
                                            match o.borrow().prev.as_ref(){
                                                Some(d)=>{
                                                    n.borrow_mut().value=d.borrow().value;
                                                },None=>{panic!("unexpected None");}
                                            }o.borrow_mut().prev=Tree::new();
                                            Heap::heapify(root.clone());
                                            return root;
                                        }
                                    }else{
                                        match previous.as_ref(){
                                            Some(d)=>{
                                                match d.borrow().next.as_ref(){
                                                    Some(e)=>{
                                                        temp=e.borrow().value;
                                                    },None=>{panic!("unexpected None");}
                                                }n.borrow_mut().value=temp;
                                                d.borrow_mut().next=Tree::new();
                                                Heap::heapify(root.clone());
                                                return root;
                                            },None=>{return Tree::new();}
                                        }
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }
            },None=>{return root;}
        }
    }pub fn is_heap(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>)->bool{
        match root.as_ref(){
            Some(n)=>{
                if Heap::is_heap(n.borrow().prev.clone())
                &&Heap::is_heap(n.borrow().next.clone()){
                    match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            if o.borrow().value>n.borrow().value{return false;}
                        },None=>{}
                    }match n.borrow().next.as_ref(){
                        Some(o)=>{
                            if o.borrow().value>n.borrow().value{return false;}
                        },None=>{}
                    }return true;
                }else{return false;}
            },None=>{return true;}
        }
    }
}#[derive(Debug)]
pub struct MinHeap;impl MinHeap{
    pub fn heapify(root:std::rc::Rc<Option<std::cell::RefCell<Node>>>){
        match root.as_ref(){
            Some(n)=>{
                let mut temp:i32;
                let left:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                left=n.borrow().prev.clone();
                let right:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                right=n.borrow().next.clone();
                MinHeap::heapify(n.borrow().prev.clone());
                MinHeap::heapify(n.borrow().next.clone());
                match left.as_ref(){
                    Some(o)=>{
                        match right.as_ref(){
                            Some(d)=>{
                                if min(o.borrow().value,d.borrow().value)==o.borrow().value{
                                    if o.borrow().value<n.borrow().value{
                                        temp=o.borrow().value;
                                        o.borrow_mut().value=n.borrow().value;
                                        n.borrow_mut().value=temp;
                                        temp=o.borrow().height;
                                        o.borrow_mut().height=n.borrow().height;
                                        n.borrow_mut().height=temp;
                                        MinHeap::heapify(left);
                                    }return;
                                }else{
                                    if d.borrow().value<n.borrow().value{
                                        temp=d.borrow().value;
                                        d.borrow_mut().value=n.borrow().value;
                                        n.borrow_mut().value=temp;
                                        temp=d.borrow().height;
                                        d.borrow_mut().height=n.borrow().height;
                                        n.borrow_mut().height=temp;
                                        MinHeap::heapify(right);
                                    }return;
                                }
                            },None=>{
                                if o.borrow().value<n.borrow().value{
                                    temp=o.borrow().value;
                                    o.borrow_mut().value=n.borrow().value;
                                    n.borrow_mut().value=temp;
                                    temp=o.borrow().height;
                                    o.borrow_mut().height=n.borrow().height;
                                    n.borrow_mut().height=temp;
                                    MinHeap::heapify(left);
                                }return;
                            }
                        }
                    },None=>{return;}
                }
            },None=>{return;}
        }
    }pub fn insert(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
        value:i32,index:i32
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let node:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Node::new(value);
        let mut collection:Queue=Queue::new();collection.enqueue(0,root.clone());
        match node.as_ref(){
            Some(n)=>{
                n.borrow_mut().height=index;
            },None=>{panic!("unexpected None");}
        }loop{
            match collection.dequeue().as_ref(){
                Some(s)=>{
                    match s.borrow().data.clone().as_ref(){
                        Some(n)=>{
                            if match n.borrow().prev.as_ref()
                            {Some(_)=>{true},None=>{false}}{
                                collection.enqueue(0,n.borrow().prev.clone());
                            }else{
                                n.borrow_mut().prev=node.clone();
                                MinHeap::heapify(root.clone());
                                return root;
                            }if match n.borrow().next.as_ref()
                            {Some(_)=>{true},None=>{false}}{
                                collection.enqueue(0,n.borrow().next.clone());
                            }else{
                                n.borrow_mut().next=node.clone();
                                MinHeap::heapify(root.clone());
                                return root;
                            }
                        },None=>{return node;}
                    }
                },None=>{panic!("unexpected None");}
            }
        }
    }pub fn pop(
        root:std::rc::Rc<Option<std::cell::RefCell<Node>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        match root.as_ref(){
            Some(n)=>{
                let mut collection:Queue=Queue::new();
                let mut previous:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
                let temp:i32;let val:i32;collection.enqueue(0,root.clone());
                let mut left:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                let mut right:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
                loop{
                    match collection.dequeue().as_ref(){
                        Some(s)=>{
                            match s.borrow().data.as_ref(){
                                Some(o)=>{
                                    left=o.borrow().prev.clone();
                                    right=o.borrow().next.clone();
                                    match left.as_ref(){
                                        Some(d)=>{
                                            match right.as_ref(){
                                                Some(_)=>{
                                                    collection.enqueue(0,left.clone());
                                                    collection.enqueue(0,right.clone());
                                                    previous=s.borrow().data.clone();
                                                },None=>{
                                                    n.borrow_mut().value=d.borrow().value;
                                                    n.borrow_mut().height=d.borrow().height;
                                                    o.borrow_mut().prev=Tree::new();
                                                    MinHeap::heapify(root.clone());
                                                    return root;
                                                }
                                            }
                                        },None=>{
                                            match previous.as_ref(){
                                                Some(d)=>{
                                                    match d.borrow().next.as_ref(){
                                                        Some(e)=>{
                                                            temp=e.borrow().value;
                                                            val=e.borrow().height;
                                                        },None=>{panic!("unexpected None");}
                                                    }n.borrow_mut().value=temp;
                                                    n.borrow_mut().height=val;
                                                    d.borrow_mut().next=Tree::new();
                                                    MinHeap::heapify(root.clone());
                                                    return root;
                                                },None=>{return Tree::new();}
                                            }
                                        }
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }
            },None=>{return root;}
        }
    }pub fn merge_k_sorted_list(
        mut lists:Vec<std::rc::Rc<Option<std::cell::RefCell<Node>>>>
    )->std::rc::Rc<Option<std::cell::RefCell<Node>>>{
        let mut collection:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut head:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut tail:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut i:usize=0;loop{
            if i<lists.len(){
                match lists[i].as_ref(){
                    Some(n)=>{
                        collection=MinHeap::insert(collection.clone(),n.borrow().value,i as i32);
                        i+=1;
                    },None=>{}
                }
            }else{break;}
        }loop{
            match collection.clone().as_ref(){
                Some(n)=>{
                    i=n.borrow().height as usize;
                    match lists[i].clone().as_ref(){
                        Some(o)=>{
                                match tail.clone().as_ref(){
                                    Some(d)=>{
                                        d.borrow_mut().next=lists[i].clone();
                                        o.borrow_mut().prev=tail.clone();
                                        tail=lists[i].clone();
                                    },None=>{
                                        tail=lists[i].clone();
                                        head=tail.clone();
                                    }
                                }lists[i]=o.borrow().next.clone();
                                collection=MinHeap::pop(collection.clone());
                            },None=>{panic!("unexpected None");}
                        }match lists[i].clone().as_ref(){
                            Some(o)=>{
                                collection=MinHeap::insert(collection.clone(),o.borrow().value,i as i32);
                            },None=>{}
                        }
                },None=>{break;}
            }
        }return head;
    }pub fn min_range(mut arrays:Vec<std::rc::Rc<Option<std::cell::RefCell<Node>>>>)->i32{
        let mut collection:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut max:i32=match arrays[0].as_ref(){Some(n)=>{n.borrow().value},None=>{return -1;}};
        let mut i:usize=0;let mut range:i32=-1;loop{    
            if i<arrays.len(){
                match arrays[i].as_ref(){
                    Some(n)=>{
                        collection=MinHeap::insert(collection.clone(),n.borrow().value,i as i32);
                        if max<n.borrow().value{max=n.borrow().value;}i+=1;
                    },None=>{break;}
                }
            }else{break;}
        }println!("max:{}",max);
        loop{
            match collection.clone().as_ref(){
                Some(n)=>{i=n.borrow().height as usize;},None=>{return range;}
            }match arrays[i].clone().as_ref(){
                Some(n)=>{
                    println!("i={}",i);
                    collection=MinHeap::pop(collection.clone());
                    println!("max={}",max);
                    if max-n.borrow().value<range||0>range{
                        range=max-n.borrow().value;
                        println!("max={}&&n={}",max,n.borrow().value);
                    }
                    println!("n={}",n.borrow().value);
                    println!("range={}",range);
                    arrays[i]=n.borrow().next.clone();
                    match n.borrow().next.clone().as_ref(){
                        Some(o)=>{
                            println!("next={}",o.borrow().value);
                            collection=MinHeap::insert(collection.clone(),o.borrow().value,i as i32);
                            if o.borrow().value>max{max=o.borrow().value;println!("max>>{}",max);}
                        },None=>{return range;}
                    }
                },None=>{panic!("unexpected None");}
            }
        }
    }pub fn median_in_stream(){
        let mut arr:Vec<i32>=Vec::new();
        let mut maxi:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut mini:std::rc::Rc<Option<std::cell::RefCell<Node>>>=Tree::new();
        let mut value:i32;let mut command:String;
        let mut max_height:i32=0;let mut min_height:i32=0;
        loop{
            println!("[command]");
            command=input();
            if command=="insert"{
                value=intput();
                if value<0{return;}
                else{arr.push(value);
                    match maxi.as_ref(){
                        Some(n)=>{
                            match mini.as_ref(){
                                Some(o)=>{
                                    if value>
                                    if signum(max_height,min_height)==0{(n.borrow().value+o.borrow().value)/2}
                                    else if signum(max_height,min_height)==1{n.borrow().value}
                                    else{o.borrow().value}{
                                        if signum(max_height,min_height)<0{
                                            maxi=Heap::insert(maxi.clone(),o.borrow().value);
                                            mini=MinHeap::pop(mini.clone());
                                            mini=MinHeap::insert(mini.clone(),value,0);
                                            max_height+=1;
                                        }else{
                                            mini=MinHeap::insert(mini.clone(),value,0);
                                            min_height+=1;
                                        }
                                    }else{
                                        if signum(max_height,min_height)>0{
                                            mini=MinHeap::insert(mini.clone(),n.borrow().value,0);
                                            maxi=Heap::pop(maxi.clone());
                                            maxi=Heap::insert(maxi.clone(),value);
                                        }else{
                                            maxi=Heap::insert(maxi.clone(),value);
                                            max_height+=1;
                                        }
                                    }
                                },None=>{
                                    if value>n.borrow().value{
                                        mini=MinHeap::insert(mini.clone(),value,0);
                                        min_height+=1;
                                    }else{
                                        mini=MinHeap::insert(mini.clone(),n.borrow().value,0);
                                        maxi=Heap::pop(maxi.clone());
                                        maxi=Heap::insert(maxi.clone(),value);min_height+=1;
                                    }
                                }
                            }
                        },None=>{
                            maxi=Heap::insert(maxi.clone(),value);
                            max_height+=1;
                        }
                    }
                }
            }else if command=="median"{
                println!(
                    "[{}]",
                    match maxi.as_ref(){
                        Some(n)=>{
                            match mini.as_ref(){
                                Some(o)=>{
                                    if signum(max_height,min_height)==0{(n.borrow().value+o.borrow().value)/2}
                                    else if signum(max_height,min_height)==1{n.borrow().value}
                                    else{o.borrow().value}
                                },None=>{n.borrow().value}
                            }
                        },None=>{-1}
                    }
                );
            }else if command=="log"{
                println!("{:?}",arr);
            }else{
                println!("unknown command")
            }
        }
    }
}
pub mod graph{
    use super::{map::{HashTable,node,Node,Hash},bgd::*};
    #[derive(Debug)]
    pub struct AdjacencyList<T:Hash+Clone>{pub list:HashTable<T,node<T>>}
    impl Hash for i32{fn hashcode(&self)->usize{return*self as usize;}}
    impl<T:Clone+Hash>AdjacencyList<T>{
        pub fn new()->AdjacencyList<T>{
            return AdjacencyList{list:HashTable::new()};
        }pub fn add_edge(&mut self,a:T,b:T,directed:bool)where T:PartialEq{
            match self.list.get(a.clone()){
                Some(mut target)=>{
                    loop{
                        match target.clone().as_ref(){
                            Some(n)=>{
                                if n.borrow().value
                                ==b{break;}else{
                                    if match n.borrow().next.as_ref()
                                    {Some(_)=>{true},None=>{false}}
                                    {target=n.borrow().next.clone();}else
                                    {n.borrow_mut().next=Node::new(b.clone());break;}
                                }
                            },None=>{
                                self.list.modify(a.clone(),Node::new(b.clone()));
                                break;
                            }
                        }
                    }
                },None=>{self.list.insert(a.clone(),Node::new(b.clone()));}
            }if !directed{
                match self.list.get(b.clone()){
                    Some(mut target)=>{
                        loop{
                            match target.clone().as_ref(){
                                Some(n)=>{
                                    if n.borrow().value
                                    ==a{break;}else{
                                        if match n.borrow().next.as_ref()
                                        {Some(_)=>{true},None=>{false}}
                                        {target=n.borrow().next.clone();}else
                                        {n.borrow_mut().next=Node::new(a);break;}
                                    }
                                },None=>{
                                    self.list.modify(b,Node::new(a));
                                    break;
                                }
                            }
                        }
                    },None=>{self.list.insert(b,Node::new(a));}
                }
            }else{
                match self.list.get(b.clone()){
                    Some(_)=>{},None=>
                    {self.list.insert(b,Node::blank());}
                }
            }
        }
    }impl<T:std::fmt::Display+Hash+Clone>std::fmt::Display for AdjacencyList<T>{
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
    }impl<T:Clone+Hash>AdjacencyList<T>{// 
        pub fn detect_undirected_cycle(&self)where T:std::fmt::Display+PartialEq{
            //! undirected graph bfs traversal approach
            let mut visited:HashTable<T,()>=HashTable::new();
            let mut queue:Queue<T>=Queue::new();
            let mut parents:HashTable<T,T>=HashTable::new();
            for(value,_)in self.list.iter(){
                match visited.get(value.clone())
                {Some(_)=>{},None=>{queue.enqueue(value);}}
                loop{
                    match queue.dequeue().as_ref(){
                        Some(n)=>{
                            match self.list.get(n.borrow().value.clone()){
                                Some(mut edge)=>{
                                    loop{
                                        match edge.clone().as_ref(){
                                            Some(o)=>{
                                                match visited.get(o.borrow().value.clone()){
                                                    Some(_)=>{
                                                        match parents.get(n.borrow().value.clone()){
                                                            Some(p)=>{
                                                                if o.borrow().value.clone()!=p{
                                                                    println!(
                                                                        "cycle found at {}",
                                                                        o.borrow().value.clone()
                                                                    );return;
                                                                }
                                                            },None=>{panic!("unexpected None");}
                                                        }
                                                    },None=>{
                                                        parents.insert(
                                                            o.borrow().value.clone(),
                                                            n.borrow().value.clone()
                                                        );queue.enqueue(o.borrow().value.clone());
                                                    }
                                                }edge=o.borrow().next.clone();
                                            },None=>{break;}
                                        }
                                    }
                                },None=>{panic!("unexpected None");}
                            }visited.insert(n.borrow().value.clone(),());
                        },None=>{break;}
                    }
                }
            }println!("no cycle detected");
        }pub fn detect_directed_cycle
        (&self,position:T,mut visited:HashTable<T,()>,mut syscall:Stack<T>)
        ->(HashTable<T,()>,Stack<T>)where T:std::fmt::Display+PartialEq{
            //! recursive dfs traversal approach on directed graph
            if syscall.query(position.clone()){
                println!("cycle detected at {position}");return (visited,syscall);
            }else{
                match visited.get(position.clone()){
                    Some(_)=>{
                    },None=>{
                        syscall.push(position.clone());
                        visited.insert(position.clone(),());
                        for temp in Node
                        ::iter(self.list.get(position.clone()).unwrap()){
                            match temp.as_ref(){
                                Some(n)=>{
                                    (visited,syscall)=self.detect_directed_cycle(n.borrow().value.clone(),visited,syscall);
                                },None=>{}
                            }
                        }syscall.pop();
                    }
                }
            }
            println!("no cycle detected at {position}");
            return (visited,syscall);
        }pub fn topological_traversal(&self,position:T,mut visited:HashTable<T,()>,mut ans:Stack<T>)
        ->(HashTable<T,()>,Stack<T>)where T:PartialEq{
            match visited.get(position.clone()){
                Some(_)=>{return(visited,ans);}
                ,None=>{
                    visited.insert(position.clone(),());
                    for temp in Node::iter(self.list.get(position.clone()).unwrap()){
                        match temp.as_ref(){
                            Some(n)=>{
                                (visited,ans)=self.topological_traversal(n.borrow().value.clone(),visited,ans)
                            },None=>{}
                        }
                    }ans.push(position.clone());
                    return(visited,ans);
                }
            }
        }pub fn topological_traversal_in_place(&self)->Stack<T>where T:std::fmt::Display+PartialEq{
            let mut queue:Queue<T>=Queue::new();
            let mut visited:Stack<T>=Stack::new();
            let mut indegrees:HashTable<T,u8>=HashTable::new();
            for(vortex,edges)in self.list.iter(){
                indegrees.or_insert(vortex,0);
                for edge in Node::iter(edges){
                    match edge.as_ref(){
                        Some(n)=>{
                            match indegrees.get(n.borrow().value.clone()){
                                Some(indegree)=>{
                                    indegrees.modify(n.borrow().value.clone(),indegree+1);
                                },None=>{
                                    indegrees.insert(n.borrow().value.clone(),1);
                                }
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }
            }println!("{}\n",indegrees);
            for(vortex,_)in self.list.iter(){
                if!visited.query(vortex.clone()){
                    match indegrees.get(vortex.clone()){
                        Some(indegree)=>{
                            if indegree==0{
                                queue.enqueue(vortex);
                            }
                        },None=>{panic!("unexpected None");}
                    }
                }loop{
                    match queue.dequeue().as_ref(){
                        Some(n)=>{
                            println!("dequeued:{}",n.borrow().value.clone());
                            if!(visited.query(n.borrow().value.clone())){
                                visited.push(n.borrow().value.clone());
                                for edge in Node::iter(self.list.get(n.borrow().value.clone()).unwrap()){
                                    match edge.as_ref(){
                                        Some(o)=>{
                                            match indegrees.get(o.borrow().value.clone()){
                                                Some(indegree)=>{
                                                    println!("n:{},o:{},id:{}",n.borrow().value.clone(),o.borrow().value.clone(),indegree);
                                                    if indegree>0{
                                                        indegrees.modify(o.borrow().value.clone(),indegree-1);
                                                    }if indegrees.get(o.borrow().value.clone()).unwrap()==0{
                                                        queue.enqueue(o.borrow().value.clone());
                                                    }println!("{}",indegrees);
                                                },None=>{panic!("unexpected None");}
                                            }
                                        },None=>{panic!("unexpected None");}
                                    }
                                }
                            }
                        },None=>{break;}
                    }
                }
            }
            println!("{}",visited);
            return Stack::new();
        }pub fn shortest_path(&self,beginning:T,mut destination:T)where T:std::fmt::Display+PartialEq{
            let mut parents:HashTable<T,T>=HashTable::new();
            let mut visited:HashTable<T,()>=HashTable::new();
            let mut queue:Queue<T>=Queue::new();
            let mut path:Stack<T>=Stack::new();
            queue.enqueue(beginning.clone());
            loop{
                match queue.dequeue().as_ref(){
                    Some(n)=>{
                        visited.insert(n.borrow().value.clone(),());
                        for edge in Node::iter
                        (self.list.get(n.borrow().value.clone()).unwrap()){
                            match edge.as_ref(){
                                Some(o)=>{
                                    match visited.get(o.borrow().value.clone()){
                                        Some(_)=>{},None=>{
                                            queue.enqueue(o.borrow().value.clone());
                                            parents.or_insert(o.borrow().value.clone(),n.borrow().value.clone());
                                        }
                                    }
                                },None=>{panic!("unexpected None");}
                            }
                        }
                    },None=>{break;}
                }
            }
            // println!("{}\n{}",path,parents);
            loop{
                match parents.get(destination.clone()){
                    Some(value)=>{
                        path.push(destination.clone());
                        destination=value;
                    },None=>{
                        path.push(beginning);
                        println!("{}\n{}",path,parents);
                        return;
                    }
                }
            }
        }
    }
}