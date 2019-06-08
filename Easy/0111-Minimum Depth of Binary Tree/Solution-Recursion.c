/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

int minDepth(struct TreeNode* root){
    int lmin = 0, rmin = 0;
    if(root == NULL)
        return 0;
    if(root -> left == NULL && root -> right == NULL)
        return 1;
    if(root -> left != NULL)
        lmin = minDepth(root -> left) + 1;
    if(root -> right != NULL)
        rmin = minDepth(root -> right) + 1;
    if(lmin != 0 && rmin != 0)
        return lmin > rmin ? rmin : lmin;
    else
        return lmin == 0 ? rmin : lmin;
}
