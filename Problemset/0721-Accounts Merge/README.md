# 721. Accounts Merge
Given a list of `accounts` where each element `accounts[i]` is a list of strings, where the first element `accounts[i][0]` is a name, and the rest of the elements are emails representing **emails** of the account.

Now, we would like to merge these accounts. Two accounts definitely belong to the same person if there is some common email to both accounts. Note that even if two accounts have the same name, they may belong to different people as people could have the same name. A person can have any number of accounts initially, but all of their accounts definitely have the same name.

After merging the accounts, return the accounts in the following format: the first element of each account is the name, and the rest of the elements are emails **in sorted order**. The accounts themselves can be returned in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
<strong>Output:</strong>
[["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
<strong>Explanation:</strong>
The first and second John's are the same person as they have the common email "johnsmith@mail.com".
The third John and Mary are different people as none of their email addresses are used by other accounts.
We could return these lists in any order, for example the answer [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'],
['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
<strong>Output:</strong>
[["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
</pre>

#### Constraints:
* `1 <= accounts.length <= 1000`
* `2 <= accounts[i].length <= 10`
* `1 <= accounts[i][j].length <= 30`
* `accounts[i][0]` consists of English letters.
* `accounts[i][j] (for j > 0)` is a valid email.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        def merge(name: str, emailslists: List[List[str]]) -> List[List[str]]:
            parent = {}
            groups = {}
            ret = []

            for emails in emailslists:
                minemail = emails[0]

                for email in emails:
                    if email not in parent:
                        parent[email] = email
                    while parent[email] != parent[parent[email]]:
                        parent[email] = parent[parent[email]]
                    minemail = min(minemail, parent[email])

                for email in emails:
                    parent[parent[email]] = minemail
                    parent[email] = minemail

            for email in parent:
                while parent[email] != parent[parent[email]]:
                    parent[email] = parent[parent[email]]
                if parent[email] not in groups:
                    groups[parent[email]] = []
                groups[parent[email]].append(email)

            for group in groups.values():
                ret.append([name] + sorted(group))

            return ret

        groups = {}
        ret = []

        for account in accounts:
            if account[0] not in groups:
                groups[account[0]] = []
            groups[account[0]].append(account[1:])

        for name, emailslists in groups.items():
            ret += merge(name, emailslists)

        return ret
```
