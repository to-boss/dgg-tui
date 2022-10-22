use std::ops::Range;

#[derive(PartialEq, Eq)]
pub struct Window {
    pub window_type: WindowType,
    pub active: bool,
    pub auto_scroll: bool,
    pub scroll: i16,
}

impl Window {
    pub fn new(window_type: WindowType, active: bool) -> Self {
        Window {
            window_type,
            active,
            auto_scroll: true,
            scroll: 0,
        }
    }

    pub fn compute_viewport(&mut self, height: usize, list_len: usize) -> Range<usize> {
        let mut start: i16 = 0;
        let mut end = list_len as i16;

        if self.auto_scroll {
            if list_len > height {
                self.scroll = (list_len - height) as i16;
                start = self.scroll;
            }
        } else {
            // handle start
            if start + self.scroll >= 0 {
                start += self.scroll;
            }
            // handle end
            if end + self.scroll < list_len as i16 {
                end += self.scroll;
            }
        }

        start as usize..end as usize
    }

    pub fn scroll(&mut self, val: i16) {
        if self.scroll + val >= 0 {
            self.scroll += val;
        }
        self.auto_scroll = self.scroll < 1;
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
    pub fn new() -> WindowList {
        WindowList {
            windows: vec![
                Window::new(WindowType::Chat, true),
                Window::new(WindowType::ChatInput, true),
                Window::new(WindowType::Debug, false),
                Window::new(WindowType::UserList, false),
            ],
        }
    }
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
