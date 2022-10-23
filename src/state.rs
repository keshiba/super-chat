
#[derive(Clone)]
pub enum InputMode {
    Command,
    Edit
}

#[derive(Clone)]
pub struct AppData {
    pub messages: Vec<(String, String)>,
    pub input: String
}

#[derive(Clone)]
pub struct AppState {
    pub data: AppData,
    pub input_mode: InputMode,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            messages: vec![],
            input: String::from("")
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            data: AppData::default(),
            input_mode: InputMode::Command
        }
    }
}