

// Using this ensures that all text edit indexing uses the same type
type IdxSize = usize;

pub struct Selection {
    start_index: IdxSize,
    end_index: IdxSize,
    slice: String
}

impl Clone for Selection {
    fn clone(&self) -> Self {
        Self {
            start_index: self.start_index,
            end_index: self.end_index,
            slice: self.slice.clone()
        }
    }
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

    /// Removes additional cursors and deselects all text, moves the cursor to the specified position (clamped to the length of the text)
    pub fn set_cursor_index(&mut self, idx: IdxSize) {
        self.secondary_cursors.clear();
        self.cursor_idx = idx.min(self.content.len() as IdxSize);

    }

    /// Used for typing, automatically accounts for selected text and multiple cursors
    pub fn insert_at_cursor(&mut self, content: String) {

    }

    /// pastes content. Follows a variety of rules for pasting based
    /// off what is being pasted and how many cursors exist at the time
    pub fn paste_at_cursor(&mut self) {

    }

    /// does a backspace, accounting for selected text and multiple cursors
    pub fn backspace_at_cursor(&mut self) {
        let mut indexes = self.secondary_cursors.clone();
        indexes.push(self.cursor_idx);
        indexes.sort();

        // thing to account for shifting since we're removing content, which de-syncs the indices
        let mut idx_mod = 0;


    }

    /// same as backspace_at_cursor, but with delete behavior
    pub fn delete_at_cursor(&mut self) {

    }

    /// Returns a Vec of all Selections. This Vec will be empty if no text is selected.
    pub fn get_selections(&self) -> Vec<Selection> {
        self.selections.clone()
    }



}

