/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


bool isUnivalTree(struct TreeNode* root){
    int top = 0;
    struct TreeNode stack[100];
    struct TreeNode tn;
    stack[0] = *root;
    while(top >= 0){
        tn = stack[top];
        top--;
        if(tn.left != NULL)
            if(tn.left -> val != root -> val)
                return false;
            else{
                top++;
                stack[top] = *(tn.left);
            }
        if(tn.right != NULL)
            if(tn.right -> val != root -> val)
                return false;
            else{
                top++;
                stack[top] = *(tn.right);
            }
    }
    return true;
}
