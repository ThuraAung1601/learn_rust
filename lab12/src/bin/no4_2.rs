trait Text {
    fn value(&self) -> String;
}

#[derive(Clone)]
struct PlainText {
    chars: String,
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText {
            chars: text.to_string(),
        }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }
}

struct RepeatedText {
    subtext: Box<dyn Text>,
    count: usize,
}

impl RepeatedText {
    fn with_parts(subtext: Box<dyn Text>, count: usize) -> RepeatedText {
        RepeatedText {
            subtext,
            count,
        }
    }
}

impl Text for RepeatedText {
    fn value(&self) -> String {
        self.subtext.value().repeat(self.count)
    }
}

fn main() {
    let t1 = PlainText::from("Hi");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(Box::new(t2.clone()), 3);

    println!("{}", t1.value());
    println!("{}", t2.value());
    println!("{}", t3.value());
}

#[test]
fn test_text_repeated() {
    let t1 = PlainText::from("Hi");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(Box::new(t2.clone()), 3);
    // let t4 = RepeatedText::with_parts(Box::new(t3), 5);

    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    // assert_eq!(t4.value(), "[+]".repeat(15));
}