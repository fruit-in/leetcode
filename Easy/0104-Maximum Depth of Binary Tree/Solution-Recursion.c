/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


int maxDepth(struct TreeNode* root){
    if(root == NULL)
        return 0;
    int lmax = maxDepth(root -> left);
    int rmax = maxDepth(root -> right);
    return (lmax > rmax ? lmax : rmax) + 1;
}
