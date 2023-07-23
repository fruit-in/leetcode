# 284. Peeking Iterator
Given an Iterator class interface with methods: `next()` and `hasNext()`, design and implement a PeekingIterator that support the `peek()` operation -- it essentially peek() at the element that will be returned by the next call to next().

#### Example:
<pre>
Assume that the iterator is initialized to the beginning of the list: <strong>[1,2,3]</strong>.

Call <strong>next()</strong> gets you <strong>1</strong>, the first element in the list.
Now you call <strong>peek()</strong> and it returns <strong>2</strong>, the next element. Calling <strong>next()</strong> after that <i><strong>still</strong></i> return <strong>2</strong>.
You call <strong>next()</strong> the final time and it returns <strong>3</strong>, the last element.
Calling <strong>hasNext()</strong> after that should return <strong>false</strong>.
</pre>

**Follow up:** How would you extend your design to be generic and work with all types, not just integer?

## Solutions (Python)

### 1. Solution
```Python
# Below is the interface for Iterator, which is already defined for you.
#
# class Iterator:
#     def __init__(self, nums):
#         """
#         Initializes an iterator object to the beginning of a list.
#         :type nums: List[int]
#         """
#
#     def hasNext(self):
#         """
#         Returns true if the iteration has more elements.
#         :rtype: bool
#         """
#
#     def next(self):
#         """
#         Returns the next element in the iteration.
#         :rtype: int
#         """

class PeekingIterator:
    def __init__(self, iterator):
        """
        Initialize your data structure here.
        :type iterator: Iterator
        """
        self.iter = iterator
        self.cache = None

    def peek(self):
        """
        Returns the next element in the iteration without advancing the iterator.
        :rtype: int
        """
        if self.cache is None:
            self.cache = self.iter.next()
        return self.cache

    def next(self):
        """
        :rtype: int
        """
        if self.cache is not None:
            temp = self.cache
            self.cache = None
            return temp
        return self.iter.next()

    def hasNext(self):
        """
        :rtype: bool
        """
        return self.cache is not None or self.iter.hasNext()

# Your PeekingIterator object will be instantiated and called as such:
# iter = PeekingIterator(Iterator(nums))
# while iter.hasNext():
#     val = iter.peek()   # Get the next element but not advance the iterator.
#     iter.next()         # Should return the same value as [val].
```
