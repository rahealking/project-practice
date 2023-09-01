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