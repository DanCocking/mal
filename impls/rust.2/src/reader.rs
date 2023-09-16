use lazy_static::lazy_static;
use regex::Regex;

use crate::types::MalNode;

#[derive(Debug, Clone)]
pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn from_str(s: &str) -> Self {
        Reader {
            tokens: tokenize(s),
            position: 0,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<&String> {
        self.position += 1;
        self.tokens.get(self.position - 1)
    }

    fn read_form(&mut self) -> Option<MalNode> {
        match self.peek() {
            Some(token) => match token.chars().next().unwrap() {
                '(' => {
                    self.next();
                    let ret = Some(self.read_list(')'));
                    self.next();
                    ret
                }
                _ => {
                    let ret = Some(self.read_atom());
                    self.next();
                    ret
                }
            },
            None => None,
        }
    }

    fn read_list(&mut self, term: char) -> MalNode {
        let mut mal_list: Vec<MalNode> = Vec::new();
        loop {
            if let Some(token) = self.peek() {
                if token.chars().next().unwrap() == term {
                    return MalNode::List(mal_list);
                } else if let Some(form) = self.read_form() {
                    mal_list.push(form)
                }
            }
        }
    }

    fn read_atom(&mut self) -> MalNode {
        let token = self.peek().unwrap();
        if token.parse::<i32>().is_ok() {
            MalNode::Int(token.parse::<i32>().unwrap())
        } else {
            MalNode::Sym(token.to_string())
        }
    }
}

fn tokenize(s: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##
        )
        .unwrap();
    }
    let mut vector: Vec<String> = RE.find_iter(s).map(|mat| mat.as_str().to_owned()).collect();
    for st in vector.iter_mut() {
        *st = st.trim().to_string()
    }
    vector.into_iter().filter(|item| !item.is_empty()).collect()
}

pub fn read_str(s: &str) -> MalNode {
    let s = s.replace(',', " ");
    let mut reader = Reader::from_str(&s);
    if reader.tokens.is_empty() {
        return MalNode::Nil;
    }
    reader.read_form().unwrap()
}
