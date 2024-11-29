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

```python
from byte_trie import PatriciaTrie, AdaptiveRadixTrie

pt = PatriciaTrie()
pt.insert(b"hello", 1)
pt.insert(b"world", 2)

art = AdaptiveRadixTrie()
art.insert(b"hello", 1)
art.insert(b"world", 2)

```
