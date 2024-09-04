use magnetise::jaccard_similarity;

fn main() {
    let pattern_1 = r"\b(SELECT|WHERE|AND|OR|JOIN|ON|IN|AS|BETWEEN)\B|\w+";

    // Short length queries are cases with
    // higher sensibility towards small differences.
    let query1 = "SELECT * FROM users WHERE age > 30";
    let query2 = "SELECT * FROM users WHERE age > 31";

    let similarity = jaccard_similarity(query1, query2, pattern_1);
    println!("Jaccard similarity: {}", similarity);
}
