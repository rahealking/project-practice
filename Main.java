class node{
    int row;
    int colum;
    node next;
    public node(int row,int colum){
        this.row=row;
        this.colum=colum;
        this.next=null;
    }
}
class queue{
    node front;
    node rare;
    public queue(){
        front=null;
        rare=null;
    }
    public boolean enqueue(int row,int colum){
        node newNode=new node(row,colum);
        if(this.front==null){
            this.front=this.rare=newNode;
            return true;
        }else{
            this.rare.next=newNode;
            this.rare=this.rare.next;
            return true;
        }
    }
    public node dequeue(){
        try{
            return this.front;
        }finally{
            if(this.front!=null){
                this.front=this.front.next;
                if(this.front==null){
                    this.rare=null;
                }
            }
        }
    }
    public boolean empty(){
        if(this.front==null){
            return true;
        }else{
            return false;
        }
    }
}
class solves{
    public static int rottenOranges(int[][]basket){
        int time=0;
        queue primaryQueue=new queue();
        queue secondaryQueue=new queue();
        queue tempQueue=null;
        int i=0,j=0;
        node value;
        for(i=0;i<basket.length;i+=1){
            for(j=0;j<basket[0].length;j+=1){
                if(basket[i][j]==2){
                    primaryQueue.enqueue(i,j);
                }
            }
        }
        while(!primaryQueue.empty()){
            while(!primaryQueue.empty()){
                value=primaryQueue.dequeue();
                value.next=null;
                if(value.row>0&&value.row<basket.length){
                    if(basket[value.row-1][value.colum]==1){
                        basket[value.row-1][value.colum]=2;
                        secondaryQueue.enqueue(value.row-1,value.colum);
                    }
                }if(value.colum<basket[0].length-1&&value.colum>-1){
                    if(basket[value.row][value.colum+1]==1){
                        basket[value.row][value.colum+1]=2;
                        secondaryQueue.enqueue(value.row,value.colum+1);
                    }
                }if(value.row<basket.length-1&&value.row>-1){
                    if(basket[value.row+1][value.colum]==1){
                        basket[value.row+1][value.colum]=2;
                        secondaryQueue.enqueue(value.row+1,value.colum);
                    }
                }if(value.colum>0&&value.colum<basket[0].length){
                    if(basket[value.row][value.colum-1]==1){
                        basket[value.row][value.colum-1]=2;
                        secondaryQueue.enqueue(value.row,value.colum-1);
                    }
                }
            }
            if(secondaryQueue.empty()){
                return-1;
            }else{
                time++;
                tempQueue=primaryQueue;
                primaryQueue=secondaryQueue;
                secondaryQueue=tempQueue;
            }
        }
        if(time>0){
            return time;
        }else{
            return-1;
        }
    }
}
public class Main{
    public static void main(String[]args){
        int[][]arr={
            {1,1,1},
            {1,1,1},
            {1,1,2}
        };
        System.out.println(solves.rottenOranges(arr));
        System.out.println("working");
    }
}