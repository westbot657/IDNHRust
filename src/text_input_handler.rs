

// Using this ensures that all text edit indexing uses the same type
type IdxSize = u64;

pub struct Selection {
    start_index: IdxSize,
    end_index: IdxSize,
    slice: String
}

impl Selection {
    pub fn new(start_index: IdxSize, end_index: IdxSize, slice: String) -> Self {
        Self {
            start_index,
            end_index,
            slice
        }
    }
}

// Base for all text boxes
pub struct TextInputHandler {
    content: String,
    allow_newlines: bool,
    max_length: IdxSize,
    enforce_max_length: bool,
    allow_editing: bool,
    cursor_idx: IdxSize,
    secondary_cursors: Vec<IdxSize>,
    selections: Vec<Selection>
}

impl TextInputHandler {

    /// ### Creates a new text input handler
    /// ## Parameters
    /// - **content**: content for the handler to start with
    /// - **allow_newlines**: set to false for single-line text inputs
    /// - **max_length**: set to None for no length limit. Otherwise, specify a max content length
    pub fn new(content: String, allow_newlines: bool, max_length: Option<IdxSize>, allow_editing: bool) -> Self {

        Self {
            content,
            allow_newlines,
            max_length: max_length.unwrap_or(0),
            enforce_max_length: max_length.is_some(),
            allow_editing,
            cursor_idx: 0,
            secondary_cursors: Vec::new(),
            selections: Vec::new()
        }
    }

    pub fn insert_at_cursor(&mut self, content: String) {

    }

    pub fn delete_at_cursor(&mut self) {

    }

    pub fn get_selections(&self) -> Vec<Selection> {
        let mut out = Vec::new();


        out
    }



}

