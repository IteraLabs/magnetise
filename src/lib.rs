//! A Rust library to asses the similarity between SQL queries.

use regex::Regex;
use std::collections::HashSet;

/// Itemizes a query string based on a given pattern using regular expressions.
///
/// # Arguments
///
/// * `query` - The input query string to be itemized.
/// * `pattern` - The regular expression pattern used for itemization.
///
/// # Returns
///
/// A `HashSet` containing the itemized tokens from the query, converted to uppercase.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use jaccard_similarity::itemize_query;
///
/// let query = "SELECT * FROM users WHERE age > 30";
/// let pattern = r"\b(SELECT|WHERE|FROM|AND|OR)\B|\w+";
/// let tokens: HashSet<_> = itemize_query(query, pattern);
/// assert_eq!(tokens, ["SELECT", "WHERE", "FROM", "users", "age"].iter().cloned().collect());
/// ```

pub fn itemize_query(query: &str, pattern: &str) -> HashSet<String> {
    let re = Regex::new(pattern).unwrap();
    re.find_iter(query)
        .map(|matched| matched.as_str().to_uppercase())
        .collect()
}

/// Calculates the Jaccard similarity between two queries.
///
/// The Jaccard similarity is a measure of similarity between two sets, defined as the size of the intersection divided by the size of the union of the sets.
///
/// # Arguments
///
/// * `query1` - The first query string.
/// * `query2` - The second query string.
/// * `itemizer_pattern` - The regular expression pattern used for itemizing the queries.
///
/// # Returns
///
/// The Jaccard similarity between the two queries, as a floating-point value between 0.0 and 1.0.
///
/// # Examples
///
/// ```rust
/// use jaccard_similarity::jaccard_similarity;
///
/// let query1 = "SELECT * FROM users WHERE age > 30";
/// let query2 = "SELECT * FROM users WHERE age > 31";
/// let pattern = r"\b(SELECT|WHERE|FROM|AND|OR)\B|\w+";
/// let similarity = jaccard_similarity(query1, query2, pattern);
/// assert!(similarity > 0.7 && similarity < 0.8);
/// ```

pub fn jaccard_similarity(query1: &str, query2: &str, itemizer_pattern: &str) -> f64 {
    let tokens1: HashSet<_> = itemize_query(query1, itemizer_pattern);
    let tokens2: HashSet<_> = itemize_query(query2, itemizer_pattern);
    let intersection_size = tokens1.intersection(&tokens2).count();
    let union_size = tokens1.union(&tokens2).count();

    if union_size == 0 {
        return 0.0;
    }

    let jaccard = intersection_size as f64 / union_size as f64;

    println!("The tokens from 1st query: {:?}", tokens1);
    println!("The tokens from 2nd query: {:?}", tokens2);
    println!(
        "Jaccard = intersection / union => {:?} / {:?} = {:?}",
        intersection_size, union_size, jaccard
    );

    jaccard
}
