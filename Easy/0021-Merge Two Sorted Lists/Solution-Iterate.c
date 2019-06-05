/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* mergeTwoLists(struct ListNode* l1, struct ListNode* l2){
    struct ListNode* head = (struct ListNode*)malloc(sizeof(struct ListNode));
    struct ListNode* p = head;
    while(l1 != NULL && l2 != NULL)
        if(l1 -> val < l2 -> val){
            p -> next = l1;
            p = p -> next;
            l1 = l1 -> next;
        }else{
            p -> next = l2;
            p = p -> next;
            l2 = l2 -> next;
        }
    p -> next = l1 == NULL ? l2 : l1;
    return head -> next;
}
