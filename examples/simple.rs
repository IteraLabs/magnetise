use magnetise::jaccard_similarity;

fn main() {

    // Short length queries are cases with 
    // higher sensibility towards small differences.
    let query1 = "SELECT * FROM users WHERE age > 30";
    let query2 = "SELECT * FROM users WHERE age > 31";

    let similarity = jaccard_similarity(query1, query2);
    println!("Jaccard similarity: {}", similarity);

}
