use std::fs;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, Criterion};
use prefix_tree::PrefixSearch;
use prefix_tree::{AdaptiveRadixTrie, PatriciaTrie};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use art_tree::{Art, ByteString};
use patricia_tree::PatriciaMap;

fn bench_prefix(c: &mut Criterion) {
    let dir = env!("CARGO_MANIFEST_DIR");
    let index = fs::read_to_string(PathBuf::from(dir).join("resources/test/index.txt"))
        .expect("failed to read file");
    let words: Vec<_> = index.lines().map(|s| s.as_bytes()).collect();
    let mut rng = ChaCha8Rng::seed_from_u64(22);
    // sample random word from all words
    let word = *words.choose(&mut rng).unwrap();
    let mut group = c.benchmark_group("prefix_search");

    // benchmark art-tree
    let mut trie: Art<_, _> = Art::new();
    for (i, word) in words.iter().enumerate() {
        trie.insert(ByteString::new(word), i);
    }
    group.bench_with_input("art_tree_insert", word, |b, input| {
        b.iter(|| trie.insert(ByteString::new(input), 1));
    });
    group.bench_with_input("art_tree_get", word, |b, input| {
        b.iter(|| trie.get(&ByteString::new(input)));
    });

    // benchmark patricia_tree
    let mut trie: PatriciaMap<_> = PatriciaMap::new();
    for (i, word) in words.iter().enumerate() {
        trie.insert(word, i);
    }
    group.bench_with_input("patricia_tree_insert", word, |b, input| {
        b.iter(|| trie.insert(input, 1));
    });
    group.bench_with_input("patricia_tree_get", word, |b, input| {
        b.iter(|| trie.get(input));
    });

    // benchmark patricia trie
    let mut trie: PatriciaTrie<_> = words.iter().zip(0..words.len()).collect();
    group.bench_with_input("patricia_trie_insert", word, |b, input| {
        b.iter(|| trie.insert(input, 1));
    });
    group.bench_with_input("patricia_trie_get", word, |b, input| {
        b.iter(|| trie.get(input));
    });
    group.bench_with_input("patricia_trie_contains", word, |b, input| {
        b.iter(|| trie.contains(&input[..input.len().saturating_sub(3)]));
    });

    // benchmark adaptive radix tree
    let mut trie: AdaptiveRadixTrie<_> = words.iter().zip(0..words.len()).collect();
    group.bench_with_input("adaptive_radix_trie_insert", word, |b, input| {
        b.iter(|| trie.insert(input, 1));
    });
    group.bench_with_input("adaptive_radix_trie_get", word, |b, input| {
        b.iter(|| trie.get(input));
    });
    group.bench_with_input("adaptive_radix_trie_contains", word, |b, input| {
        b.iter(|| trie.contains(&input[..input.len().saturating_sub(3)]));
    });
}

criterion_group!(benches, bench_prefix);
criterion_main!(benches);
