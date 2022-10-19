#[derive(PartialEq, Eq)]
pub struct Window {
    pub window_type: WindowType,
    pub active: bool,
    pub auto_scroll: bool,
    pub max_displays: u16,
    pub scroll: u16,
}

impl Window {
    pub fn new(window_type: WindowType, active: bool, max_displays: u16) -> Self {
        Window {
            window_type,
            active,
            auto_scroll: false,
            max_displays,
            scroll: 0,
        }
    }

    pub fn scroll_to_bottom(&self) -> (u16, u16) {
        (self.max_displays - 2, 0)
    }

    pub fn scroll(&mut self, val: i16) {
        let mut scroll = self.scroll as i16;
        if scroll + val >= 0 {
            scroll += val;
        }
        self.scroll = scroll as u16;
    }

    pub fn flip(&mut self) {
        self.active = !self.active;
    }
}

#[derive(PartialEq, Eq)]
pub enum WindowType {
    Chat,
    ChatInput,
    Debug,
    UserList,
}

pub struct WindowList {
    pub windows: Vec<Window>,
}

impl WindowList {
    pub fn get(&self, window_type: WindowType) -> &Window {
        self.windows
            .iter()
            .find(|w| w.window_type == window_type)
            .unwrap()
    }

    pub fn get_mut(&mut self, window_type: WindowType) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|w| w.window_type == window_type)
            .unwrap()
    }
}
