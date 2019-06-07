/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

bool isMirror(struct TreeNode* root, struct TreeNode* toor){
    if(root == NULL && toor == NULL)
        return true;
    if(root == NULL || toor == NULL || root -> val != toor -> val)
        return false;
    return isMirror(root -> left, toor -> right) && isMirror(root -> right, toor -> left);
}

bool isSymmetric(struct TreeNode* root){
     return isMirror(root, root);
}
