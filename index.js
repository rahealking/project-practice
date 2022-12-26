var prompt=require("prompt-sync")();
class queue{
    constructor(size,k){
        this.arr=[];
        this.front=[];
        this.rare=[];
        while(k>0){
            this.front.push(-1);
            this.rare.push(-1);
            k-=1;
        }while(size>0){
            this.arr.push(0);
            size-=1;
        }
    }
    enqueue(k,value){
        if(this.rare[this.rare.length-1]<this.arr.length-1){
            var i=this.rare.length-1;
            var bool=false;
            var j=0;
            while(i>k-1){
                if(i==0){
                    if(this.front[i]>-1&&this.rare[i]>this.front[i]-1){
                        bool=true;
                    }
                }else{
                    if(i>0){
                        if(this.front[i]>this.front[i-1]&&this.rare[i]>this.front[i]-1){
                            bool=true;
                        }
                    }
                }
                if(bool==true){
                    j=this.rare[i];
                    while(j>this.front[i]-1){
                        this.arr[j+1]=this.arr[j];
                        j-=1;
                    }
                    this.front[i]+=1;
                    this.rare[i]+=1;
                }
                i-=1;
            }
            this.arr[--this.front[k]]=value;
            return value;
        }else{
            console.error("main queue overflowed;");
            return undefined;
        }
    }
    dequeue(k){
        var bool=false;
        if(k==0){
            if(this.front[k]>-1&&this.rare[k]>=this.front[k]){
                bool=true;
            }
        }else{
            if(k>0){
                if(this.front[k]>this.front[k-1]&&this.rare[k]>=this.front[k]){
                    bool=true;
                }
            }
        }
        if(bool==true){
            var value=this.arr[this.front[k]];
            var i=0;
            var j=0;
            i=k;
            while(i<this.front.length){
                j=this.front[i];
                while(j<this.rare[i]+1){
                    this.arr[j]=this.arr[j+1];
                    j+=1;
                }
                this.front[i]-=1;
                this.rare[i]-=1;
                i+=1;
            }
            this.front[k]+=1;
            return value;
        }else{
            console.log("queue overflowed;");
            return undefined;
        }
    }
    static debug(){
        console.log(["debugging"]);
        var command="";
        var i=0;
        i=Number.parseInt(prompt("[size]$ "));
        var st=new queue(i,Number.parseInt(prompt("[k]$ ")));
        while(true){
            command=prompt("[command]$ ");
            if(command=="exit"){
                console.log("process finished;");
                return;
            }else if(command=="info"){
                console.log(["queue information"]);
                process.stdout.write("[array]:");
                console.log(st.arr);
                process.stdout.write("[front]:");
                console.log(st.front);
                process.stdout.write("[rare]:");
                console.log(st.rare);
                process.stdout.write("[size]:");
                console.log([st.arr.length]);
            }else if(command=="enqueue"){
                st.enqueue(Number.parseInt(prompt("[k]$ ")),Number.parseInt(prompt("[value]$ ")));
                console.log(["done"]);
            }else if(command=="dequeue"){
                process.stdout.write("dequeued ");
                console.log([st.dequeue(Number.parseInt(prompt("[k]$ ")))]);
            }else{
                process.stdout.write("unknown command ");
                console.log([command]);
            }
        }
    }
}
function main(){
    queue.debug();
    return;
}
main();