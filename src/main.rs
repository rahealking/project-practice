use project_practice::{backtracing::Position,bgd::{AdjacencyList,Stack,finites},map::HashTable};
mod color{
    #[derive(Debug)]
    pub struct Rgb{
        pub red:u8,pub green:u8,pub blue:u8
    }impl Rgb{
        pub fn rgb(r:u8,g:u8,b:u8)->Rgb
        {return Rgb{red:r,green:g,blue:b};}
    }impl std::fmt::Display for Rgb{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(f,"{}",format!("({}),({}),({})",self.red,self.green,self.blue));
        }
    }
}
pub trait Draw
where Self:ToString+Sized{
    fn show(&self);
}
#[derive(Debug)]
pub struct piratical{
    height:u128,length:u128,
    width:u128,density:u32,
    opacity:u16,color:color::Rgb,
    duration:u8,coordinate:Position
}impl std::fmt::Display for piratical{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f,"{:#?}",self);
    }
}impl Draw for piratical{
    fn show(&self){
        println!("{:#?}",self.to_string());
    }
}impl Default for piratical{
    fn default()->Self{
        return piratical{
            height:0,
            length:0,
            width:0,
            density:0,
            opacity:0,
            color:color::Rgb::rgb(0,0,0),
            duration:0,
            coordinate:Position::new(0,0,0)
        };
    }
}impl finites for piratical{
    const MAX:Self=piratical{
        height:u128::MAX,
        length:u128::MAX,
        width:u128::MAX,
        density:u32::MAX,
        opacity:u16::MAX,
        color:color::Rgb{red:125,green:125,blue:125},
        duration:125,
        coordinate:Position{
            x:usize::MAX,y:usize::MAX,z:usize::MAX
        }
    };const Min:Self=piratical{
        height:0,
        length:0,
        width:0,
        density:0,
        opacity:0,
        color:color::Rgb{red:0,green:0,blue:0},
        duration:0,
        coordinate:Position{
            x:0,y:0,z:0
        }
    };
}

fn main(){
    let mut a=<piratical as finites>::MAX;
    let mut b=<piratical as finites>::Min;
    println!("{a}");
}

/*
fn main(){
    let mut a=AdjacencyList::new();
    // a.add_edge(1,2,3,true);
    // a.add_edge(1,3,2,true);
    // a.add_edge(3,4,5,true);
    // a.add_edge(2,4,7,true);
    // a.add_edge(4,5,1,true);
    // a.add_edge(4,6,1,true);
    // a.add_edge(5,6,9,true);
    // a.remove_edge(1,2,true);
    a.add_edge(0,1,5,true);
    a.add_edge(0,2,3,true);
    a.add_edge(1,3,6,true);
    a.add_edge(1,2,2,true);
    a.add_edge(2,3,7,true);
    a.add_edge(2,4,4,true);
    a.add_edge(2,9,2,true);
    a.add_edge(3,4,-1,true);
    a.add_edge(4,9,-2,true);
    println!("{}",a);
    a.shortest_paths(3);
    println!("{}",std::i32::MIN);
    // println!("{:#?}",b);
}
*/