fn wc(file: &str) -> usize {
    if file.is_empty() {
        0
    }
    else {
        file.split_whitespace().count()
    }
}

fn lc(file: &str) -> usize {
    if file.is_empty() {
        0
    }
    else {
        file.split("\n").count()
    }
}

fn pc(file: &str) -> usize {
    if file.is_empty() {
        0
    }
    else {
        file.split("\n\n").count()
    }
}

fn make_document(file: &str) -> Vec<String> {
    file.split("\n\n").map(|s| s.to_string()).collect()
}

fn rank_documents(unranked_ls: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut ranked_ls = unranked_ls.to_vec();
    ranked_ls.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    // ranked_ls.sort_by(|a, b| b.len().cmp(&a.len()));
    ranked_ls
}


fn main() {
    let file0 = "";
    let file1 = "a";
    let file2 = "a\n\nb\n\nc";
    let file3 = "a\nb\n\nc";
    let file4 = "This is testing for word count.";

    println!("{}", file0);
    println!("{}", file1);
    println!("{}", file2);
    println!("{}", file3);
    println!("{}", file4);
    
    println!("Word counts");
    println!("{}", wc(file0));
    println!("{}", wc(file1));
    println!("{}", wc(file2));
    println!("{}", wc(file3));
    println!("{}", wc(file4));

    println!("Line counts");
    println!("{}", lc(file0));
    println!("{}", lc(file1));
    println!("{}", lc(file2));
    println!("{}", lc(file3));
    println!("{}", lc(file4));

    println!("Para Counts");
    println!("{}", pc(file0));
    println!("{}", pc(file1));
    println!("{}", pc(file2));
    println!("{}", pc(file3));
    println!("{}", pc(file4));

    // let para_ls = make_document(file1);
    // println!("{:?}", para_ls);

    let empty_doc = Vec::new();
    let doc1 = make_document(file1);
    let doc2 = make_document(file2);
    let doc3 = make_document(file3);
    let documents = vec![empty_doc, doc1, doc2, doc3];
    println!("unranked: {:?}", documents);
    let ranked = rank_documents(&documents);
    println!("ranked: {:?}", ranked);
}
