use regex::Regex;
use std::collections::HashSet;

fn itemize_query(query: &str) -> HashSet<String> {
    let pattern = r"\b(SELECT|WHERE|AND|OR|JOIN|ON|IN|AS|BETWEEN)\B|\w+";
    let re = Regex::new(pattern).unwrap();
    
    re.find_iter(query)
        .map(|matched| matched.as_str().to_uppercase())
        .collect()
}

pub fn jaccard_similarity(query1: &str, query2: &str) -> f64 {

    let tokens1: HashSet<_> = itemize_query(query1);
    println!("{:?}", tokens1);

    let tokens2: HashSet<_> = itemize_query(query2);
    println!("{:?}", tokens2);

    let intersection_size = tokens1.intersection(&tokens2).count();
    println!("{:?}", intersection_size);
    
    let union_size = tokens1.union(&tokens2).count();
    println!("{:?}", union_size);

    if union_size == 0 {
        return 0.0;
    }

    intersection_size as f64 / union_size as f64
}


