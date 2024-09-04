use regex::Regex;
use std::collections::HashSet;

fn itemize_query(query: &str, pattern: &str) -> HashSet<String> {
    let re = Regex::new(pattern).unwrap();
    re.find_iter(query)
        .map(|matched| matched.as_str().to_uppercase())
        .collect()
}

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
