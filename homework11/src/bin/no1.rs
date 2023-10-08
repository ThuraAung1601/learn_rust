fn make_document(text: &str) -> Vec<String> {
    let mut para_vec = Vec::new();
    for l in text.split("\n\n") {
        para_vec.push(l.to_string());
    }
    para_vec
}

fn rank_documents(unranked_ls: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut ranked_ls = unranked_ls.to_vec();
    for _k in 0..ranked_ls.len() {
        let len = ranked_ls.len();
        for _i in 0..len{
            for _j in 0..len - _i - 1 {
                if ranked_ls[_j].len() < ranked_ls[_j + 1].len() {
                    let temp = ranked_ls[_j].clone(); // Clone the inner vector
                    ranked_ls[_j] = ranked_ls[_j + 1].clone();
                    ranked_ls[_j + 1] = temp;
                }
            }
        }
    }
    ranked_ls
}

fn main() {
    let file1 = "a";
    let file2 = "a\n\nb\n\nc";
    let file3 = "a\nb\n\nc";

    // No 1.1
    let para_ls = make_document(file1);
    println!("{:?}", para_ls);

    // No 1.2
    let empty_doc = Vec::new();
    let doc1 = make_document(file1);
    let doc2 = make_document(file2);
    let doc3 = make_document(file3);
    let documents = vec![empty_doc, doc1, doc2, doc3];
    println!("unranked: {:?}", documents);
    let ranked = rank_documents(&documents);
    println!("ranked: {:?}", ranked);
}

#[test]
fn test_rank_documents() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";

    let empty_doc = Vec::new();
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![empty_doc.clone(), doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1, empty_doc]);
}