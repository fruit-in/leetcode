/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


bool isUnivalTree(struct TreeNode* root){
    int i = 1;
    struct TreeNode* nodes[100];
    nodes[0] = root;
    for(int j = 0; j < i; j++){
        if(nodes[j] -> val != root -> val)
            return false;
        if(nodes[j] -> left != NULL){
            nodes[i] = nodes[j] -> left;
            i++;
        }
        if(nodes[j] -> right != NULL){
            nodes[i] = nodes[j] -> right;
            i++;
        }
    }
    return true;
}
