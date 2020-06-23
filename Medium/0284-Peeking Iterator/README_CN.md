# 284. 顶端迭代器
给定一个迭代器类的接口，接口包含两个方法： `next()` 和 `hasNext()`。设计并实现一个支持 `peek()` 操作的顶端迭代器 -- 其本质就是把原本应由 `next()` 方法返回的元素 `peek()` 出来。

#### 示例:
<pre>
假设迭代器被初始化为列表 <strong>[1,2,3]</strong>。

调用 <strong>next()</strong> 返回 <strong>1</strong>，得到列表中的第一个元素。
现在调用 <strong>peek()</strong> 返回 <strong>2</strong>，下一个元素。在此之后调用 <strong>next()</strong> 仍然返回 <strong>2</strong>。
最后一次调用 <strong>next()</strong> 返回 <strong>3</strong>，末尾元素。在此之后调用 <strong>hasNext()</strong> 应该返回 <strong>false</strong>。
</pre>

**进阶:** 你将如何拓展你的设计？使之变得通用化，从而适应所有的类型，而不只是整数型？

## 题解 (Python)

### 1. 题解
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
