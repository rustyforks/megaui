use crate::types::Vector2;

pub use crate::input_handler::KeyCode;

#[derive(Clone, Debug)]
pub enum Key {
    Char(char),
    KeyCode(KeyCode),
}

#[derive(Clone, Debug)]
pub struct InputCharacter {
    pub key: Key,
    pub modifier_shift: bool,
    pub modifier_ctrl: bool,
}

#[derive(Default, Clone)]
pub struct Input {
    pub(crate) mouse_position: Vector2,
    pub(crate) is_mouse_down: bool,
    pub(crate) click_down: bool,
    pub(crate) click_up: bool,
    pub(crate) mouse_wheel: Vector2,
    pub(crate) input_buffer: Vec<InputCharacter>,
    pub(crate) modifier_ctrl: bool,
    pub(crate) escape: bool,
    pub(crate) enter: bool,
    pub(crate) cursor_grabbed: bool,
    // TODO: its a hack to prevent button click behind modal
    pub(crate) modal_active: bool,
    pub(crate) in_modal: bool,
}

impl Input {
    pub fn is_mouse_down(&self) -> bool {
        self.is_mouse_down
            && self.cursor_grabbed == false
            && (self.in_modal || self.modal_active == false)
    }

    pub fn click_down(&self) -> bool {
        self.click_down
            && self.cursor_grabbed == false
            && (self.in_modal || self.modal_active == false)
    }

    pub fn click_up(&self) -> bool {
        self.click_up
            && self.cursor_grabbed == false
            && (self.in_modal || self.modal_active == false)
    }

    pub fn reset(&mut self) {
        self.modifier_ctrl = false;
        self.escape = false;
        self.enter = false;
        self.click_down = false;
        self.click_up = false;
        self.modal_active = false;
        self.mouse_wheel = Vector2::new(0., 0.);
        self.input_buffer = vec![];
    }
}
