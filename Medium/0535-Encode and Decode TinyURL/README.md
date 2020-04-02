# 535. Encode and Decode TinyURL
> Note: This is a companion problem to the [System Design](https://leetcode.com/discuss/interview-question/system-design/) problem: [Design TinyURL](https://leetcode.com/discuss/interview-question/124658/Design-a-URL-Shortener-(-TinyURL-)-System/).

TinyURL is a URL shortening service where you enter a URL such as ```https://leetcode.com/problems/design-tinyurl``` and it returns a short URL such as ```http://tinyurl.com/4e9iAk```.

Design the ```encode``` and ```decode``` methods for the TinyURL service. There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.

## Solutions (Python)

### 1. Solution
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
