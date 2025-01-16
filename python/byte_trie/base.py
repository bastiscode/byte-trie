from typing import Any


class ByteTrie:
    """

    Base class for byte tries.

    """

    def __len__(self) -> int:
        """

        Get the number of key-value pairs in the trie.

        """

    def insert(self, key: bytes, value: Any) -> Any | None:
        """

        Insert a key-value pair into the trie and return the previous value if it exists.

        """

    def delete(self, key: bytes) -> Any | None:
        """

        Delete a key from the trie and return its value if it exists.

        """

    def get(self, key: bytes) -> Any | None:
        """

        Get the value associated with a key.

        """

    def contains(self, prefix: bytes) -> bool:
        """

        Check if the trie contains a key with a given prefix.

        """

    def values_along_path(self, prefix: bytes) -> list[tuple[int, Any]]:
        """

        Get the values associated with keys along a given prefix.

        """

    def continuations(self, prefix: bytes) -> list[tuple[bytes, Any]]:
        """

        Get the keys and values that follow a given prefix.

        """
