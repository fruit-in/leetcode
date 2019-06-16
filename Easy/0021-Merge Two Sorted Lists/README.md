# 21. Merge Two Sorted Lists
Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.

#### Example 1:
<pre>
<strong>Input:</strong> 1->2->4, 1->3->4
<strong>Output:</strong> 1->1->2->3->4->4
</pre>

## Solutions

### 1. Iteration (C)
```C
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
```

### 2. Recursion (C)
```C
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* mergeTwoLists(struct ListNode* l1, struct ListNode* l2){
    if(l1 == NULL)
        return l2;
    if(l2 == NULL)
        return l1;
    if(l1 -> val < l2 -> val){
        l1 -> next = mergeTwoLists(l1 -> next, l2);
        return l1;
    }else{
        l2 -> next = mergeTwoLists(l1, l2 -> next);
        return l2;
    }
}
```
