use sudoku::history::{HistoryAction, HistoryManager};

#[test]
fn test_history_undo_redo() {
    let mut history = HistoryManager::new();
    assert!(!history.can_undo());
    assert!(!history.can_redo());

    history.push(HistoryAction::SetCell {
        index: 10,
        old_val: 0,
        new_val: 5,
    });
    assert!(history.can_undo());
    assert!(!history.can_redo());

    let action = history.undo();
    assert_eq!(
        action,
        Some(HistoryAction::SetCell {
            index: 10,
            old_val: 0,
            new_val: 5,
        })
    );
    assert!(!history.can_undo());
    assert!(history.can_redo());

    let redone = history.redo();
    assert_eq!(
        redone,
        Some(HistoryAction::SetCell {
            index: 10,
            old_val: 0,
            new_val: 5,
        })
    );
    assert!(history.can_undo());
    assert!(!history.can_redo());
}
