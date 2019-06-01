/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

typedef struct StackNode{
    struct TreeNode* t1;
    struct TreeNode* t2;
    struct StackNode* next;
}StackNode;

struct TreeNode* mergeTrees(struct TreeNode* t1, struct TreeNode* t2){
    if(t1 == NULL)
        return t2;
    if(t2 == NULL)
        return t1;
    StackNode* top, * push, * pop;
    top = (StackNode*)malloc(sizeof(StackNode));
    top -> next = NULL;
    push = (StackNode*)malloc(sizeof(StackNode));
    push -> t1 = t1;
    push -> t2 = t2;
    push -> next = top;
    top = push;
    while(top -> next != NULL){
        pop = top;
        top = top -> next;
        pop -> t1 -> val += pop -> t2 -> val;
        if(pop -> t1 -> left != NULL && pop -> t2 -> left != NULL){
            push = (StackNode*)malloc(sizeof(StackNode));
            push -> t1 = pop -> t1 -> left;
            push -> t2 = pop -> t2 -> left;
            push -> next = top;
            top = push;
        }else if(pop -> t1 -> left == NULL && pop -> t2 -> left != NULL)
            pop -> t1 -> left = pop -> t2 -> left;
        if(pop -> t1 -> right != NULL && pop -> t2 -> right != NULL){
            push = (StackNode*)malloc(sizeof(StackNode));
            push -> t1 = pop -> t1 -> right;
            push -> t2 = pop -> t2 -> right;
            push -> next = top;
            top = push;
        }else if(pop -> t1 -> right == NULL && pop -> t2 -> right != NULL)
            pop -> t1 -> right = pop -> t2 -> right;
        free(pop);
    }
    free(top);
    return t1;
}
