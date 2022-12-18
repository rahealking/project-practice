#include<stdio.h>
#include<stdlib.h>
#include<string.h>
struct node{
    int value;
    struct node*next;
};
struct node*createNode(int value){
    struct node*newNode=(struct node*)malloc(sizeof(struct node));
    newNode->value=value;
    newNode->next=NULL;
    return newNode;
}
void printList(struct node*temp){
    if(temp!=NULL){
        printf("%d%c",temp->value,temp->next==NULL?'\0':',');
        printList(temp->next);
    }
    return;
}
int listSize(struct node*temp){
    int size=0;
    for(size=0;temp!=NULL;temp=temp->next){
        size+=1;
    }
    return size;
}
struct queue{
    struct node*front;
    struct node*rare;
};
struct queue*createQueue(){
    struct queue*q=(struct queue*)malloc(sizeof(struct queue));
    q->front=NULL;
    q->rare=NULL;
    return q;
}
void enqueue(struct queue*q,int value){
    if(q->front==NULL){
        q->front=q->rare=createNode(value);
    }else{
        q->rare->next=createNode(value);
        q->rare=q->rare->next;
    }
    return;
}
int dequeue(struct queue*q){
    if(q->front==NULL){
        printf("queue underflowed;\n");
        return-1;
    }else{
        int value=q->front->value;
        struct node*temp=q->front;
        q->front=q->front->next;
        if(q->front==NULL){
            q->rare=NULL;
        }
        free(temp);
        return value;
    }
}
int peek(struct queue*q,int position){
    struct node*temp=NULL;
    for(temp=q->front;temp!=NULL&&position>0;temp=temp->next){
        position-=1;
    }
    if(temp==NULL){
        printf("list index out of bounds;\n");
        return-1;
    }else{
        return temp->value;
    }
}
void reverseQueue(struct queue*q){
    if(q->front!=NULL){
        int value=dequeue(q);
        reverseQueue(q);
        enqueue(q,value);
    }
    return;
}
void reverseK(struct queue*q,int round){
    if(round>0&&q->front!=NULL){
        int value=dequeue(q);
        reverseK(q,round-1);
        enqueue(q,value);
    }
    return;
}
void reverseKElements(struct queue*q,int k){
    reverseK(q,k);
    int size=listSize(q->front);
    for(size-=k;size>0&&q->front!=NULL;size-=1){
        enqueue(q,dequeue(q));
    }
    return;
}
void debug(){
    printf("[debugging]\n");
    struct queue*q=createQueue();
    int i=0;
    char command[10];
    struct node*tempNode;
    while(1){
        printf("[command]$ ");
        scanf("%s",command);
        if(strcmp(command,"exit\0")==0){
            printf("process finished with command [exit]\n");
            return;
        }else if(strcmp(command,"info\0")==0){
            printf("[info]\n");
            printf("[list]:[");
            printList(q->front);
            printf("]\n[list size]:[%d]\n",listSize(q->front));
            printf("[front]:[%d]\n[rare]:[%d]\n",q->front==NULL?-1:q->front->value,q->rare==NULL?-1:q->rare->value);
            printf("[empty]:[%s]\n",q->front==NULL?"true":"false");
        }else if(strcmp(command,"enqueue\0")==0){
            printf("[value]$ ");
            scanf("%d",&i);
            enqueue(q,i);
            printf("enqueued [%d]\n",i);
        }else if(strcmp(command,"dequeue\0")==0){
            i=dequeue(q);
            printf("dequeued [%d]\n",i);
        }else if(strcmp(command,"peek\0")==0){
            printf("[position]$ ");
            scanf("%d",&i);
            i=peek(q,i);
            printf("peeked [%d]\n",i);
        }else if(strcmp(command,"reverse\0")==0){
            reverseQueue(q);
            printf("message [queue reversed]\n");
        }else if(strcmp(command,"reverseK\0")==0){
            printf("[k]$ ");
            scanf("%d",&i);
            reverseKElements(q,i);
            printf("first [%d] elements reversed;\n",i);
        }else{
            printf("unknown command [%s]\n",command);
        }
    }
}
int main(){
    printf("process started;\n");
    debug();
    return 0;
}