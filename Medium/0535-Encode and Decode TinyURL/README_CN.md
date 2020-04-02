# 535. TinyURL 的加密与解密
TinyURL是一种URL简化服务， 比如：当你输入一个URL ```https://leetcode.com/problems/design-tinyurl``` 时，它将返回一个简化的URL ```http://tinyurl.com/4e9iAk```.

要求：设计一个 TinyURL 的加密 ```encode``` 和解密 ```decode``` 的方法。你的加密和解密算法如何设计和运作是没有限制的，你只需要保证一个URL可以被加密成一个TinyURL，并且这个TinyURL可以用解密方法恢复成原本的URL。

## 题解 (Python)

### 1. 题解
```Python
class Codec:
    short_long = {}

    def encode(self, longUrl: str) -> str:
        """Encodes a URL to a shortened URL.
        """
        x = hash(longUrl)
        shortUrl = "p" if x > 0 else "n"
        x = abs(x)

        while x:
            shortUrl += chr(x % 95 + 32)
            x //= 95

        self.short_long[shortUrl] = longUrl

        return shortUrl


    def decode(self, shortUrl: str) -> str:
        """Decodes a shortened URL to its original URL.
        """
        return self.short_long[shortUrl]


# Your Codec object will be instantiated and called as such:
# codec = Codec()
# codec.decode(codec.encode(url))
```
