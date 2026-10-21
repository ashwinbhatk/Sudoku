use slint::{ComponentHandle, Model, ModelRc, Timer, TimerMode, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::{generate_puzzle, Difficulty};
use crate::history::{HistoryAction, HistoryManager};
use crate::solver::{find_conflicts, get_candidates_bitmask, get_hint, is_valid_puzzle, solver};

slint::include_modules!();

struct AppState {
    cells_model: Rc<VecModel<i32>>,
    given_model: Rc<VecModel<bool>>,
    conflict_model: Rc<VecModel<bool>>,
    same_num_model: Rc<VecModel<bool>>,
    crosshair_model: Rc<VecModel<bool>>,
    notes_model: Rc<VecModel<i32>>,
    initial_puzzle: [[u8; 9]; 9],
    history: HistoryManager,
    selected_cell: i32,
    pencil_mode: bool,
    dark_mode: bool,
    custom_mode: bool,
    difficulty: Difficulty,
    timer_seconds: u32,
    is_paused: bool,
}

impl AppState {
    fn update_highlights(&self) {
        if self.selected_cell < 0 || self.selected_cell >= 81 {
            for i in 0..81 {
                if self.crosshair_model.row_data(i).unwrap_or(false) {
                    self.crosshair_model.set_row_data(i, false);
                }
                if self.same_num_model.row_data(i).unwrap_or(false) {
                    self.same_num_model.set_row_data(i, false);
                }
            }
            return;
        }

        let sel = self.selected_cell as usize;
        let sel_val = self.cells_model.row_data(sel).unwrap_or(0);
        let sel_r = sel / 9;
        let sel_c = sel % 9;
        let sel_b = (sel_r / 3) * 3 + (sel_c / 3);

        for i in 0..81 {
            let r = i / 9;
            let c = i % 9;
            let b = (r / 3) * 3 + (c / 3);

            let is_cross = (r == sel_r || c == sel_c || b == sel_b) && i != sel;
            if self.crosshair_model.row_data(i).unwrap_or(false) != is_cross {
                self.crosshair_model.set_row_data(i, is_cross);
            }

            let val = self.cells_model.row_data(i).unwrap_or(0);
            let is_same = sel_val > 0 && val == sel_val && i != sel;
            if self.same_num_model.row_data(i).unwrap_or(false) != is_same {
                self.same_num_model.set_row_data(i, is_same);
            }
        }
    }

    fn read_grid(&self) -> [[u8; 9]; 9] {
        let mut grid = [[0u8; 9]; 9];
        for i in 0..81 {
            grid[i / 9][i % 9] = self.cells_model.row_data(i).unwrap_or(0) as u8;
        }
        grid
    }

    fn update_conflicts(&self) {
        let grid = self.read_grid();
        let conflicts = find_conflicts(&grid);
        for i in 0..81 {
            if self.conflict_model.row_data(i).unwrap_or(false) != conflicts[i] {
                self.conflict_model.set_row_data(i, conflicts[i]);
            }
        }
    }

    fn check_completion_with_grid(&self, grid: &[[u8; 9]; 9], window: &SudokuWindow) {
        let filled = grid.iter().flat_map(|r| r.iter()).filter(|&&v| v != 0).count();
        if filled == 81 && is_valid_puzzle(grid) {
            window.set_status_message("🎉 Congratulations! Puzzle completed successfully!".into());
            window.set_status_type(1);
        }
    }

    fn check_completion(&self, window: &SudokuWindow) {
        let grid = self.read_grid();
        self.check_completion_with_grid(&grid, window);
    }

    fn load_grid(&mut self, grid: [[u8; 9]; 9], window: &SudokuWindow) {
        self.initial_puzzle = grid;
        self.history.clear();
        window.set_can_undo(false);
        window.set_can_redo(false);
        self.selected_cell = -1;
        window.set_selected_cell(-1);

        for i in 0..81 {
            let val = grid[i / 9][i % 9] as i32;
            self.cells_model.set_row_data(i, val);
            self.given_model.set_row_data(i, val != 0);
            self.notes_model.set_row_data(i, 0);
            self.conflict_model.set_row_data(i, false);
            self.same_num_model.set_row_data(i, false);
            self.crosshair_model.set_row_data(i, false);
        }

        self.timer_seconds = 0;
        window.set_timer_text("00:00".into());
    }
}

pub fn run() -> Result<(), slint::PlatformError> {
    let window = SudokuWindow::new()?;

    // Initial default easy puzzle
    let initial_grid = generate_puzzle(Difficulty::Easy);

    let cells_model = Rc::new(VecModel::from(vec![0i32; 81]));
    let given_model = Rc::new(VecModel::from(vec![false; 81]));
    let conflict_model = Rc::new(VecModel::from(vec![false; 81]));
    let same_num_model = Rc::new(VecModel::from(vec![false; 81]));
    let crosshair_model = Rc::new(VecModel::from(vec![false; 81]));
    let notes_model = Rc::new(VecModel::from(vec![0i32; 81]));

    window.set_cells(ModelRc::from(cells_model.clone()));
    window.set_given_cells(ModelRc::from(given_model.clone()));
    window.set_conflict_cells(ModelRc::from(conflict_model.clone()));
    window.set_same_num_cells(ModelRc::from(same_num_model.clone()));
    window.set_crosshair_cells(ModelRc::from(crosshair_model.clone()));
    window.set_cell_notes(ModelRc::from(notes_model.clone()));

    let state = Rc::new(RefCell::new(AppState {
        cells_model: cells_model.clone(),
        given_model: given_model.clone(),
        conflict_model: conflict_model.clone(),
        same_num_model: same_num_model.clone(),
        crosshair_model: crosshair_model.clone(),
        notes_model: notes_model.clone(),
        initial_puzzle: initial_grid,
        history: HistoryManager::new(),
        selected_cell: -1,
        pencil_mode: false,
        dark_mode: false,
        custom_mode: false,
        difficulty: Difficulty::Easy,
        timer_seconds: 0,
        is_paused: false,
    }));

    state.borrow_mut().load_grid(initial_grid, &window);

    // Timer loop: fires every 1 second
    let timer = Timer::default();
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(1),
        move || {
            if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
                let mut state = state_rc.borrow_mut();
                if !state.is_paused {
                    state.timer_seconds += 1;
                    let mins = state.timer_seconds / 60;
                    let secs = state.timer_seconds % 60;
                    window.set_timer_text(format!("{:02}:{:02}", mins, secs).into());
                }
            }
        },
    );

    // Cell clicked callback
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_cell_clicked(move |idx| {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.selected_cell = idx;
            window.set_selected_cell(idx);
            state.update_highlights();
        }
    });

    // Digit entered callback (1-9)
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_digit_entered(move |digit| {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            let idx = state.selected_cell;
            if idx < 0 || idx >= 81 {
                window.set_status_message("Select a cell on the board first.".into());
                window.set_status_type(0);
                return;
            }
            let uidx = idx as usize;

            // Check if immutable given cell (unless in custom setup mode)
            if state.given_model.row_data(uidx).unwrap_or(false) && !state.custom_mode {
                window.set_status_message("This clue is part of the initial puzzle and cannot be modified.".into());
                window.set_status_type(2);
                return;
            }

            if state.pencil_mode {
                // Toggle candidate note bit (1 << (digit - 1))
                let bit = 1 << (digit - 1);
                let current_mask = state.notes_model.row_data(uidx).unwrap_or(0);
                let new_mask = current_mask ^ bit;
                state.notes_model.set_row_data(uidx, new_mask);
            } else {
                let old_val = state.cells_model.row_data(uidx).unwrap_or(0);
                if old_val != digit {
                    state.history.push(HistoryAction::SetCell {
                        index: uidx,
                        old_val,
                        new_val: digit,
                    });
                    window.set_can_undo(state.history.can_undo());
                    window.set_can_redo(state.history.can_redo());
                }

                state.cells_model.set_row_data(uidx, digit);
                state.notes_model.set_row_data(uidx, 0); // Clear notes when number entered

                if !state.custom_mode {
                    let grid = state.read_grid();
                    let conflicts = find_conflicts(&grid);
                    for i in 0..81 {
                        if state.conflict_model.row_data(i).unwrap_or(false) != conflicts[i] {
                            state.conflict_model.set_row_data(i, conflicts[i]);
                        }
                    }
                    state.check_completion_with_grid(&grid, &window);
                }
                state.update_highlights();
            }
        }
    });

    // Erase clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_erase_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            let idx = state.selected_cell;
            if idx < 0 || idx >= 81 {
                return;
            }
            let uidx = idx as usize;

            if state.given_model.row_data(uidx).unwrap_or(false) && !state.custom_mode {
                window.set_status_message("Cannot erase given puzzle clues.".into());
                window.set_status_type(2);
                return;
            }

            let old_val = state.cells_model.row_data(uidx).unwrap_or(0);
            if old_val != 0 {
                state.history.push(HistoryAction::SetCell {
                    index: uidx,
                    old_val,
                    new_val: 0,
                });
                window.set_can_undo(state.history.can_undo());
                window.set_can_redo(state.history.can_redo());
            }

            state.cells_model.set_row_data(uidx, 0);
            state.notes_model.set_row_data(uidx, 0);
            let grid = state.read_grid();
            let conflicts = find_conflicts(&grid);
            for i in 0..81 {
                if state.conflict_model.row_data(i).unwrap_or(false) != conflicts[i] {
                    state.conflict_model.set_row_data(i, conflicts[i]);
                }
            }
            state.update_highlights();
        }
    });

    // Toggle pencil/note mode
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_toggle_pencil_mode(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.pencil_mode = !state.pencil_mode;
            window.set_pencil_mode(state.pencil_mode);
            if state.pencil_mode {
                window.set_status_message("Pencil Note Mode active: type digits to toggle notes.".into());
            } else {
                window.set_status_message("Pen Mode active: type digits to fill cells.".into());
            }
            window.set_status_type(0);
        }
    });

    // Hint clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_hint_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            let mut grid = [[0u8; 9]; 9];
            for i in 0..81 {
                grid[i / 9][i % 9] = state.cells_model.row_data(i).unwrap_or(0) as u8;
            }

            match get_hint(&grid) {
                Some((r, c, val, explanation)) => {
                    let idx = r * 9 + c;
                    state.selected_cell = idx as i32;
                    window.set_selected_cell(idx as i32);
                    state.cells_model.set_row_data(idx, val as i32);
                    state.notes_model.set_row_data(idx, 0);
                    state.update_conflicts();
                    state.update_highlights();
                    state.check_completion(&window);

                    window.set_status_message(format!("💡 Cell ({}, {}): {}", r + 1, c + 1, explanation).into());
                    window.set_status_type(3);
                }
                None => {
                    window.set_status_message("No hints available. Check if current entries contain conflicts.".into());
                    window.set_status_type(2);
                }
            }
        }
    });

    // Auto notes clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_auto_notes_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let state = state_rc.borrow();
            let mut grid = [[0u8; 9]; 9];
            for i in 0..81 {
                grid[i / 9][i % 9] = state.cells_model.row_data(i).unwrap_or(0) as u8;
            }

            for r in 0..9 {
                for c in 0..9 {
                    let idx = r * 9 + c;
                    if grid[r][c] == 0 {
                        let mask = get_candidates_bitmask(r, c, &grid);
                        state.notes_model.set_row_data(idx, mask as i32);
                    } else {
                        state.notes_model.set_row_data(idx, 0);
                    }
                }
            }

            window.set_status_message("Legal candidate notes calculated for all empty cells.".into());
            window.set_status_type(1);
        }
    });

    // Validate / Check clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_validate_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let state = state_rc.borrow();
            state.update_conflicts();

            let mut has_conflict = false;
            let mut empty_count = 0;
            for i in 0..81 {
                if state.conflict_model.row_data(i).unwrap_or(false) {
                    has_conflict = true;
                }
                if state.cells_model.row_data(i).unwrap_or(0) == 0 {
                    empty_count += 1;
                }
            }

            if has_conflict {
                window.set_status_message("Conflicts found! Check red highlighted cells.".into());
                window.set_status_type(2);
            } else if empty_count == 0 {
                window.set_status_message("🎉 Outstanding! The board is completely and correctly solved!".into());
                window.set_status_type(1);
            } else {
                window.set_status_message(format!("Looking good! No conflicts detected so far. ({} cells remaining)", empty_count).into());
                window.set_status_type(1);
            }
        }
    });

    // Undo clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_undo_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            if let Some(action) = state.history.undo() {
                match action {
                    HistoryAction::SetCell { index, old_val, .. } => {
                        state.cells_model.set_row_data(index, old_val);
                    }
                    _ => {}
                }
                state.update_conflicts();
                state.update_highlights();
                window.set_can_undo(state.history.can_undo());
                window.set_can_redo(state.history.can_redo());
            }
        }
    });

    // Redo clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_redo_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            if let Some(action) = state.history.redo() {
                match action {
                    HistoryAction::SetCell { index, new_val, .. } => {
                        state.cells_model.set_row_data(index, new_val);
                    }
                    _ => {}
                }
                state.update_conflicts();
                state.update_highlights();
                window.set_can_undo(state.history.can_undo());
                window.set_can_redo(state.history.can_redo());
            }
        }
    });

    // Solve clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_solve_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let state = state_rc.borrow();
            let mut grid = [[0u8; 9]; 9];
            for i in 0..81 {
                grid[i / 9][i % 9] = state.cells_model.row_data(i).unwrap_or(0) as u8;
            }

            if solver(&mut grid) {
                for i in 0..81 {
                    state.cells_model.set_row_data(i, grid[i / 9][i % 9] as i32);
                    state.notes_model.set_row_data(i, 0);
                    state.conflict_model.set_row_data(i, false);
                }
                state.update_highlights();
                window.set_status_message("Puzzle solved successfully!".into());
                window.set_status_type(1);
            } else {
                window.set_status_message("No valid solution exists for the current puzzle configuration.".into());
                window.set_status_type(2);
            }
        }
    });

    // Clear user entries
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_clear_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let state = state_rc.borrow();
            for i in 0..81 {
                if !state.given_model.row_data(i).unwrap_or(false) {
                    state.cells_model.set_row_data(i, 0);
                    state.notes_model.set_row_data(i, 0);
                    state.conflict_model.set_row_data(i, false);
                }
            }
            state.update_highlights();
            window.set_status_message("User entries and notes cleared. Given clues preserved.".into());
            window.set_status_type(0);
        }
    });

    // Reset game
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_reset_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            let initial = state.initial_puzzle;
            state.load_grid(initial, &window);
            window.set_status_message("Game reset to initial puzzle.".into());
            window.set_status_type(0);
        }
    });

    // New game clicked
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_new_game_clicked(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            let new_puzzle = generate_puzzle(state.difficulty);
            state.load_grid(new_puzzle, &window);
            window.set_status_message(format!("New {} puzzle generated! Enjoy playing.", state.difficulty.as_str()).into());
            window.set_status_type(1);
        }
    });

    // Difficulty changed
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_difficulty_changed(move |diff_idx| {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.difficulty = match diff_idx {
                1 => Difficulty::Medium,
                2 => Difficulty::Hard,
                3 => Difficulty::Expert,
                _ => Difficulty::Easy,
            };
            window.set_difficulty(diff_idx);
            let new_puzzle = generate_puzzle(state.difficulty);
            state.load_grid(new_puzzle, &window);
            window.set_status_message(format!("Difficulty switched to {}. New puzzle generated.", state.difficulty.as_str()).into());
            window.set_status_type(0);
        }
    });

    // Toggle theme (Dark / Light)
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_toggle_theme(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.dark_mode = !state.dark_mode;
            window.set_dark_mode(state.dark_mode);
        }
    });

    // Toggle custom mode ("Set Clues" / "Lock Clues")
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_toggle_custom_mode(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.custom_mode = !state.custom_mode;
            window.set_custom_mode(state.custom_mode);

            if state.custom_mode {
                // Clear given flags so user can place clues anywhere
                for i in 0..81 {
                    state.given_model.set_row_data(i, false);
                }
                window.set_status_message("Custom Setup Mode: Click cells and type numbers to set up your puzzle, then click 'Lock Clues'.".into());
                window.set_status_type(3);
            } else {
                // Lock in current non-zero numbers as given clues
                let mut grid = [[0u8; 9]; 9];
                for i in 0..81 {
                    let val = state.cells_model.row_data(i).unwrap_or(0);
                    grid[i / 9][i % 9] = val as u8;
                    state.given_model.set_row_data(i, val != 0);
                }
                state.initial_puzzle = grid;
                state.update_conflicts();
                window.set_status_message("Custom puzzle clues locked! You can now solve manually or click 'Solve'.".into());
                window.set_status_type(1);
            }
        }
    });

    // Toggle pause
    let state_weak = Rc::downgrade(&state);
    let window_weak = window.as_weak();
    window.on_toggle_pause(move || {
        if let (Some(state_rc), Some(window)) = (state_weak.upgrade(), window_weak.upgrade()) {
            let mut state = state_rc.borrow_mut();
            state.is_paused = !state.is_paused;
            window.set_is_paused(state.is_paused);
            if state.is_paused {
                window.set_status_message("Game paused. Click Play to resume.".into());
            } else {
                window.set_status_message("Game resumed.".into());
            }
        }
    });

    window.run()
}
