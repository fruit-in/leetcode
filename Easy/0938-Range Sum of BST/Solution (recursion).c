/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


int rangeSumBST(struct TreeNode* root, int L, int R){
    int sum = root -> val;
    if(sum < L || sum > R)
        sum = 0;
    if(root -> left != NULL)
        sum += rangeSumBST(root -> left, L, R);
    if(root -> right != NULL)
        sum += rangeSumBST(root -> right, L, R);
    return sum;
}
