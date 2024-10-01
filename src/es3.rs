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

    Error(String)
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

pub mod style_flags {
    pub const BOLD: u8          = 0b0000_0001;
    pub const ITALIC: u8        = 0b0000_0010;
    pub const UNDERLINE: u8     = 0b0000_0100;
    pub const STRIKETHROUGH: u8 = 0b0000_1000;
    pub const ERROR: u8         = 0b0001_0000;
    pub const WARNING: u8       = 0b0010_0000;
    pub const FADED: u8         = 0b0100_0000;
    pub const BACKGROUND: u8    = 0b1000_0000;
}

struct TokenStyle {
    flags: u8
}
impl TokenStyle {
    /// ```
    /// flags: 0b 0 0 0 0 0 0 0 0
    ///           ^ ^ ^ ^ ^ ^ ^ ^
    ///           | | | | | | | Bold
    ///           | | | | | | Italic
    ///           | | | | | Underline
    ///           | | | | Strikethrough
    ///           | | | Error
    ///           | | Warning
    ///           | Faded
    ///           Background
    /// ```
    pub fn new(flags: u8) -> Self {

        Self {
            flags
        }
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn is_bold(&self) -> bool {
        self.flags & style_flags::BOLD != 0
    }
    pub fn is_italic(&self) -> bool {
        self.flags & style_flags::ITALIC != 0
    }
    pub fn is_underline(&self) -> bool {
        self.flags & style_flags::UNDERLINE != 0
    }
    pub fn is_strikethrough(&self) -> bool {
        self.flags & style_flags::STRIKETHROUGH != 0
    }
    pub fn is_error(&self) -> bool {
        self.flags & style_flags::ERROR != 0
    }
    pub fn is_warning(&self) -> bool {
        self.flags & style_flags::WARNING != 0
    }
    pub fn is_faded(&self) -> bool {
        self.flags & style_flags::FADED != 0
    }
    pub fn is_background(&self) -> bool {
        self.flags & style_flags::BACKGROUND != 0
    }

    pub fn clear_flags(&mut self) {
        self.flags = 0;
    }
    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

}

struct PositionedToken {
    token: Token,
    index: usize,
    line: usize,
    column: usize,
    style: TokenStyle,
    links: HashMap<String, String>
}

macro_rules! tok {
    ( $tk:expr => $idx:expr, $line:expr, $col:expr ) => {
        PositionedToken {
            token: $tk,
            index: $idx,
            line: $line,
            column: $col,
            style: TokenStyle::new(0),
            links: HashMap::new()
        }
    };
}

pub struct ES3Compiler {
    pub tokens: Vec<PositionedToken>,
    pub body: Box<dyn Node>,
    patterns: HashMap<&'static str, &'static str>
}

impl ES3Compiler {

    pub fn new() -> Self {

        let mut patterns = HashMap::new();

        patterns.insert(r"\/\/.*", "COMMENT");
        patterns.insert(r"(?<!\/)\/\*(\*[^/]|[^*])+\*\/", "COMMENT");
        patterns.insert(r"\\#\\![^\n;]*;?", "CONTEXT");
        patterns.insert("(\"(\\\\.|[^\"\\\\])*\"|\'(\\\\.|[^\'\\\\])*\')", "STRING");
        patterns.insert(r"=>[^ ]*", "TAG");
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
        patterns.insert(r".", "ERROR");


        Self {
            tokens: Vec::new(),
            body: Box::new(StatementsNode::new(Vec::new())),
            patterns
        }
    }

    pub fn tokenize(&mut self, input: &str) {

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
        let mut matches = false;

        for mat in pattern.find_iter(input) {
            matches = false;
            'pattern_loop: for (key, tp) in &self.patterns {
                let sub_pattern = regex::Regex::new(key).unwrap();
                if sub_pattern.find(mat.as_str()).is_some() {
                    let str_val = input[mat.range()].to_string();
                    matches = true;
                    idx += mat.len();
                    if *tp == "ignore" {
                        column += mat.len();
                        break 'pattern_loop
                    }
                    else if *tp == "NEWLINE" {
                        column = 0;
                        line += mat.len();
                        break 'pattern_loop
                    }
                    else if *tp == "STRING" {
                        let li = str_val.split("\n").count() - 1;
                        line += li;
                        if li > 0 {
                            column = str_val.rsplit_once("\n").unwrap().1.len();
                        } else {
                            column += str_val.len();
                        }

                        tokens_out.push(tok!(Token::String(str_val) => idx, line, column));
                        break 'pattern_loop
                    }

                    column += mat.len();


                    if *tp == "CONTEXT" {
                        tokens_out.push(tok!(Token::Context(str_val) => idx, line, column));
                    }
                    else if *tp == "COMMENT" {
                        tokens_out.push(tok!(Token::Comment(str_val) => idx, line, column));
                    }
                    else if *tp == "TAG" {
                        tokens_out.push(tok!(Token::Tag(str_val) => idx, line, column));
                    }
                    else if *tp == "MACRO" {
                        tokens_out.push(tok!(Token::Macro(str_val) => idx, line, column));
                    }
                    else if *tp == "BOOLEAN" {
                        tokens_out.push(tok!(Token::Boolean(str_val.to_lowercase().parse::<bool>().unwrap()) => idx, line, column));
                    }
                    else if *tp == "OBJECT" {
                        tokens_out.push(tok!(Token::Object(str_val) => idx, line, column));
                    }
                    else if *tp == "COMP" {
                        tokens_out.push(tok!(Token::Comparison(str_val) => idx, line, column));
                    }
                    else if *tp == "CONCAT" {
                        tokens_out.push(tok!(Token::Literal(str_val) => idx, line, column));
                    }
                    else if *tp == "COMMAND" {
                        tokens_out.push(tok!(Token::Command(str_val) => idx, line, column));
                    }
                    else if *tp == "KEYWORD" {
                        tokens_out.push(tok!(Token::Keyword(str_val) => idx, line, column));
                    }
                    else if *tp == "WORD" {
                        tokens_out.push(tok!(Token::Word(str_val) => idx, line, column));
                    }
                    else if *tp == "NUMBER" {
                        if str_val.contains(".") {
                            tokens_out.push(tok!(Token::Float(str_val.parse::<f64>().unwrap()) => idx, line, column));
                        } else {
                            tokens_out.push(tok!(Token::Integer(str_val.parse::<i64>().unwrap()) => idx, line, column));
                        }
                    }
                    else if *tp == "LITERAL" {
                        tokens_out.push(tok!(Token::Literal(str_val) => idx, line, column));
                    }
                    else if *tp == "ERROR" {
                        let mut token = tok!(Token::Error(str_val) => idx, line, column);
                        token.style.set_flag(style_flags::ERROR, true);
                        tokens_out.push(token);
                    }

                    break 'pattern_loop;
                }
            }
        }

        self.tokens = tokens_out;

    }

    /// Attempts to parse the current token vec into an AST.
    /// The parser will attempt to recover from errors, and will collect errors to be returned at the end of parsing.
    /// This function will also update the tokens with more informed highlighting, and create links for
    /// definitions, uses, etc...
    pub fn parse(&mut self) -> Result<(), Vec<String>> {



        Ok(())
    }


}










