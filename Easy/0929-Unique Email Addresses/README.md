# 929. Unique Email Addresses
Every email consists of a local name and a domain name, separated by the @ sign.

For example, in <code>alice@leetcode.com</code>, <code>alice</code> is the local name, and <code>leetcode.com</code> is the domain name.

Besides lowercase letters, these emails may contain <code>'.'</code>s or <code>'+'</code>s.

If you add periods (<code>'.'</code>) between some characters in the **local name** part of an email address, mail sent there will be forwarded to the same address without dots in the local name.  For example, <code>"alice.z@leetcode.com"</code> and <code>"alicez@leetcode.com"</code> forward to the same email address.  (Note that this rule does not apply for domain names.)

If you add a plus (<code>'+'</code>) in the **local name**, everything after the first plus sign will be **ignored**. This allows certain emails to be filtered, for example <code>m.y+name@email.com</code> will be forwarded to <code>my@email.com</code>.  (Again, this rule does not apply for domain names.)

It is possible to use both of these rules at the same time.

Given a list of <code>emails</code>, we send one email to each address in the list.  How many different addresses actually receive mails?

#### Example 1:
<pre>
<strong>Input:</strong>
["test.email+alex@leetcode.com","test.e.mail+bob.cathy@leetcode.com","testemail+david@lee.tcode.com"]
<strong>Output:</strong>
"testemail@leetcode.com" and "testemail@lee.tcode.com" actually receive mails
</pre>

#### Note:
* <code>1 <= emails[i].length <= 100</code>
* <code>1 <= emails.length <= 100</code>
* Each <code>emails[i]</code> contains exactly one <code>'@'</code> character.
* All local and domain names are non-empty.
* Local names do not start with a <code>'+'</code> character.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def numUniqueEmails(self, emails: List[str]) -> int:
        result = set()
        for email in emails:
            localname, domainname = email.split('@')
            localname = localname.replace('.', '')
            if '+' in localname:
                localname = localname[:localname.find('+')]
            result.add('@'.join([localname, domainname]))
        return len(result)
```
