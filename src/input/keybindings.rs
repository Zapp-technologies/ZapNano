use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

pub enum Command {
    InsertChar(char),
    InsertNewline,
    InsertTab,
    DeleteBackwards,
    DeleteForward,
    MoveUp(bool),
    MoveDown(bool),
    MoveLeft(bool),
    MoveRight(bool),
    SelectAll,
    Undo,
    Redo,
    Save,
    Search,
    Quit,
    Escape,
    PrevTab,
    NextTab,
    None,
}

pub fn map_key_event(event: Event) -> Command {
    if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event {
        if kind != KeyEventKind::Press {
            return Command::None;
        }
        
        let shift = modifiers.contains(KeyModifiers::SHIFT);
        match code {
            KeyCode::Left if modifiers.contains(KeyModifiers::ALT) => Command::PrevTab,
            KeyCode::Right if modifiers.contains(KeyModifiers::ALT) => Command::NextTab,
            KeyCode::Char('f') if modifiers.contains(KeyModifiers::CONTROL) => Command::Search,
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => Command::Save,
            KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => Command::Quit,
            KeyCode::Char('a') if modifiers.contains(KeyModifiers::CONTROL) => Command::SelectAll,
            KeyCode::Char('z') if modifiers.contains(KeyModifiers::CONTROL) => Command::Undo,
            KeyCode::Char('y') if modifiers.contains(KeyModifiers::CONTROL) => Command::Redo,
            KeyCode::Char(c) if !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT) => Command::InsertChar(c),
            KeyCode::Enter => Command::InsertNewline,
            KeyCode::Tab => Command::InsertTab,
            KeyCode::Backspace => Command::DeleteBackwards,
            KeyCode::Delete => Command::DeleteForward,
            KeyCode::Up => Command::MoveUp(shift),
            KeyCode::Down => Command::MoveDown(shift),
            KeyCode::Left => Command::MoveLeft(shift),
            KeyCode::Right => Command::MoveRight(shift),
            KeyCode::Esc => Command::Escape,
            _ => Command::None,
        }
    } else {
        Command::None
    }
}
