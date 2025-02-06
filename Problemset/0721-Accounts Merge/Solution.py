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
