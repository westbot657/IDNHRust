use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use crate::app::App;

// Using this ensures that all text edit indexing uses the same type
pub type IdxSize = usize;

#[derive(Eq)]
struct Cursor {
    pub idx: IdxSize,
    pub selection_idx: Option<IdxSize>,
    pub preferred_column: IdxSize
}

impl Cursor {
    pub fn new(idx: IdxSize) -> Self {
        Self {
            idx,
            selection_idx: None,
            preferred_column: 0
        }
    }
    
    pub fn get_range(&self) -> (IdxSize, IdxSize) {
        if self.selection_idx.is_some() {
            let mut i = vec![self.selection_idx.unwrap(), self.idx];
            i.sort();
            (i[0], i[1])
        } else {
            (self.idx, self.idx)
        }
    }
    
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.selection_idx == other.selection_idx && self.preferred_column == other.preferred_column
    }
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

impl Ord for Cursor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl Clone for Cursor {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            selection_idx: self.selection_idx,
            preferred_column: self.preferred_column
        }
    }
}


// Base for all text boxes
pub struct TextInputHandler {
    pub content: String,
    allow_newlines: bool,
    max_length: IdxSize,
    enforce_max_length: bool,
    allow_editing: bool,
    pub cursor: Cursor,
    pub cursors: Vec<Cursor>
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
            cursor: Cursor::new(0),
            cursors: Vec::new()
        }
    }

    pub fn get_line(&self, line: IdxSize) -> Option<&str> {

        let mut lines = self.content.split("\n");

        lines.nth(line)
    }

    pub fn get_line_start_index(&self, line: IdxSize) -> Option<IdxSize> {

        let mut lines = self.content.split_inclusive('\n');

        let mut l = line;

        let mut idx = 0;

        while l > 0 {
            if let Some(s) = lines.next() {
                l -= 1;
                idx += s.len();
            } else {
                return None
            }
        }

        Some(idx)
    }
    
    fn merge_groups(ranges: &mut Vec<(IdxSize, IdxSize)>) {
        
        let rc1 = ranges.clone();
        let mut rc2 = Vec::new();
        
        let mut i = 0;
        let mut j = 0;
        
        for r in rc1 {
            let mut should_add = true;
            for r2 in ranges.clone() {
                if i != j && r2.0 <= r.0 && r.1 <= r2.1 {
                    should_add = false;
                }
                    
                j += 1;
            }
            
            if should_add {
                rc2.push(r);
            }
            
            i += 1;
        }
        
        i = 0;
        while i < rc2.len()-2 {
            j = i+1;
            if rc2[i].1 >= rc2[j].0 {
                let r1 = rc2.remove(i);
                rc2[i] = (r1.0, rc2[i].1)
            } else {
                i += 1;
            }
        }
        
        *ranges = rc2;
        
    }

    /// removes duplicate cursor positions
    fn truncate_cursors(&mut self) {
        self.cursors.sort();
        self.cursors.dedup();
        
        if let Ok(idx) = self.cursors.binary_search(&self.cursor) {
            self.cursors.remove(idx);
        }
    }
    
    fn deselect_all(&mut self) {
        self.cursor.selection_idx = None;
        for cur in &mut self.cursors {
            cur.selection_idx = None;
        }
    }
    
    /// set `left` to true for all cursors to snap to the left side of their selection. set to false to snap to the right side
    fn deselect_all_directional(&mut self, left: bool, move_cursors: bool) {
        if left {
            if self.cursor.selection_idx.is_some() {
                self.cursor.idx = self.cursor.idx.min(self.cursor.selection_idx.unwrap());
            } else if self.cursor.idx > 0 && move_cursors {
                self.cursor.idx -= 1;
            }
            
            for cur in &mut self.cursors {
                if cur.selection_idx.is_some() {
                    cur.idx = cur.idx.min(cur.selection_idx.unwrap());
                } else if cur.idx > 0 && move_cursors {
                    cur.idx -= 1;
                }
            }
            
        } else {
            let l = self.content.len();
            if self.cursor.selection_idx.is_some() {
                self.cursor.idx = self.cursor.idx.min(self.cursor.selection_idx.unwrap());
            } else if self.cursor.idx < l && move_cursors {
                self.cursor.idx += 1;
            }

            for cur in &mut self.cursors {
                if cur.selection_idx.is_some() {
                    cur.idx = cur.idx.min(cur.selection_idx.unwrap());
                } else if cur.idx < l && move_cursors{
                    cur.idx += 1;
                }
            }
        }
    }
    
    fn set_cursor_preference(&mut self) {
        self.cursor.preferred_column = self.get_text_pos(self.cursor.idx).unwrap().1;
        
        let mut cursors = Vec::new();
        
        mem::swap(&mut cursors, &mut self.cursors);
        for cur in &mut cursors {
            cur.preferred_column = self.get_text_pos(cur.idx).unwrap().1;
        }
        self.cursors = cursors;
    }

    pub fn process(&mut self, app: &mut App) -> bool {
        if !self.allow_editing {
            return false;
        }
        let mut out = false;
        for key in &app.keyboard.triggered_keys {

            if app.keybinds.check_binding("Copy") {
                self.copy_at_cursor();
            }
            else if app.keybinds.check_binding("Cut") {
                self.cut_at_cursor();
            }
            else if app.keybinds.check_binding("Paste") {
                self.paste_at_cursor();
            }
            else if app.keybinds.matches_any() {
                println!("Keybind");
                // do nothing because keybinds
            }
            else if key.len() == 1 {
                println!("Type '{}'", key);
                self.insert_at_cursor(app, key.to_string());
                out = true;
            }
            else if key == "Space" {
                self.insert_at_cursor(app, " ".to_string());
                out = true;
            }
            else if key == "Backspace" {
                self.backspace_at_cursor();
                out = true;
            }
            else if key == "Delete" {
                self.delete_at_cursor();
                out = true;
            }
            else if (key == "Return" || key == "Keypad Enter") && self.allow_newlines{
                self.insert_at_cursor(app, "\n".to_string());
                out = true;
            }
            else if key.starts_with("Keypad") {
                if key.rsplit_once(" ").is_some_and(|x| { x.1.len() == 1 }) {
                    self.insert_at_cursor(app, key[key.len()-2..].to_string());
                    out = true;
                }
            }
            else if key == "Left" {
                self.deselect_all_directional(true, true);
                self.truncate_cursors();
            }
            else if key == "Right" {
                self.deselect_all_directional(false, true);
                self.truncate_cursors();
            }
            else if key == "Up" {
                self.deselect_all_directional(true, false);
                self.set_cursor_preference();
            }
            else if key == "Down" {
                self.deselect_all_directional(false, false);
                self.set_cursor_preference();
                
            }
            else {
                println!("Unprocessed event: {}", key);
            }
        }

        out
    }

    /// Returns the `(line, column)` that corresponds to the given index, or None if the index is out of bounds
    /// The index of self.content.len() is considered valid
    /// line and column start at 0
    pub fn get_text_pos(&self, idx: IdxSize) -> Option<(IdxSize, IdxSize)> {
        if idx <= self.content.len() {
            let mut line = 0;
            let mut dx = idx;
            for l in self.content.split_inclusive('\n') {
                if dx < l.len() {
                    return Some((line, dx))
                } else {
                    line += 1;
                    dx -= l.len();
                }
            }
        }
        
        None
    }
    
    /// Removes additional cursors and deselects all text, moves the cursor to the specified position (clamped to the length of the text)
    pub fn set_cursor_index(&mut self, idx: IdxSize) {
        self.cursors.clear();
        self.cursor.idx = idx;
        self.cursor.selection_idx = None;

    }

    pub fn mod_char(&self, app: &App, c: &str) -> String {
        if app.keyboard.shift_held {
            {
                if c == "`" { "~" }
                else if c == "1" {"!"}
                else if c == "2" {"@"}
                else if c == "3" {"#"}
                else if c == "4" {"$"}
                else if c == "5" {"%"}
                else if c == "6" {"^"}
                else if c == "7" {"&"}
                else if c == "8" {"*"}
                else if c == "9" {"("}
                else if c == "0" {")"}
                else if c == "-" {"_"}
                else if c == "=" {"+"}
                else if c == "[" {"{"}
                else if c == "]" {"}"}
                else if c == "\\" {"|"}
                else if c == ";" {":"}
                else if c == "'" {"\""}
                else if c == "," {"<"}
                else if c == "." {">"}
                else if c == "/" {"?"}
                else { c }
            }.to_string()
        } else {
            c.to_lowercase()
        }
    }

    /// Used for typing, automatically accounts for selected text and multiple cursors
    pub fn insert_at_cursor(&mut self, app: &App, content: String) {
        self.collapse_selections();

        let mut offset = 0;
        let mut cursors = Vec::new();

        cursors.push(self.cursor.idx);

        for idx in &self.cursors {
            if !cursors.contains(&idx.idx) {
                cursors.push(idx.idx);
            }
        }
        cursors.sort();

        for cursor in cursors {
            self.content = self.content[0..cursor].to_string() + &self.mod_char(app, &content) + &self.content[cursor..];
            if cursor + offset <= self.cursor.idx {
                self.cursor.idx += content.len();
            }
            offset += content.len();
        }


    }

    /// Copies selected text. if no text is selected then copies the line the cursor is on. if there are multiple cursors, the selections are joined with a newline
    /// content is automatically put into the clipboard and then returned
    pub fn copy_at_cursor(&self) -> String {

        "".to_string()
    }

    /// pastes content. Follows a variety of rules for pasting based
    /// off what is being pasted and how many cursors exist at the time
    pub fn paste_at_cursor(&mut self) {

    }

    /// Cuts selections or lines if there are no selections. Automatically puts content into the clipboard and then returns it
    pub fn cut_at_cursor(&mut self) -> String {


        "".to_string()
    }

    /// Removes selected text. Any cursors that end up in the same place collapse into one cursor.
    pub fn collapse_selections(&mut self) {
        let mut regions: Vec<(IdxSize, IdxSize)> = Vec::new();

        regions.push(self.cursor.get_range());

        for cursor in &self.cursors {
            regions.push(cursor.get_range());
        }

        regions.sort();
        regions.dedup();

        let mut offset = 0;

        TextInputHandler::merge_groups(&mut regions);


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

            let i = (region.0 as i128 - idx_mod as i128).max(0) as IdxSize;
            let i2 = (region.1 as i128 - idx_mod as i128).max(0) as IdxSize;

            new_cursor_positions.push(i);
            self.content = self.content[0..i].to_string() + &self.content[i2..];
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

