/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


bool isUnivalTree(struct TreeNode* root){
    return (root -> left == NULL
           || (root -> val == root -> left -> val && isUnivalTree(root -> left)))
        && (root -> right == NULL
           || (root -> val == root -> right -> val && isUnivalTree(root -> right)));
}
