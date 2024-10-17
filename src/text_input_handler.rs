use std::cmp::Ordering;
use std::mem;
use std::str::FromStr;
use fancy_regex::Regex;
use crate::app::App;

// Using this ensures that all text edit indexing uses the same type
pub type IdxSize = usize;

#[derive(Eq)]
pub struct Cursor {
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

    pub fn selection(idx: IdxSize, selection_idx: IdxSize) -> Self {
        Self {
            idx,
            selection_idx: Some(selection_idx),
            preferred_column: 0
        }
    }
    
    pub fn get_range(&self) -> (IdxSize, IdxSize) {
        if self.selection_idx.is_some() {
            let mut i = [self.selection_idx.unwrap(), self.idx];
            i.sort();
            (i[0], i[1])
        } else {
            (self.idx, self.idx)
        }
    }

    pub fn get_backspace_range(&self) -> (IdxSize, IdxSize) {
        if self.selection_idx.is_some() {
            let mut i = [self.selection_idx.unwrap(), self.idx];
            i.sort();
            (i[0], i[1])
        } else if self.idx == 0 {
            (self.idx, self.idx)
        } else {
            (self.idx - 1, self.idx)
        }
    }

    pub fn get_delete_range(&self, content_length: IdxSize) -> (IdxSize, IdxSize) {
        if self.selection_idx.is_some() {
            let mut i = [self.selection_idx.unwrap(), self.idx];
            i.sort();
            (i[0], i[1])
        } else if self.idx == content_length {
            (self.idx, self.idx)
        } else {
            (self.idx, self.idx + 1)
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
        Some(self.cmp(other))
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
    pub cursors: Vec<Cursor>,
    pub flags: u8,
    // flags are: ANS- --HC
    // A: alpha   N: numeric   S: special chars   H: should push to history   C: should focus cursor
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
            cursors: Vec::new(),
            flags: 0b_0000_0000
        }
    }
    
    /// returns whether the cursor was updated in a way that would cause a traditional editor to focus on it.
    /// The flag is set to false after querying
    pub fn should_focus_cursor(&mut self) -> bool {
        
        let out = self.flags & 0b_0000_0001 != 0;
        if out {
            self.set_focus_cursor(false);
        }
        out
    }
    
    fn set_focus_cursor(&mut self, val: bool) {
        if val {
            self.flags |= 0b_0000_0001;
        } else {
            self.flags &= !0b_0000_0001;
        }
    }
    
    /// Use to query whether to push to history. Sets the flag to false after being called
    pub fn should_update_history(&mut self) -> bool {
        let out = self.flags & 0b_0000_0010 != 0;
        if out {
            self.set_update_history(false);
        }
        out
    }
    
    fn set_update_history(&mut self, val: bool) {
        if val {
            self.flags |= 0b_0000_0010;
        } else {
            self.flags &= !0b_0000_0010;
        }
    }
    
    fn set_typing_flags(&mut self, c: char) {
        let f = self.flags;
        self.flags &= !0b_1110_0000;
        if c.is_alphabetic() {
            self.flags |= 0b_1000_0000;
        }
        else if c.is_numeric() {
            self.flags |= 0b_0100_0000;
        }
        else {
            self.flags |= 0b_0010_0000;
        }
        if f != self.flags {
            self.set_update_history(true);
        }
    }
    

    /// returns the specified line if it exists. Trailing newline is not included
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
                idx += s.chars().count();
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
        let mut j;
        
        for r in rc1 {
            let mut should_add = true;
            j = 0;
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
        while rc2.len() >= 2 && i <= rc2.len()-2 {
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
    
    pub fn get_selections(&self) -> Vec<(IdxSize, IdxSize)> {
        let mut ranges = Vec::new();
        
        if self.cursor.selection_idx.is_some() {
            ranges.push(self.cursor.get_range());
        }
        
        for cursor in &self.cursors {
            if cursor.selection_idx.is_some() {
                ranges.push(cursor.get_range());
            }
        }
        
        TextInputHandler::merge_groups(&mut ranges);
        
        ranges
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
            let l = self.content.chars().count();
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

    pub fn ctrl_move(&mut self, left: bool) {
        println!("Ctrl move");
        if left {
            let pattern = Regex::new(r"(\w+ *|. *)").unwrap();
            let (l, c) = self.get_text_pos(self.cursor.idx).unwrap();
            let mut line = self.get_line(l).unwrap().to_string();
            let mut offset = 0;
            
            for mat in pattern.find_iter(&line) {
                println!("{:?}", mat);
                if let Ok(m) = mat {
                    println!("match left: {:?}", m);
                    if c <= m.end() {
                        offset = m.start();
                        self.cursor.idx -= c-offset;
                        break
                    }
                }
            }
            
            let mut cursors = Vec::new();
            
            mem::swap(&mut cursors, &mut self.cursors);
            
            for cursor in &mut cursors {
                let (l, c) = self.get_text_pos(cursor.idx).unwrap();
                line = self.get_line(l).unwrap().to_string();
                let offset;

                for mat in pattern.find_iter(&line) {
                    println!("{:?}", mat);
                    if let Ok(m) = mat {
                        if c <= m.end() {
                            offset = m.start();
                            cursor.idx -= c-offset;
                            break
                        }
                    }
                }
            }
            
            mem::swap(&mut cursors, &mut self.cursors);
            
            

        } else {
            let pattern = Regex::new(r"( *\w+| *.)").unwrap();
            let (l, c) = self.get_text_pos(self.cursor.idx).unwrap();
            let mut line = self.get_line(l).unwrap().to_string();
           
            let mut offset = 0;

            for mat in pattern.find_iter(&line) {
                println!("{:?}", mat);
                if let Ok(m) = mat {
                    if m.start() <= c && c < m.end() {
                        offset = m.end();
                        self.cursor.idx += offset-c;
                    }
                }
            }

            let mut cursors = Vec::new();

            mem::swap(&mut cursors, &mut self.cursors);

            for cursor in &mut cursors {
                let (l, c) = self.get_text_pos(cursor.idx).unwrap();
                line = self.get_line(l).unwrap().to_string();
                let mut offset = 0;

                for mat in pattern.find_iter(&line) {
                    println!("{:?}", mat);
                    if let Ok(m) = mat {
                        if m.start() <= c && c < m.end() {
                            offset = m.end();
                            self.cursor.idx += offset-c;
                        }
                    }
                }
            }

            mem::swap(&mut cursors, &mut self.cursors);
            
        }
    }

    /// Simply moves all cursors up or down. selection index is untouched and nothing is force-deselected
    pub fn move_cursors(&mut self, up: bool) {

        if up {
            let (mut line, mut column) = self.get_text_pos(self.cursor.idx).unwrap();
            if line == 0 {
                self.cursor.idx = 0;
                self.cursor.preferred_column = 0;
            } else {
                let ln = self.get_line(line - 1).unwrap();
                let col = ln.chars().count().min(self.cursor.preferred_column);
                self.cursor.idx = self.get_index(line-1, col).unwrap();
            }

            let mut cursors = Vec::new();
            mem::swap(&mut cursors, &mut self.cursors);

            for cursor in &mut cursors {
                (line, column) = self.get_text_pos(cursor.idx).unwrap();
                if line == 0 {
                    cursor.idx = 0;
                    cursor.preferred_column = 0;
                } else {
                    let ln = self.get_line(line - 1).unwrap();
                    let col = ln.chars().count().min(cursor.preferred_column);
                    cursor.idx = self.get_index(line-1, col).unwrap();
                }
            }

            mem::swap(&mut cursors, &mut self.cursors);

        }
        else {
            let (mut line, mut column) = self.get_text_pos(self.cursor.idx).unwrap();

            let next_line = self.get_line(line + 1);

            if next_line.is_none() {
                self.cursor.idx = self.content.chars().count();
                self.cursor.preferred_column = self.get_line(line).unwrap().chars().count();
            } else {
                let col = next_line.unwrap().chars().count().min(self.cursor.preferred_column);
                self.cursor.idx = self.get_index(line+1, col).unwrap();
            }

            let mut cursors = Vec::new();
            mem::swap(&mut cursors, &mut self.cursors);

            for cursor in &mut cursors {
                (line, column) = self.get_text_pos(cursor.idx).unwrap();
                let next_line = self.get_line(line + 1);

                if next_line.is_none() {
                    cursor.idx = self.content.chars().count();
                    cursor.preferred_column = self.get_line(line).unwrap().chars().count();
                } else {
                    let col = next_line.unwrap().chars().count().min(cursor.preferred_column);
                    cursor.idx = self.get_index(line+1, col).unwrap();
                }
            }

            mem::swap(&mut cursors, &mut self.cursors);

        }
    }

    /// processes typing events, arrow key movement, copy/paste, and other relevant keybinds
    pub fn process(&mut self, app: &mut App) -> bool {
        if !self.allow_editing {
            return false;
        }
        let mut out = false;

        if app.keybinds.check_binding("Copy") {
            self.copy_at_cursor();
            self.set_cursor_preference();
            self.set_focus_cursor(true);

        }
        else if app.keybinds.check_binding("Cut") {
            self.cut_at_cursor();
            self.set_cursor_preference();
            self.set_focus_cursor(true);
            self.set_update_history(true);
        }
        else if app.keybinds.check_binding("Paste") {
            if !(self.enforce_max_length && self.content.chars().count() >= self.max_length) {
                self.paste_at_cursor();
                if self.content.chars().count() > self.max_length {
                    // self.content = self.content[0..self.max_length].to_owned();
                    self.content = self.content.chars().take(self.max_length).collect::<String>()
                }
                self.set_cursor_preference();
                self.set_focus_cursor(true);
                self.set_update_history(true);
            }
        }
        else if app.keybinds.check_binding("Select-All") {
            self.cursor.idx = self.content.chars().count();
            self.cursor.selection_idx = Some(0);
            self.cursors.clear();
        }
        
        for key in &app.keyboard.triggered_keys {
            
            if app.keybinds.matches_any() || app.keyboard.alt_held {
                // println!("Keybind");
                // do nothing because keybinds
            }
            else if key.chars().count() == 1 {
                if self.enforce_max_length && self.content.chars().count() >= self.max_length { continue }
                // println!("Type '{}'", key);
                self.insert_at_cursor(app, key.to_string());
                out = true;
                self.set_cursor_preference();
                self.set_focus_cursor(true);
                if let Ok(c) = key.parse::<char>() {
                    self.set_typing_flags(c);
                }
            }
            else if key == "Tab" {
                if self.enforce_max_length && self.content.chars().count() >= self.max_length { continue }
                
                self.tab_at_cursor(app);
                
                out = true;
                self.set_cursor_preference();
                self.set_focus_cursor(true);
                self.set_typing_flags(' ');
            }
            // else if key == "Space" {
            //     if self.enforce_max_length && self.content.chars().count() >= self.max_length { continue }
            //     self.insert_at_cursor(app, " ".to_string());
            //     out = true;
            //     self.set_cursor_preference();
            //     self.set_focus_cursor(true);
            // }
            else if key == "Backspace" {
                self.backspace_at_cursor();
                out = true;
                self.set_cursor_preference();
                self.set_focus_cursor(true);
            }
            else if key == "Delete" {
                self.delete_at_cursor();
                out = true;
                self.set_cursor_preference();
                self.set_focus_cursor(true);
            }
            else if key == "Return" || key == "Keypad Enter" {
                if self.allow_newlines {
                    if self.enforce_max_length && self.content.chars().count() >= self.max_length { continue; }
                    self.insert_at_cursor(app, "\n".to_string());
                    out = true;
                    self.set_cursor_preference();
                }
                self.set_focus_cursor(true);
                self.set_update_history(true);
            }
            // else if key.starts_with("Keypad") {
            //     if self.enforce_max_length && self.content.chars().count() >= self.max_length { continue }
            //     if key.rsplit_once(" ").is_some_and(|x| { x.1.chars().count() == 1 }) {
            //         self.insert_at_cursor(app, key[key.chars().count()-1..].to_string());
            //         out = true;
            //         self.set_cursor_preference();
            //         self.set_focus_cursor(true);
            //     }
            // }
            else if key == "Left" {
                if app.keyboard.shift_held {
                    if self.cursor.selection_idx.is_none() {
                        self.cursor.selection_idx = Some(self.cursor.idx);
                    }
                    if !app.keyboard.ctrl_held {
                        if self.cursor.idx > 0 {
                            self.cursor.idx -= 1;
                        }
                    }
                    
                    for cursor in &mut self.cursors {
                        if cursor.selection_idx.is_none() {
                            cursor.selection_idx = Some(cursor.idx);
                        }
                        if !app.keyboard.ctrl_held {
                            if cursor.idx > 0 {
                                cursor.idx -= 1;
                            }
                        }
                    }

                    if app.keyboard.ctrl_held {
                        self.ctrl_move(true);
                    }
                    
                } else {
                    if app.keyboard.ctrl_held {
                        self.ctrl_move(true);
                    } else {
                        self.deselect_all_directional(true, true);
                    }
                }
                self.truncate_cursors();
                self.set_cursor_preference();
                out = true;
                self.set_focus_cursor(true);
            }
            else if key == "Right" {
                if app.keyboard.shift_held {
                    if self.cursor.selection_idx.is_none() {
                        self.cursor.selection_idx = Some(self.cursor.idx);
                    }
                    if !app.keyboard.ctrl_held {
                        self.cursor.idx = self.content.chars().count().min(self.cursor.idx + 1);
                    }
                    
                    for cursor in &mut self.cursors {
                        if cursor.selection_idx.is_none() {
                            cursor.selection_idx = Some(cursor.idx);
                        }
                        if !app.keyboard.ctrl_held {
                            cursor.idx = self.content.chars().count().min(cursor.idx + 1);
                        }
                    }
                    
                    if app.keyboard.ctrl_held {
                        self.ctrl_move(false);
                    }
                    

                } else {
                    if app.keyboard.ctrl_held {
                        self.ctrl_move(false)
                    } else {
                        self.deselect_all_directional(false, true);
                    }
                }
                self.truncate_cursors();
                self.set_cursor_preference();
                out = true;
                self.set_focus_cursor(true);
            }
            else if key == "Up" {
                if app.keyboard.shift_held {
                    if self.cursor.selection_idx.is_none() {
                        self.cursor.selection_idx = Some(self.cursor.idx);
                    }
                    for cursor in &mut self.cursors {
                        if cursor.selection_idx.is_none() {
                            cursor.selection_idx = Some(cursor.idx);
                        }
                    }
                    
                } else {
                    self.deselect_all_directional(false, true);
                }
                self.move_cursors(true);
                self.truncate_cursors();
                out = true;
                self.set_focus_cursor(true);
            }
            else if key == "Down" {
                if app.keyboard.shift_held {
                    if self.cursor.selection_idx.is_none() {
                        self.cursor.selection_idx = Some(self.cursor.idx);
                    }
                    for cursor in &mut self.cursors {
                        if cursor.selection_idx.is_none() {
                            cursor.selection_idx = Some(cursor.idx);
                        }
                    }

                } else {
                    self.deselect_all_directional(false, false);
                }
                self.move_cursors(false);
                self.truncate_cursors();
                out = true;
                self.set_focus_cursor(true);
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
        if self.content.chars().count() == 0 {
            return Some((0, 0))
        }
        if idx <= self.content.chars().count() {
            let mut line = 0;
            let mut dx = idx;
            let mut lines = self.content.split_inclusive('\n').peekable();
            println!("Searching for: {}", idx);
            while let Some(l) = lines.next() {
                if dx < l.chars().count() {
                    println!("Found {}, {}", line, dx);
                    return Some((line, dx));
                } else if dx == l.chars().count() && lines.peek().is_none() {
                    return if l.ends_with('\n') {
                        Some((line + 1, 0))
                    } else {
                        Some((line, dx.min(l.chars().count())))
                    }
                } else {
                    line += 1;
                    dx -= l.chars().count();
                }
            }
        }
        None
    }

    /// Inverse of `get_text_pos`
    pub fn get_index(&self, line: IdxSize, column: IdxSize) -> Option<IdxSize> {
        let mut idx = 0;
        let mut i = 0;
        for ln in self.content.split('\n') {
            if i == line {
                idx += column.min(ln.chars().count());
                return Some(idx + i)
            }
            idx += ln.chars().count();
            i += 1;
        }

        None
    }
    
    /// Removes additional cursors and deselects all text, moves the cursor to the specified position (clamped to the length of the text)
    pub fn set_cursor_index(&mut self, idx: IdxSize) {
        self.cursors.clear();
        self.cursor.idx = idx.min(self.content.chars().count());
        self.cursor.selection_idx = None;

    }

    pub fn mod_char(&self, app: &App, c: &str) -> String {
        if app.keyboard.shift_held {
            {
                if c == "`" {"~"}
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

        let mod_c = &self.mod_char(app, &content);
        
        cursors.push(&mut self.cursor.idx);

        for idx in &mut self.cursors {
            if !cursors.contains(&&mut idx.idx) {
                cursors.push(&mut idx.idx);
            }
        }
        cursors.sort();

        for cursor in &mut cursors {
            // self.content = self.content[0..cursor].to_string() + &self.mod_char(app, &content) + &self.content[cursor..];
            self.content = self.content.chars().take(**cursor - offset).collect::<String>()
                + mod_c
                + &self.content.chars().skip(**cursor - offset).collect::<String>();
            
            **cursor += content.chars().count();
            offset += content.chars().count();
        }
    }
    
    pub fn tab_at_cursor(&mut self, app: &App) {
        self.collapse_selections();
        
        let mut offset = 0;
        let mut cursors = Vec::new();
        
        let mut curs = Cursor::new(0);
        
        mem::swap(&mut curs, &mut self.cursor);
        let mut crs = mem::take(&mut self.cursors);
        
        cursors.push(&mut curs.idx);
        for idx in &mut crs {
            if !cursors.contains(&&mut idx.idx) {
                cursors.push(&mut idx.idx);
            }
        }
        cursors.sort();
        
        for cursor in &mut cursors {
            let (_, c) = self.get_text_pos(**cursor).unwrap();
            let spaces = (3 - (((c as isize % 4) - 1) % 4)) as usize;
            self.content = self.content.chars().take(**cursor - offset).collect::<String>()
                + " ".repeat(spaces).as_str()
                + &self.content.chars().skip(**cursor - offset).collect::<String>();
            **cursor += spaces;
            offset += spaces;
        }
        
        self.cursors = crs;
        self.cursor = curs;
        
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

        let mut offset = 0;
        let r = self.cursor.get_range();
        regions.push(r);

        offset += r.1-r.0;
        self.cursor.idx = r.0;
        self.cursor.selection_idx = None;

        for cursor in &mut self.cursors {
            let r= cursor.get_range();
            regions.push(r);
            offset += r.1-r.0;
            cursor.idx = r.0 - offset;
            cursor.selection_idx = None;
        }

        regions.sort();
        regions.dedup();

        offset = 0;

        TextInputHandler::merge_groups(&mut regions);

        for region in regions {
            self.content = self.content.chars().take(region.0 - offset).collect::<String>()
                + &self.content.chars().skip(region.1 - offset).collect::<String>();

            offset += region.1-region.0;
        }

    }

    /// does a backspace, accounting for selected text and multiple cursors
    pub fn backspace_at_cursor(&mut self) {


        let mut regions: Vec<(IdxSize, IdxSize)> = Vec::new();

        let mut offset = 0;
        let r = self.cursor.get_backspace_range();
        regions.push(r);
        offset += r.1-r.0;
        self.cursor.idx = r.0;
        self.cursor.selection_idx = None;

        for cursor in &mut self.cursors {
            let r = cursor.get_backspace_range();
            regions.push(r);
            offset += r.1-r.0;
            cursor.idx = r.0 - offset;
            cursor.selection_idx = None;
        }

        regions.sort();
        regions.dedup();

        offset = 0;

        TextInputHandler::merge_groups(&mut regions);


        for region in regions {
            
            if self.content.len() > 0 {
                if let Ok(c) = char::from_str(&self.content.chars().skip(region.0 - offset).take(1).collect::<String>()[0..1]) {
                    self.set_typing_flags(c);
                }
            }
            self.content = self.content.chars().take(region.0 - offset).collect::<String>()
                + &self.content.chars().skip(region.1 - offset).collect::<String>();
            offset += region.1-region.0;
        }

    }

    /// same as backspace_at_cursor, but with delete behavior
    pub fn delete_at_cursor(&mut self) {

        let mut regions: Vec<(IdxSize, IdxSize)> = Vec::new();
        let mut offset = 0;

        let r = self.cursor.get_delete_range(self.content.chars().count());
        regions.push(r);
        offset += r.1-r.0;
        self.cursor.selection_idx = None;

        for cursor in &mut self.cursors {
            regions.push(cursor.get_delete_range(self.content.chars().count()));
            cursor.idx = r.0 - offset;
            offset += r.1-r.0;
            cursor.selection_idx = None;
        }

        regions.sort();
        regions.dedup();


        TextInputHandler::merge_groups(&mut regions);

        offset = 0;

        for region in regions {
            self.content = self.content.chars().take(region.0 - offset).collect::<String>()
                + &self.content.chars().skip(region.1 - offset).collect::<String>();
            offset += region.1-region.0;
        }

    }

}


#[cfg(test)]
mod handler_tests {
    use crate::text_input_handler::{Cursor, TextInputHandler};

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

        handler.cursor.idx = 14;

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #1");


    }

    #[test]
    pub fn test_backspace_2_cursors() {
        let mut handler: TextInputHandler = TextInputHandler::new("This is tes t ##2".to_string(), true, None, true);
        //                                                                    ^  ^

        handler.cursor.idx = 12;
        handler.cursors.push(Cursor::new(15));

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #2");

    }

    #[test]
    pub fn test_backspace_selection_edge() {

        let mut handler: TextInputHandler = TextInputHandler::new("This is tes   t #3".to_string(), true, None, true);
        //                                                                    ^~^

        handler.cursor.idx = 14;
        handler.cursors.push(Cursor::selection(11, 14));

        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #3");
    }

    #[test]
    pub fn test_backspace_selection_2_cursors() {
        let mut handler: TextInputHandler = TextInputHandler::new("This is tes      t #4".to_string(), true, None, true);
        //                                                                    ^~~~~^

        handler.cursor.idx = 17;
        handler.cursors.push(Cursor::new(14));

        handler.cursors.push(Cursor::selection(11, 17));


        handler.backspace_at_cursor();

        assert_eq!(handler.content, "This is test #4");
    }

    
}

