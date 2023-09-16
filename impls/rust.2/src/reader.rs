use lazy_static::lazy_static;
use regex::Regex;

use crate::types::MalNode;

#[derive(Debug, Clone)]
pub struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    pub fn from_str(s: &String) -> Self {
        Reader {
            tokens: tokenize(s),
            position: 0
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
            Some(token) =>  {
                match token.chars().next().unwrap() {
                    '(' => {
                        self.next(); 
                        let ret = Some(self.read_list(')'));
                        self.next();
                        ret
                    }
                    _   => {
                        let ret = Some(self.read_atom());
                        self.next();
                        ret
                    }
                }
            }
            None => None
        }

    }


    fn read_list(&mut self, term: char) -> MalNode {
        let mut mal_list: Vec<MalNode> = Vec::new();
        loop {
            match self.peek() {
                
                Some(token) => {
                    if token.chars().next().unwrap() == term {
                        return MalNode::List(mal_list);
                    } else {

                        match self.read_form() {
                            Some(form) => mal_list.push(form), 
                            None => ()

                        };
                    }
                }
                None => ()
            };
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

fn tokenize(s: &String) -> Vec<String> {
    lazy_static! {
        static ref RE : Regex = Regex::new(
                r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##
            ).unwrap();
    }
    let mut vector: Vec<String> = RE.find_iter(s).map(|mat| mat.as_str().to_owned()).collect();
    for st in vector.iter_mut() {
        *st = st.trim().to_string()
    }
    vector.into_iter().filter(|item| !item.is_empty()).collect()
}

pub fn read_str(s: &String) -> MalNode {
    let s = s.replace(",", " ");
    let mut reader = Reader::from_str(&s);
    // println!("READER LEN {}", reader.tokens.len());
    // for r in &reader.tokens {
    //     println!("T{}T", r)
    // }
    if reader.tokens.len() == 0 {
        return MalNode::Nil
    }
    reader.read_form().unwrap()
}


