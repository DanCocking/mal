#[derive(Debug)]
pub enum MalNode {
    List(Vec<MalNode>),
    Int(i32),
    Sym(String),
    Nil
}

impl ToString for MalNode {
    fn to_string(self: &Self) -> String {
        let mut res_str = String::new();
        match self {
            MalNode::List(elems) => {
                res_str.push('(');
                for elem in elems {
                    res_str.push_str(&elem.to_string());
                    res_str.push(' ');
                }
                res_str = res_str.trim().to_string();
                res_str.push(')');
            }
            MalNode::Int(i) => res_str.push_str(&i.to_string()),
            MalNode::Sym(symbol) => res_str.push_str(&symbol),
            MalNode::Nil => res_str.push_str("Nil"),
        };
        // match self {
        //     MalNode::List(_) => (),
        //     _ => res_str.push(' ')
        // };
        res_str
    }
}
