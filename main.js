const prompt=require("prompt-sync")();
class node{
    constructor(value){
        this.value=value;
        this.next=null;
    }
    static listMid(head){
        var slow=head;
        var fast=head;
        while(fast!=null){
            slow=slow.next;
            if(fast.next!=null){
                fast=fast.next.next;
            }else{
                break;
            }
        }
        if(slow!=null){
            return slow;
        }else{
            return head;
        }
    }
}
class queue{
    constructor(){
        this.front=null;
        this.rare=null;
    }
    toList(){
        var arr=[];
        var temp=this.front;
        while(temp!=null){
            arr.push((temp.value).toString());
            temp=temp.next;
        }
        return arr;
    }
    listSize(){
        var size=0;
        var temp=this.front;
        while(temp!=null){
            temp=temp.next;
            size+=1;
        }
        return size;
    }
    enqueue(value){
        if(this.front==null){
            this.front=this.rare=new node(value);
            return;
        }else{
            this.rare.next=new node(value);
            this.rare=this.rare.next;
            return;
        }
    }
    dequeue(){
        if(this.front==null){
            return undefined;
        }else{
            var value=this.front.value;
            this.front=this.front.next;
            if(this.front==null){
                this.rare=null;
            }
            return value;
        }
    }
    peek(position){
        var temp=this.front;
        while(position>0&&temp!=null){
            temp=temp.next;
        }
        if(temp==null){
            return undefined;
        }else{
            return temp.value;
        }
    }
    reverse(){
        if(this.front!=null){
            var value=this.dequeue();
            this.reverse();
            this.enqueue(value);
        }
        return;
    }
    static debug(){
        console.log("[debugging]");
        var q=new queue();
        var i=0;
        var command="";
        while(true){
            command=prompt("[command]$ ");
            if(command=="exit"){
                console.log(`process finished with command [${command}]`);
                return;
            }else if(command=="info"){
                console.log(["queue information"]);
                process.stdout.write("[list]:");
                console.log(q.toList());
                process.stdout.write("[size]:");
                console.log(q.listSize());
                process.stdout.write("[front]:");
                if(q.front==null){
                    console.log(undefined);
                }else{
                    i=q.front.value;
                    console.log(i);
                }
                process.stdout.write("[rare]:");
                if(q.front==null){
                    console.log(undefined);
                }else{
                    i=q.rare.value;
                    console.log(i);
                }
                process.stdout.write("[empty]:");
                console.log(q.front==null?true:false);
                process.stdout.write("[full]:");
                console.log(false)
                process.stdout.write("[limit]:");
                console.log(["no limits"]);
            }else if(command=="enqueue"){
                i=prompt("[value]$ ");
                i=Number(i);
                q.enqueue(i);
                process.stdout.write("[enqueued]:");
                console.log(i);
            }else if(command=="dequeue"){
                i=q.dequeue();
                process.stdout.write("[dequeued]:");
                console.log(i);
            }else if(command=="reverse"){
                q.reverse();
                console.log("[information]:[queue reversed]");
            }else if(command=="reverseK"){
                i=prompt("[k]$ ");
                i=Number(i);
                solves.reverseFirstKElements(q,i);
                console.log("[information]:[queue reversed in k groups]");
            }else if(command=="interleave"){
                solves.interleave(q);
                console.log("[information]:[queue interleaved]");
            }else{
                console.log("[unknown command]:["+command+"]");
            }
        }
    }
}
class stack{
    constructor(){
        this.top=null;
    }
    toList(){
        var arr=[];
        var temp=this.top;
        while(temp!=null){
            arr.push(temp.value.toString());
            temp=temp.next;
        }
        return arr;
    }
    size(){
        return this.toList().length;
    }
    push(value){
        var newNode=new node(value);
        newNode.next=this.top;
        this.top=newNode;
    }
    pop(){
        var value=undefined;
        if(this.top!=null){
            value=this.top.value;
            this.top=this.top.next;
        }
        return value;
    }
    peek(position){
        var temp=this.top;
        while(temp!=null&&position>0){
            temp=temp.next;
        }
        if(temp!=null){
            temp=temp.value;
        }else{
            temp=undefined;
        }
        return temp;
    }
    reverse(){
        var value=this.pop();
        if(value!=undefined){
            this.reverse();
            this.push(value);
        }
        return;
    }
    static debug(){
        console.log(["debugging"]);
        var st=new stack();
        var command="";
        var i=0;
        while(true){
            command=prompt("[command]:$ ");
            if(command=="exit"){
                process.stdout.write("process finished with command ");
                console.log(["exit"]);
                return;
            }else if(command=="info"){
                console.log("[stack information]");
                process.stdout.write("[list]:");
                console.log(st.toList());
                process.stdout.write("[size]:");
                console.log([st.size()]);
                process.stdout.write("[top]:");
                console.log([st.top!=null?st.top.value:undefined]);
                process.stdout.write("[empty]:");
                console.log([st.top==null?true:false]);
                process.stdout.write("[full]:");
                console.log([false]);
                process.stdout.write("[limit]:");
                console.log(["no limits"]);
            }else if(command=="push"){
                i=Number(prompt("[value]$ "));
                st.push(i);
                process.stdout.write("[pushed]:");
                console.log([i]);
            }else if(command=="pop"){
                process.stdout.write("[popped]:");
                console.log([st.pop()]);
            }else if(command=="peek"){
                i=prompt("[position]$ ");
                process.stdout.write("[peeked element]:");
                console.log([st.peek(i)]);
            }else if(command=="reverse"){
                st.reverse();
                console.log("[information]:[stack reversed]");
            }else{
                process.stdout.write("[unknown command]:");
                console.log([command]);
            }
        }
    }
}
class solves{
    static reverseK(q,round){
        if(round>0&&q.front!=null){
            var value=q.dequeue();
            solves.reverseK(q,round-1);
            q.enqueue(value);
        }
        return;
    }static reverseFirstKElements(q,k){
        if(q.front!=null){
            solves.reverseK(q,k);
            for(k=q.listSize()-k;k>0;k-=1){
                q.enqueue(q.dequeue());
            }
        }
        return;
    }
    static interleave(q){
        if(q.front!=null){
            var i=q.listSize();
            i=Number.parseInt(i/2);
            var st=new stack();
            while(i>0){
                st.push(q.dequeue());
                i-=1;
            }while(st.top!=null){
                q.enqueue(st.pop());
            }
            i=q.listSize();
            i-=Number.parseInt(i/2);
            while(i>0){
                q.enqueue(q.dequeue());
                i-=1;
            }
            i=q.listSize();
            i=Number.parseInt(i/2);
            while(i>0){
                st.push(q.dequeue());
                i-=1;
            }while(st.top!=null){
                q.enqueue(st.pop());
                if(st.size()==0){
                    if(Number.parseInt(((q.listSize()+st.size()))/2)%2!=0){
                        break;
                    }
                }
                q.enqueue(q.dequeue());
            }
        }
        return;
    }
}
function main(){
    // queue.debug();
    return;
}
main();