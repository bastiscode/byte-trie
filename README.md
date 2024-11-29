## Byte tries in Rust with Python bindings

### Installation

Install from PyPI (only built for Python 3.10+ on Linux)

```
pip install byte-trie
```

### Manual installation

Make sure you have Rust installed, then do the following:

```bash
git clone https://github.com/bastiscode/byte-trie.git
cd byte-trie
pip install maturin[patchelf]
maturin develop --release
```

### Usage

Currently implemented tries:
- Patricia trie
- Adaptive radix trie

Key needs to be a bytes object, value can be anything.
Make sure that the key never contains a null/zero byte, as it is used as a terminator internally. For utf8-encoded text this usually is the case, but for other types of data you might need to encode it in a way that ensures this.

```python
from byte_trie import PatriciaTrie, AdaptiveRadixTrie

pt = PatriciaTrie()

# add key-value pairs
pt.insert(b"hello", 1)
pt.insert(b"world", 2)

# delete key
print(pt.delete(b"hello")) # 1

# check for key with prefix
print(pt.contains(b"hel")) # False
print(pt.contains(b"wor")) # True

# get values
print(pt.get(b"hello")) # None
print(pt.get(b"world")) # 2

# overwrite
print(pt.insert(b"world", 3)) # 2
print(pt.get(b"world")) # 3

# continuations for prefix
print(pt.continuations(b"wo")) # [(b'world', 3)]

# same for ART
art = AdaptiveRadixTrie()

art.insert(b"hello", 1)
art.insert(b"world", 2)

print(art.delete(b"hello")) # 1

print(art.contains(b"hel")) # False
print(art.contains(b"wor")) # True

print(art.get(b"hello")) # None
print(art.get(b"world")) # 2

print(art.insert(b"world", 3)) # 2
print(art.get(b"world")) # 3

print(art.continuations(b"wo")) # [(b'world', 3)]
```
