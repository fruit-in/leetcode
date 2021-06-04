# @param {String} s
# @return {String}
def mask_pii(s)
  s.include?('@') ? mask_email(s) : mask_phone(s)
end

# @param {String} s
# @return {String}
def mask_email(s)
  first, remain = s.downcase.split('@')

  first[0] + '*****' + first[-1] + '@' + remain
end

# @param {String} s
# @return {String}
def mask_phone(s)
  s.delete!('+( )-')

  s.size == 10 ? '***-***-' + s[-4..] : '+' + '*' * (s.size - 10) + '-***-***-' + s[-4..]
end
