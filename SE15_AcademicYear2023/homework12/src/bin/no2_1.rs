#[derive(Clone)]
enum Text {
    Plain(String),
    Repeated(Box<Text>, usize),
    Joined(Vec<Box<Text>>, Box<Text>),
}

impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(subtext, count) => subtext.value().repeat(*count),
            Text::Joined(texts, separator) => {
                let mut result = String::new();
                for (i, text) in texts.iter().enumerate() {
                    result.push_str(&text.value());
                    if i < texts.len() - 1 {
                        result.push_str(&separator.value());
                    }
                }
                result
            }
        }
    }
}

impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        Box::new(t.clone())
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        &self
    }
}

fn main() {
    let t1 = Text::Plain("Hi".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    
    println!("{}", t1.value());
    println!("{}", t2.value());
    println!("{}", t3.value());
    println!("{}", t4.value());
}


#[test]
fn test_text_repeated() {
    let t1 = Text::Plain("Hi".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);

    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    assert_eq!(t4.value(), "[+]".repeat(15));
}

#[test]
fn test_text_composition() {
    let t1 = Text::Plain("x|x".into());
    let t2 = Text::Plain("[+]".into());
    let t3 = Text::Repeated(t2.as_ref().into(), 3);
    let t4 = Text::Repeated(t3.as_ref().into(), 5);
    let mut tvec: Vec<Box<Text>> = Vec::new();
    tvec.push(t1.into());
    tvec.push(t2.into());
    tvec.push(t3.into());
    tvec.push(t4.into());
    let t5 = Text::Plain("--".into());
    let t6 = Text::Joined(tvec, t5.into());
    let ptn = ["x|x", "[+]", &"[+]".repeat(3), &"[+]".repeat(15)];
    let expected = ptn.join("--");
    assert_eq!(t6.value(), expected);
}