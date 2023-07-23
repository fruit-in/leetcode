# @param {String[]} emails
# @return {Integer}
def num_unique_emails(emails)
    ret = Set.new

    for email in emails
        localname, domainname = email.split('@')
        localname.gsub!('.', '')
        if localname.include?('+')
            localname = localname[0...(localname.index('+'))]
        end
        ret.add([localname, domainname].join('@'))
    end

    return ret.length
end
