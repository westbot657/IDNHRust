use std::collections::HashMap;
use serde_json::{Map, Value};

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
    ( $cls:tt => [ $( $v:vis $name:ident : $tp:ty ),* ]) => {
        struct $cls {
            $( $v $name: $tp ),*
        }
        impl $cls {
            pub fn new( $( $name : $tp ),* ) -> Self {

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
        for node in &self.nodes {
            out.push(node.compile(compile_context));
        }
        Value::Array(out)
    }
}

node!(CompareNode => [ left: Box<dyn Node>, op: String, right: Box<dyn Node> ]);
impl Node for CompareNode {
    fn compile(&self, compile_context: &mut CompileContext) -> Value {
        let mut out = Map::new();
        out.insert("left".to_string(), self.left.compile(compile_context));
        out.insert("op".to_string(), Value::String(self.op.clone()));
        out.insert("right".to_string(), self.right.compile(compile_context));
        Value::Object(out)
    }
}

node!(IfNode => [ condition: Box<dyn Node>, body: Box<dyn Node>, else_node: Option<Box<dyn Node>> ]);
impl Node for IfNode {
    fn compile(&self, compile_context: &mut CompileContext) -> Value {
        let mut out = Map::new();
        out.insert("#check".to_string(), self.condition.compile(compile_context));
        out.insert("true".to_string(), self.body.compile(compile_context));
        if let Some(else_node) = &self.else_node {
            out.insert("false".to_string(), else_node.compile(compile_context));
        } else {
            out.insert("false".to_string(), Value::Null);
        }
        Value::Object(out)
    }
}

struct PositionedToken {
    token: Token,
    index: usize,
    line: usize,
    column: usize,
}

macro_rules! tok {
    ( $tk:expr @ $idx:expr, $line:expr, $col:expr ) => {
        PositionedToken {
            token: $tk,
            index: $idx,
            line: $line,
            column: $col
        }
    };
}

pub struct ES3Compiler {
    tokens: Vec<Token>,
    body: Box<dyn Node>,
    patterns: HashMap<&'static str, &'static str>
}

impl ES3Compiler {

    pub fn new() -> Self {

        let mut patterns = HashMap::new();

        patterns.insert(r"\/\/.*", "ignore");
        patterns.insert(r"(?<!\/)\/\*(\*[^/]|[^*])+\*\/", "ignore");
        patterns.insert(r"\\#\\![^\n;]*;?", "CONTEXT");
        patterns.insert("(\"(\\\\.|[^\"\\\\])*\"|\'(\\\\.|[^\'\\\\])*\')", "STRING");
        patterns.insert(r"@[^ ]*", "TAG");
        patterns.insert(r"\$[a-zA-Z_][a-zA-Z0-9_]*", "MACRO");
        patterns.insert(r"\b(true|false)\b", "BOOLEAN");
        patterns.insert(r"\<[^<> ]+\>", "OBJECT");
        patterns.insert(r"(<=|>=|<|>|==|!=)", "COMP");
        patterns.insert(r"(\.\.|::)", "CONCAT");
        patterns.insert(r"\b(new|move)\b", "COMMAND");
        patterns.insert(r"\b(if|elif|else|while|for|in|and|not|or|none|match|case|class|def|break|continue)\b", "KEYWORD");
        patterns.insert(r"[a-zA-Z_][a-zA-Z0-9_]*", "WORD");
        patterns.insert(r"(\d+(\.\d+)?|\.\d+)", "NUMBER");
        patterns.insert(r"[=\-+*/()&\[\]{},#%:|^\.\$;~`]", "LITERAL");
        patterns.insert(r"\n+", "NEWLINE");
        patterns.insert(r"[\t ]+", "ignore");


        Self {
            tokens: Vec::new(),
            body: Box::new(StatementsNode::new(Vec::new())),
            patterns
        }
    }

    /// attempts to tokenize the input value.
    /// If tokenization fails for whatever reason, `self.tokens` will be unaffected and an error will be returned.
    /// this means `self.tokens` will always hold tokens from the last successful tokenization
    pub fn tokenize(&mut self, input: &str) -> Result<(), String> {

        let mut pat = "(".to_string();

        for key in self.patterns.keys() {
            pat += &("|".to_string() + key);
        }
        pat += ")";

        let pattern = regex::Regex::new(&pat).unwrap();

        let mut tokens_out: Vec<PositionedToken> = Vec::new();
        let mut idx: usize = 0;
        let mut line: usize = 1;
        let mut column: usize = 0;

        for mat in pattern.find_iter(input) {
            'pattern_loop: for (key, tp) in &self.patterns {
                let sub_pattern = regex::Regex::new(key).unwrap();
                if sub_pattern.find(mat.as_str()).is_some() {
                    idx += mat.len();
                    if tp == &"ignore" {
                        column += mat.len();
                        break 'pattern_loop
                    }
                    else if tp == &"NEWLINE" {
                        column = 0;
                        line += mat.len();
                        break 'pattern_loop
                    }
                    else if tp == &"STRING" {
                        column += input[mat.range()].rsplit_once("\n").unwrap_or(("", &input[mat.range()])).1.len();
                        line += input[mat.range()].split("\n").count() - 1;
                        tokens_out.push(tok!(Token::String(input[mat.range()].to_string()) @ idx, line, column));
                    }


                    if tp == &"CONTEXT" {}
                    else if tp == &"TAG" {}
                    else if tp == &"MACRO" {}
                    else if tp == &"BOOLEAN" {}
                    else if tp == &"OBJECT" {}
                    else if tp == &"COMP" {}
                    else if tp == &"CONCAT" {}
                    else if tp == &"COMMAND" {}
                    else if tp == &"KEYWORD" {}
                    else if tp == &"WORD" {}
                    else if tp == &"NUMBER" {}
                    else if tp == &"LITERAL" {}

                    break;
                }
            }
        }

        Ok(())
    }


}










