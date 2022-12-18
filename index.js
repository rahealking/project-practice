var prompt=require("prompt-sync")();
class st{
    constructor(){
        this.arr=[];
    }
    append(value){
        this.arr.push(value);
        return value;
    }
    out(){
        return this.arr.pop();
    }
}
class queue{
    constructor(){
        this.input=new st();
        this.output=new st();
    }
    enqueue(value){
        return this.input.append(value);
    }
    dequeue(){
        var value=this.output.out();
        if(value==undefined){
            value=this.input.out();
            while(value!=undefined){
                this.output.append(value);
                value=this.input.out();
            }
            value=this.output.out();
        }
        return value;
    }
}
class stack{
    constructor(){
        this.primaryQueue=new queue();
        this.secondaryQueue=new queue();
    }
    append(value){
        this.secondaryQueue.enqueue(value);
        var value=this.primaryQueue.dequeue();
        while(value!=undefined){
            this.secondaryQueue.enqueue(value);
            var value=this.primaryQueue.dequeue();
        }
        value=this.secondaryQueue;
        this.secondaryQueue=this.primaryQueue;
        this.primaryQueue=value;
    }
    out(){
        return this.primaryQueue.dequeue();
    }
}
function main(){
    var stk=new stack();
    stk.append(Number(prompt(":")));
    stk.append(Number(prompt(":")));
    stk.append(Number(prompt(":")));
    stk.append(Number(prompt(":")));
    console.log(stk.out());
    console.log(stk.out());
    console.log(stk.out());
    console.log(stk.out());
    console.log(stk.out());
    stk.append(Number(prompt(":")));
    console.log(stk.out());
    return;
}
main();