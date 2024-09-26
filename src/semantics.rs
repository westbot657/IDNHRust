use crate::text_input_handler::IdxSize;

pub enum Semantics {
    None,
    JSON,
    ES3,
    ES3JSON,
    ES3asm,
    MD,
    LangServer(String)
}
pub enum UnderlineStyle {
    None,
    Normal((u8, u8, u8, u8)),
    Squiggly((u8, u8, u8, u8)),
    Dotted((u8, u8, u8, u8)),
}
pub mod styles {
    pub const NONE: u8 = 0;
    pub const ITALIC: u8 = 1;
    pub const BOLD: u8 = 2;
    pub const STRIKETHROUGH: u8 = 4;
}

pub enum ControlToken {
    Newline,
    Tab,
    CarriageReturn,
    LineDelta(IdxSize),
    ColDelta(IdxSize)
}

enum RenderToken {
    Semantic(SemanticToken),
    Control(ControlToken)
}

impl RenderToken {
    fn is_semantic() -> bool {true}
    fn is_control() -> bool {true}
}

pub struct SemanticToken {
    pub content: String,
    pub color: (u8, u8, u8, u8),
    pub underline_style: UnderlineStyle,
    pub styles: u8
}


impl SemanticToken {
    pub fn new(content: String, color: (u8, u8, u8, u8), underline_style: UnderlineStyle, styles: u8) -> Self {

        Self {
            content,
            color,
            underline_style,
            styles
        }
    }

    pub fn set_bold(&mut self, bold: bool) {
        if bold {
            self.styles = self.styles | styles::BOLD;
        }
        else {
            self.styles = self.styles & !styles::BOLD;
        }
    }

    pub fn set_italic(&mut self, italic: bool) {
        if italic {
            self.styles = self.styles | styles::ITALIC;
        }
        else {
            self.styles = self.styles & !styles::ITALIC;
        }
    }

    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        if strikethrough {
            self.styles = self.styles | styles::STRIKETHROUGH;
        }
        else {
            self.styles = self.styles & !styles::STRIKETHROUGH;
        }
    }

    pub fn set_style(&mut self, style: u8) {
        self.styles = style & 0b111;
    }

}


#[cfg(test)]
mod semantics_tests {
    use crate::macros::string;
    use crate::semantics::{styles, SemanticToken, UnderlineStyle};

    #[test]
    pub fn test_flags() {
        let mut token = SemanticToken::new(string!("Semantics"), (255, 255, 255, 255), UnderlineStyle::None, styles::ITALIC);

        assert_eq!(token.styles, 0x01);

        token.set_bold(true);

        assert_eq!(token.styles, 0b011);

        token.set_bold(false);

        assert_eq!(token.styles, 0x01);

        token.set_bold(false);

        assert_eq!(token.styles, 0x01);
    }

}


