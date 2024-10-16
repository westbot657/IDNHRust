use crate::app::App;

pub trait HistoryEvent {
    fn redo(&mut self, app: &mut App);
    fn undo(&mut self, app: &mut App);
}


pub struct HistoryManager {
    history: Vec<Box<dyn HistoryEvent>>,
    future: Vec<Box<dyn HistoryEvent>>,
}

pub trait WrapHistory {
    fn wrap(self) -> Box<dyn HistoryEvent>;
}

impl<T: HistoryEvent + 'static> WrapHistory for T {
    fn wrap(self) -> Box<dyn HistoryEvent> {
        Box::new(self)
    }
}

impl HistoryManager {
    
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            future: Vec::new()
        }
    }
    
    pub fn add_history(&mut self, hist: impl WrapHistory) {
        self.history.push(hist.wrap());
        self.future.clear();
    }
    
    pub fn undo(&mut self, app: &mut App) {
        if let Some(mut hist) = self.history.pop() {
            hist.undo(app);
            self.future.push(hist);
        }
    }
    pub fn redo(&mut self, app: &mut App) {
        if let Some(mut hist) = self.future.pop() {
            hist.redo(app);
            self.history.push(hist);
        }
    }
    
}
