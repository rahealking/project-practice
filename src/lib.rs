pub use std;
pub fn max(a:i32,b:i32)->i32{return if a>b{a}else{b};}
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
    }
}
#[derive(Debug)]
pub struct Shell{
    pub data:std::rc::Rc<Option<std::cell::RefCell<Node>>>,
    pub next:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
    pub prev:std::rc::Rc<Option<std::cell::RefCell<Shell>>>,
    pub value:i32
}impl Shell{
    pub fn new(data:i32,
        value:std::rc::Rc<Option<std::cell::RefCell<Node>>>
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
                    match n.borrow().prev.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    },
                    match n.borrow().next.as_ref(){
                        Some(o)=>{
                            o.borrow().height
                        },None=>{-1}
                    }
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
        let mut temp_a:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        temp_a=std::rc::Rc::new(None);
        let mut temp_b:std::rc::Rc<Option<std::cell::RefCell<Node>>>;
        temp_b=std::rc::Rc::new(None);
        if std::rc::Rc::ptr_eq(&root,&a){
            temp_a=root.clone();
        }if std::rc::Rc::ptr_eq(&root,&b){
            temp_b=root.clone();
        }
        match root.as_ref(){
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
            },None=>{}
        }
        return std::rc::Rc::new(None);
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
        loop{
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
    }
}