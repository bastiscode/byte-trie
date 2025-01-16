pub mod art;
pub mod patricia;
mod py;

pub use art::AdaptiveRadixTrie;
pub use patricia::PatriciaTrie;

pub trait PrefixSearch {
    type Value;

    fn insert(&mut self, key: &[u8], value: Self::Value) -> Option<Self::Value>;

    fn delete(&mut self, key: &[u8]) -> Option<Self::Value>;

    fn get(&self, key: &[u8]) -> Option<&Self::Value>;

    fn contains(&self, key: &[u8]) -> bool;

    fn contains_prefix(&self, prefix: &[u8]) -> bool;

    fn prefix_matches(&self, key: &[u8]) -> Vec<(usize, &Self::Value)>;

    fn continuations(
        &self,
        prefix: &[u8],
    ) -> Box<dyn Iterator<Item = (Vec<u8>, &Self::Value)> + '_>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::PathBuf};

    use itertools::Itertools;
    use rand::{seq::SliceRandom, Rng, SeedableRng};

    use crate::{AdaptiveRadixTrie, PatriciaTrie, PrefixSearch};

    fn get_prefix_searchers() -> Vec<(&'static str, Box<dyn PrefixSearch<Value = usize>>)> {
        vec![
            ("art", Box::new(AdaptiveRadixTrie::default())),
            ("patricia", Box::new(PatriciaTrie::default())),
        ]
    }

    fn load_prefixes(words: &[String], n: usize) -> Vec<&[u8]> {
        // sample n random prefixes from the words
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(22);
        words
            .choose_multiple(&mut rng, n)
            .into_iter()
            .map(|s| {
                let s = s.as_bytes();
                // choose random prefix length
                let len = rng.gen_range(0..=s.len());
                &s[..len.max(2).min(s.len())]
            })
            .collect()
    }

    fn load_words() -> Vec<String> {
        let dir = env!("CARGO_MANIFEST_DIR");
        let index = fs::read_to_string(PathBuf::from(dir).join("resources/test/index.txt"))
            .expect("failed to read file");
        index.lines().map(|s| s.to_string()).sorted().collect()
    }

    #[test]
    fn test_prefix_search() {
        for (name, mut pfx) in get_prefix_searchers() {
            println!("testing {}", name);
            assert_eq!(pfx.get(b"hello"), None);
            assert_eq!(pfx.get(b""), None);
            assert!(!pfx.contains(b""));
            pfx.insert(b"", 4);
            pfx.insert(b"h", 5);
            pfx.insert(b"hello", 1);
            assert_eq!(pfx.delete(b"hello"), Some(1));
            assert_eq!(pfx.delete(b"hello "), None);
            pfx.insert(b"hello", 1);
            pfx.insert(b"hell", 2);
            pfx.insert(b"hello world", 3);
            assert_eq!(pfx.prefix_matches(b""), vec![(0, &4)]);
            assert_eq!(
                pfx.prefix_matches(b"hello my friend"),
                vec![(0, &4), (1, &5), (4, &2), (5, &1)]
            );
            assert_eq!(
                pfx.prefix_matches(b"hello world!"),
                vec![(0, &4), (1, &5), (4, &2), (5, &1), (11, &3)]
            );
            assert_eq!(pfx.get(b"hello"), Some(&1));
            assert_eq!(pfx.get(b"hell"), Some(&2));
            assert_eq!(pfx.get(b"hello world"), Some(&3));
            assert!(pfx.contains(b"hell"));
            assert!(!pfx.contains(b"hel"));
            assert!(pfx.contains_prefix(b"hel"));
            assert!(pfx.contains(b"hello"));
            assert!(pfx.contains(b""));
            assert!(!pfx.contains(b"hello world!"));
            assert!(!pfx.contains(b"test"));
            assert_eq!(pfx.get(b"hello"), Some(&1));
            assert_eq!(pfx.delete(b"hello"), Some(1));
            assert_eq!(pfx.get(b"hello"), None);
        }
    }

    #[test]
    fn test_prefix_matches() {
        let words = load_words();
        let prefixes = load_prefixes(&words, 1000);

        for (_, mut pfx) in get_prefix_searchers() {
            words.iter().enumerate().for_each(|(i, w)| {
                pfx.insert(w.as_bytes(), i);
            });

            for prefix in &prefixes {
                let path = pfx.prefix_matches(prefix);
                let path_words: Vec<_> = path
                    .iter()
                    .map(|(n, &i)| (&prefix[..*n], words[i].as_bytes()))
                    .collect();
                assert!(
                    path_words.iter().all(|&(p, w)| p == w),
                    "{:?}",
                    path_words
                        .iter()
                        .map(|(p, w)| { (String::from_utf8_lossy(p), String::from_utf8_lossy(w)) })
                        .collect::<Vec<_>>()
                );
                for (i, word) in words.iter().enumerate() {
                    if *prefix == word.as_bytes() {
                        assert!(
                            path.last() == Some(&(word.len(), &i)),
                            "last of {:?} not equal to ({}, {i})",
                            path,
                            word.len()
                        );
                    } else if prefix.starts_with(word.as_bytes()) {
                        assert!(path.iter().any(|(_, &idx)| idx == i));
                    } else {
                        assert!(path.iter().all(|(_, &idx)| idx != i));
                    }
                }
            }
        }
    }

    #[test]
    fn test_insert_delete_contains_prefix() {
        let words = load_words();

        for (_, mut pfx) in get_prefix_searchers() {
            words.iter().enumerate().for_each(|(i, w)| {
                pfx.insert(w.as_bytes(), i);
            });

            for (i, word) in words.iter().enumerate() {
                assert_eq!(pfx.get(word.as_bytes()), Some(&i));
                let bytes = word.as_bytes();
                assert!(pfx.contains_prefix(&bytes[..=bytes.len() / 2]));
            }

            for (i, word) in words.iter().enumerate() {
                let even = i % 2 == 0;
                if even {
                    assert_eq!(pfx.delete(word.as_bytes()), Some(i));
                    assert_eq!(pfx.get(word.as_bytes()), None);
                } else {
                    assert_eq!(pfx.get(word.as_bytes()), Some(&i));
                }
            }
        }
    }
}
