use std::collections::HashSet;
#[cfg(test)]
#[test]
fn test_itemizer() {
    use magnetise::itemize_query;

    let query = "SELECT * FROM users WHERE age > 30";
    let pattern = r"\b(SELECT|WHERE|FROM|AND|OR)\B|\w+";
    let tokens: HashSet<_> = itemize_query(query, pattern).into_iter().collect();

    let expected_tokens: HashSet<String> = vec!["SELECT", "FROM", "USERS", "WHERE", "AGE", "30"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_similarity() {
    use magnetise::jaccard_similarity;

    let query1 = "SELECT * FROM users WHERE age > 30";
    let query2 = "SELECT * FROM users WHERE age > 31";
    let pattern = r"\b(SELECT|WHERE|FROM|AND|OR)\B|\w+";
    let similarity = jaccard_similarity(query1, query2, pattern);

    assert!(similarity > 0.7 && similarity < 0.8);
}
