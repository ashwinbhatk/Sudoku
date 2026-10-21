#[derive(Clone, Debug, PartialEq)]
pub enum HistoryAction {
    SetCell {
        index: usize,
        old_val: i32,
        new_val: i32,
    },
    ClearUserCells {
        cells: Vec<(usize, i32)>,
    },
    SolveAction {
        old_cells: Vec<i32>,
        new_cells: Vec<i32>,
    },
    LoadPuzzle {
        old_cells: Vec<i32>,
        old_given: Vec<bool>,
        new_cells: Vec<i32>,
        new_given: Vec<bool>,
    },
}

#[derive(Default)]
pub struct HistoryManager {
    undo_stack: Vec<HistoryAction>,
    redo_stack: Vec<HistoryAction>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, action: HistoryAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<HistoryAction> {
        let action = self.undo_stack.pop()?;
        self.redo_stack.push(action.clone());
        Some(action)
    }

    pub fn redo(&mut self) -> Option<HistoryAction> {
        let action = self.redo_stack.pop()?;
        self.undo_stack.push(action.clone());
        Some(action)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
