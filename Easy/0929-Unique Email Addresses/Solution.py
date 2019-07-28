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
