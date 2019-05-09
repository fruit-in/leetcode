/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */


int rangeSumBST(struct TreeNode* root, int L, int R){
    int sum;
    int i;
    struct TreeNode* tn = (struct TreeNode*)malloc(sizeof(struct TreeNode) * 10000);
    struct TreeNode node;
    for(sum = 0, i = 0, tn[0] = *root; i >= 0; i--) {
        node = tn[i];
        if(node.val >= L && node.val <= R)
            sum += node.val;
        if(node.left != NULL) {
            tn[i] = *(node.left);
            i++;
        }
        if(node.right != NULL) {
            tn[i] = *(node.right);
            i++;
        }
    }
    free(tn);
    return sum;
}
