use serde_json::Value;

pub enum Token {
    Newline,
    String(String),
    Integer(i64),
    Float(f64),
    Context(String),
    Literal(String),
    Word(String),
    Keyword(String),
    Boolean(bool),
    Macro(String),
    Tag(String),
    Object(String),
    Command(String),
    Comparison(String),
    Comment(String),

    JSONKey(String),
    JSONValue(String),

}

struct CompileContext {}

pub trait Node {
    fn compile(&self, compile_context: &mut CompileContext) -> Value {
        Value::Null
    }
}

macro_rules! node {
    ( $cls:tt => [ $( $name:ident : $tp:ty ),* ] ) => {

        struct $cls {
            $( $name: $tp ),*
        }

        impl $cls {
            pub fn new( $( $name: $expr )* ) -> Self {

                Self {
                    $( $name ),*
                }
            }
        }
    };
}


node!(StatementsNode => [ nodes: Vec<Box<dyn Node>> ]);
impl Node for StatementsNode {
    fn compile(&self, compile_context: &mut CompileContext) -> Value {
        let mut out = Vec::new();
        for node in self.nodes.iter() {
            out.push(node.compile(compile_context));
        }
        Value::Array(out)
    }
}



pub struct ES3Compiler {
    tokens: Vec<Token>,
    body: Box<dyn Node>,
}

impl ES3Compiler {

    pub fn new() -> Self {

        Self {
            tokens: Vec::new(),
            body: Box::new(StatementsNode::new(Vec::new()))
        }
    }

}










