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
    Copy,
    Paste,
    Save,
    Search,
    Quit,
    Escape,
    PrevTab,
    NextTab,
    None,
}

pub fn map_key_event(event: Event, key_config: &crate::settings::KeybindingsSettings) -> Command {
    if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event {
        if kind != KeyEventKind::Press {
            return Command::None;
        }
        
        let combo = crate::settings::KeyCombination { code, modifiers };
        
        if combo == key_config.prev_tab { return Command::PrevTab; }
        if combo == key_config.next_tab { return Command::NextTab; }
        if combo == key_config.save { return Command::Save; }
        if combo == key_config.quit { return Command::Quit; }
        if combo == key_config.search { return Command::Search; }
        if combo == key_config.select_all { return Command::SelectAll; }
        if combo == key_config.undo { return Command::Undo; }
        if combo == key_config.redo { return Command::Redo; }
        if combo == key_config.copy { return Command::Copy; }
        if combo == key_config.paste { return Command::Paste; }

        let shift = modifiers.contains(KeyModifiers::SHIFT);
        match code {
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
