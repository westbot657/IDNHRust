use std::cmp::Ordering;
use std::collections::HashMap;
use crate::app::App;

// Using this ensures that all text edit indexing uses the same type
pub type IdxSize = usize;

#[derive(Eq)]
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

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        self.start_index.eq(&other.start_index)
    }
}

impl PartialOrd for Selection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start_index.partial_cmp(&other.start_index)
    }
}

impl Ord for Selection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_index.cmp(&other.start_index)
    }
}


// Base for all text boxes
pub struct TextInputHandler {
    pub content: String,
    allow_newlines: bool,
    max_length: IdxSize,
    enforce_max_length: bool,
    allow_editing: bool,
    cursor_idx: IdxSize,
    secondary_cursors: Vec<IdxSize>,
    selections: Vec<Selection>,
    construction_selections: HashMap<IdxSize, Selection>
}

impl TextInputHandler {

    /// ### Creates a new text input handler
    /// ## Parameters
    /// - **content**: content for the handler to start with
    /// - **allow_newlines**: set to false for single-line text inputs
    /// - **max_length**: set to NONE for no length limit. Otherwise, specify a max content length
    pub fn new(content: String, allow_newlines: bool, max_length: Option<IdxSize>, allow_editing: bool) -> Self {

        Self {
            content,
            allow_newlines,
            max_length: max_length.unwrap_or(0),
            enforce_max_length: max_length.is_some(),
            allow_editing,
            cursor_idx: 0,
            secondary_cursors: Vec::new(),
            selections: Vec::new(),
            construction_selections: HashMap::new()
        }
    }

    pub fn process(&mut self, app: &mut App) {
        for key in &app.keyboard.triggered_keys {

            if app.keybinds.matches_any() {
                println!("Keybind");
                // do nothing because keybinds
            }
            else if key.len() == 1 {
                println!("Type '{}'", key);
                self.insert_at_cursor(key.to_string());
            }
            else if key == "Backspace" {
                println!("Backspace");
                self.backspace_at_cursor();
            }
            else if key == "Delete" {
                println!("Delete");
                self.delete_at_cursor();
            }
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

    /// Removes selected text. Any cursors that end up in the same place collapse into one cursor.
    pub fn collapse_selections(&mut self) {

    }

    /// does a backspace, accounting for selected text and multiple cursors
    pub fn backspace_at_cursor(&mut self) {
        let mut indexes = self.secondary_cursors.clone();
        indexes.push(self.cursor_idx);
        indexes.sort();

        // thing to account for shifting since we're removing content, which de-syncs the indices
        let mut idx_mod = 0;

        let mut regions: Vec<(IdxSize, IdxSize)> = Vec::new();

        for index in indexes {
            if self.get_selection_at_index(index).is_none() {
                if index == 0 {
                    regions.push((index, index));
                } else {
                    regions.push(((index - 1).max(0), index));
                }
            }
        }

        for sel in &self.selections {
            regions.push((sel.start_index+1, sel.end_index))
        }

        regions.sort();

        let mut new_cursor_positions: Vec<IdxSize> = Vec::new();

        for region in regions {
            new_cursor_positions.push(region.0-idx_mod);
            self.content = self.content[0..region.0-idx_mod].to_string() + &self.content[region.1-idx_mod..];
            idx_mod += region.1 - region.0;
        }

        self.cursor_idx = new_cursor_positions[0];
        self.secondary_cursors.append(&mut new_cursor_positions[1..].to_vec());
        self.selections.clear();

    }

    pub fn get_selection_at_index(&self, index: IdxSize) -> Option<Selection> {
        for sel in &self.selections {
            if (sel.start_index..sel.end_index).contains(&index) {
                return Some(sel.clone())
            }
        }

        None
    }

    /// same as backspace_at_cursor, but with delete behavior
    pub fn delete_at_cursor(&mut self) {
        let mut indexes = self.secondary_cursors.clone();
        indexes.push(self.cursor_idx);
        indexes.sort();

        // thing to account for shifting since we're removing content, which de-syncs the indices
        let mut idx_mod = 0;

        let mut regions: Vec<(IdxSize, IdxSize)> = Vec::new();

        for index in indexes {
            if self.get_selection_at_index(index).is_none() {
                regions.push((index, (index).min(self.content.len())));
            }
        }

        for sel in &self.selections {
            regions.push((sel.start_index+1, sel.end_index))
        }

        regions.sort();

        let mut new_cursor_positions: Vec<IdxSize> = Vec::new();

        for region in regions {
            new_cursor_positions.push(region.0-idx_mod);
            self.content = self.content[0..region.0-idx_mod].to_string() + &self.content[region.1-idx_mod..];
            idx_mod += region.1 - region.0;
        }

        self.cursor_idx = new_cursor_positions[0];
        self.secondary_cursors.append(&mut new_cursor_positions[1..].to_vec());
        self.selections.clear();

    }

    /// Returns a Vec of all Selections. This Vec will be empty if no text is selected. Selections will be given in order
    pub fn get_selections(&self) -> Vec<Selection> {
        self.selections.clone()
    }
}


#[cfg(test)]
mod handler_tests {
    use crate::text_input_handler::{Selection, TextInputHandler};

    #[test]
    pub fn test_sorting() {
        let mut regions = Vec::new();

        regions.push((93, 94));
        regions.push((0, 12));
        regions.push((14, 15));

        regions.sort();

        assert_eq!(regions, vec![(0, 12), (14, 15), (93, 94)]);
    }


    #[test]
    pub fn test_backspace_1_cursor() {

        let mut handler: TextInputHandler = TextInputHandler::new("This is test ##1".to_string(), true, None, true);
        //                                                                      ^

        handler.cursor_idx = 14;

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #1");


    }

    #[test]
    pub fn test_backspace_2_cursors() {
        let mut handler: TextInputHandler = TextInputHandler::new("This is tesxt ##2".to_string(), true, None, true);
        //                                                                    ^  ^

        handler.cursor_idx = 12;
        handler.secondary_cursors.push(15);

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #2");

    }

    #[test]
    pub fn test_backspace_selection_edge() {

        let mut handler: TextInputHandler = TextInputHandler::new("This is tesxadt #3".to_string(), true, None, true);
        //                                                                    ^~^

        handler.cursor_idx = 14;
        handler.selections.push(Selection::new(11, 14, "xad".to_string()));

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #3");
    }

    #[test]
    pub fn test_backspace_selection_2_cursors() {
        let mut handler: TextInputHandler = TextInputHandler::new("This is tesaddxadt #4".to_string(), true, None, true);
        //                                                                    ^~~~~^

        handler.cursor_idx = 17;
        handler.secondary_cursors.push(14);

        handler.selections.push(
            Selection::new(11, 17, "addxad".to_string())
        );

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #4");
    }

}

