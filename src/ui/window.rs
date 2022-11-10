use std::ops::Range;

#[derive(PartialEq, Eq, Debug)]
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
        let end = list_len;

        if list_len > height {
            if self.auto_scroll {
                self.scroll = (list_len - height) as i16;
            }
        } else {
            return 0 as usize..end as usize;
        }

        // Make sure user can't scroll too far down and enable auto_scroll
        // if at the very bottom
        if self.scroll > (list_len - height) as i16 {
            self.auto_scroll = true;
            self.scroll = (list_len - height) as i16;
        }

        self.scroll as usize..end
    }

    pub fn scroll(&mut self, val: i16) {
        self.auto_scroll = false;

        if self.scroll + val >= 0 {
            self.scroll += val;
        } else {
            self.scroll = 0;
        }
    }

    /// Flips the window.active state e.g. true => false and vice versa
    pub fn flip(&mut self) {
        self.active = !self.active;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum WindowType {
    Chat,
    ChatInput,
    Debug,
    UserList,
}

#[derive(Debug)]
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

    pub fn len(&self) -> usize {
        self.windows.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_get() {
        let windows = WindowList::new();
        let debug = windows.get(WindowType::Debug);
        let chat = windows.get(WindowType::Chat);
        let chat_input = windows.get(WindowType::ChatInput);
        assert_eq!(debug.window_type, WindowType::Debug);
        assert_eq!(chat.window_type, WindowType::Chat);
        assert_eq!(chat_input.window_type, WindowType::ChatInput);
    }

    #[test]
    fn window_get_mut() {
        let mut windows = WindowList::new();
        let debug = windows.get_mut(WindowType::Debug);
        assert_eq!(debug, &mut Window::new(WindowType::Debug, false));
    }
}
