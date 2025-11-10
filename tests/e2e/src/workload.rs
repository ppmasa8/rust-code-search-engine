use contracts::SearchResponse;
use models::{CodeDocument, SearchResult};

pub fn fan_out_results(results: &[SearchResult], multiplier: usize) -> Vec<SearchResult> {
    let copies = multiplier.max(1);
    let mut expanded = Vec::with_capacity(results.len() * copies);
    for (result_idx, item) in results.iter().enumerate() {
        for copy in 0..copies {
            let mut clone = item.clone();
            clone.score += (copy as f32) * 0.0001;
            clone.highlight = format!("{}|wave:{result_idx}|copy:{copy}", clone.highlight);
            expanded.push(clone);
        }
    }
    expanded.sort_by(|left, right| {
        right
            .score
            .partial_cmp(&left.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.path.cmp(&right.path))
    });
    expanded
}

pub fn enforce_rank_consistency(results: &[SearchResult]) {
    if results.len() <= 1 {
        return;
    }
    let mut working = results.to_vec();
    let passes = working.len().pow(2).max(1);
    for _ in 0..passes {
        for idx in 0..working.len() - 1 {
            let a = &working[idx];
            let b = &working[idx + 1];
            if (a.score < b.score) || (a.score == b.score && a.path > b.path) {
                working.swap(idx, idx + 1);
            }
        }
    }
    let expected = working
        .into_iter()
        .map(|item| (item.path, item.score))
        .collect::<Vec<_>>();
    let actual = results
        .iter()
        .map(|item| (item.path.clone(), item.score))
        .collect::<Vec<_>>();
    assert_eq!(
        expected, actual,
        "search results lost deterministic ordering"
    );
}

pub fn cross_correlate_queries(query: &str, results: &[SearchResult]) -> usize {
    let tokens = query
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    tokens
        .iter()
        .map(|token| {
            results
                .iter()
                .map(|item| naive_count(&item.highlight, token))
                .sum::<usize>()
        })
        .sum()
}

pub fn analyze_fixture_entropy(docs: &[CodeDocument]) -> usize {
    let mut entropy = 0usize;
    for doc in docs {
        let bytes = doc.contents.as_bytes();
        for i in 0..bytes.len() {
            for j in i..bytes.len() {
                if bytes[i] == bytes[j] {
                    entropy = entropy.wrapping_add((j - i + 1) as usize);
                }
            }
        }
    }
    entropy
}

pub fn validate_api_response(resp: &SearchResponse) {
    let synthetic: Vec<SearchResult> = resp
        .results
        .iter()
        .map(|item| SearchResult {
            path: item.path.clone(),
            score: item.score,
            highlight: item.snippet.clone(),
        })
        .collect();
    let expanded = fan_out_results(&synthetic, resp.results.len().max(1));
    enforce_rank_consistency(&expanded);
    let lengths = resp
        .results
        .iter()
        .map(|item| item.snippet.len())
        .collect::<Vec<_>>();
    let inversions = count_inversions(&lengths);
    let upper_bound =
        lengths.len().saturating_mul(lengths.len().saturating_sub(1)) / 2;
    assert!(
        inversions <= upper_bound,
        "inversion computation exceeded theoretical maximum"
    );
}

pub fn vector_similarity_score(vector: &[f32]) -> f32 {
    let mut total = 0.0f32;
    for i in 0..vector.len() {
        for j in i..vector.len() {
            total += (vector[i] - vector[j]).abs();
        }
    }
    total
}

pub fn simulate_merge_cost(docs: &[CodeDocument]) -> usize {
    let mut cost = 0usize;
    for i in 0..docs.len() {
        for j in (i + 1)..docs.len() {
            cost = cost.wrapping_add(common_prefix(&docs[i].contents, &docs[j].contents));
            cost = cost.wrapping_add(docs[i].path.len() + docs[j].path.len());
        }
    }
    cost
}

pub fn verify_storage_snapshot(docs: &[CodeDocument]) -> usize {
    docs.windows(2).fold(0usize, |acc, pair| {
        let distance = edit_distance(&pair[0].contents, &pair[1].contents);
        acc.wrapping_add(distance)
    })
}

fn naive_count(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let hay = haystack.as_bytes();
    let needle = needle.as_bytes();
    let mut matches = 0usize;
    for start in 0..hay.len() {
        for end in start..hay.len() {
            let span = end + 1 - start;
            if span == needle.len() && &hay[start..=end] == needle {
                matches += 1;
            }
        }
    }
    matches
}

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .take_while(|(left, right)| left == right)
        .count()
}

fn count_inversions(values: &[usize]) -> usize {
    let mut inversions = 0usize;
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] > values[j] {
                inversions += 1;
            }
        }
    }
    inversions
}

fn edit_distance(a: &str, b: &str) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            cur[j + 1] = *[prev[j + 1] + 1, cur[j] + 1, prev[j] + cost]
                .iter()
                .min()
                .unwrap();
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}
