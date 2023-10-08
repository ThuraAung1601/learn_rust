use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
struct Document {
    name: String,
    content: Vec<String>,
    para_count: usize,
    word_count: usize,
}

fn word_counter(text: &str) -> usize {
    let mut word_count = 0;
    // in case we want for each paragraph
    for l in text.split("\n\n") {
        for _w in l.split_whitespace() {
            word_count += 1;
        }
    }
    word_count
}

impl Document {
    // used impl for encapsulation
    fn make_document(file: &str, text: &str) -> Document {
        let mut para_vec = Vec::new();
        for l in text.split("\n\n") {
            para_vec.push(l.to_string());
        }
        Document {
            name: file.to_string(),
            content: para_vec.clone(),
            para_count: para_vec.len(),
            word_count: word_counter(text)
        }
    }

    fn rank_by_para_count(unranked_docs: &[Document]) -> Vec<Document> {
        let mut ranked_docs = unranked_docs.to_vec();
        let len = ranked_docs.len();
        for i in 0..len {
            for j in 0..(len-i-1){
                if ranked_docs[j].para_count < ranked_docs[j+1].para_count {
                    let temp = ranked_docs[j].clone(); 
                    ranked_docs[j] = ranked_docs[j+1].clone();
                    ranked_docs[j+1] = temp;
                }
            }
        }
        ranked_docs
    }
    
    fn rank_by_word_count(unranked_docs: &[Document]) -> Vec<Document> {
        let mut ranked_docs = unranked_docs.to_vec();
        let len = ranked_docs.len();
        for i in 0..len {
            for j in 0..(len-i-1){
                if ranked_docs[j].word_count < ranked_docs[j+1].word_count {
                    let temp = ranked_docs[j].clone(); 
                    ranked_docs[j] = ranked_docs[j+1].clone();
                    ranked_docs[j+1] = temp;
                }
            }
        }
        ranked_docs
    }

    fn generate_html(ranked: &[Document]) -> String {
        // Generate an HTML table
        let mut html_table = String::new();
        html_table.push_str("<table>\n");
        html_table.push_str("<table border=\"1\" style=\"text-align: right;\">\n");
        html_table.push_str("<tr><th>File</th><th>Count</th></tr>\n");
        for doc in ranked {
            html_table.push_str("<tr>");
            html_table.push_str(&format!("<td>{}</td>", doc.name));
            html_table.push_str(&format!("<td>{:>02}</td>", doc.para_count));
            html_table.push_str("</tr>\n");
        }
        html_table.push_str("</table>\n");
        html_table
    }
}

fn main() {
    let files = vec!["f1.txt", "f2.txt", "f3.txt"];
    let mut unranked_docs = Vec::new();
    for f in &files {
        let file = File::open(f).expect("Cannot open the file !!!");
        let reader = BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line.unwrap()); // bug! it will not include \n
            content.push_str(&"\n");
        }
        unranked_docs.push(Document::make_document(f, &content));
    }

    // No 2.1
    let ranked_by_para_count = Document::rank_by_para_count(&unranked_docs);
    let para_html_table = Document::generate_html(&ranked_by_para_count);
    // Write the HTML file
    let mut output_file = File::create("doc_ranked_paracount.html").expect("Failed to create output file");
    write!(output_file, "{}", para_html_table).expect("Failed to write to output file");

    // No 2.2
    let ranked_by_word_count = Document::rank_by_word_count(&unranked_docs);
    let word_html_table = Document::generate_html(&ranked_by_word_count);
    // Write the HTML file
    let mut output_file = File::create("doc_ranked_wordcount.html").expect("Failed to create output file");
    write!(output_file, "{}", word_html_table).expect("Failed to write to output file");
}
