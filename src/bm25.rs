use std::collections::HashMap;

/// Tokenizes text into lowercase alphanumeric tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .filter(|t| !t.is_empty() && t.len() > 1)
        .map(|t| t.to_string())
        .collect()
}

/// Computes BM25 score for a document given query terms and corpus statistics.
/// k1: controls document term frequency saturation (default ~1.2)
/// b: controls degree of document length normalization (default ~0.75)
pub fn score_bm25(
    query_tokens: &[String],
    doc_tokens: &[String],
    doc_len: usize,
    avg_doc_len: f64,
    doc_freqs: &HashMap<String, usize>,
    total_docs: usize,
) -> f64 {
    if query_tokens.is_empty() || doc_tokens.is_empty() || total_docs == 0 {
        return 0.0;
    }

    let k1 = 1.2;
    let b = 0.75;

    // Count term frequency in current document
    let mut tf_map: HashMap<&str, usize> = HashMap::new();
    for token in doc_tokens {
        *tf_map.entry(token.as_str()).or_insert(0) += 1;
    }

    let mut score = 0.0;
    let avgdl = if avg_doc_len > 0.0 { avg_doc_len } else { 1.0 };
    let len_norm = 1.0 - b + b * (doc_len as f64 / avgdl);

    for q in query_tokens {
        let tf = *tf_map.get(q.as_str()).unwrap_or(&0) as f64;
        if tf == 0.0 {
            continue;
        }

        let df = *doc_freqs.get(q).unwrap_or(&0) as f64;
        // Standard Lucene/BM25 IDF: ln(1 + (N - df + 0.5) / (df + 0.5))
        let idf = ((total_docs as f64 - df + 0.5) / (df + 0.5) + 1.0).ln();

        let num = tf * (k1 + 1.0);
        let den = tf + k1 * len_norm;
        score += idf * (num / den);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello, World! BM25_score in Rust.");
        assert_eq!(tokens, vec!["hello", "world", "bm25_score", "in", "rust"]);
    }

    #[test]
    fn test_bm25_scoring() {
        let q = tokenize("auth rate limiting");
        let doc1 = tokenize("Implementing JWT auth and rate limiting for API");
        let doc2 = tokenize("Designing database migrations and schema");

        let mut doc_freqs = HashMap::new();
        doc_freqs.insert("auth".to_string(), 1);
        doc_freqs.insert("rate".to_string(), 1);
        doc_freqs.insert("limiting".to_string(), 1);

        let total_docs = 2;
        let avg_doc_len = (doc1.len() + doc2.len()) as f64 / 2.0;

        let s1 = score_bm25(&q, &doc1, doc1.len(), avg_doc_len, &doc_freqs, total_docs);
        let s2 = score_bm25(&q, &doc2, doc2.len(), avg_doc_len, &doc_freqs, total_docs);

        assert!(s1 > 0.0);
        assert_eq!(s2, 0.0);
    }
}
