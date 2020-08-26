# 1507. Reformat Date
Given a `date` string in the form `Day Month Year`, where:
* `Day` is in the set `{"1st", "2nd", "3rd", "4th", ..., "30th", "31st"}`.
* `Month` is in the set `{"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}`.
* `Year` is in the range `[1900, 2100]`.

Convert the date string to the format `YYYY-MM-DD`, where:
* `YYYY` denotes the 4 digit year.
* `MM` denotes the 2 digit month.
* `DD` denotes the 2 digit day.

#### Example 1:
<pre>
<strong>Input:</strong> date = "20th Oct 2052"
<strong>Output:</strong> "2052-10-20"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> date = "6th Jun 1933"
<strong>Output:</strong> "1933-06-06"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> date = "26th May 1960"
<strong>Output:</strong> "1960-05-26"
</pre>

#### Constraints:
* The given dates are guaranteed to be valid, so no error handling is necessary.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def reformatDate(self, date: str) -> str:
        day, month, year = date.split()
        day = int(day[:-2])
        month = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].index(month) + 1

        return "%s-%02d-%02d" % (year, month, day)
```
