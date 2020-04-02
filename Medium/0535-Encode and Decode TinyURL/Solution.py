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
