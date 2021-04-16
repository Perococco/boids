use raylib::drawing::RaylibDrawHandle;

pub struct MouseState {
    pub left: MouseButtonState,
    pub middle: MouseButtonState,
    pub right: MouseButtonState,
}

pub struct MouseButtonState {
    pub down: bool,
    pub up: bool,
    pub pressed: bool,
    pub released: bool,
}

impl MouseState {
    pub fn new(d: &RaylibDrawHandle) -> Self {
        Self {
            right: MouseButtonState::new(d, raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON),
            middle: MouseButtonState::new(d, raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON),
            left: MouseButtonState::new(d, raylib::consts::MouseButton::MOUSE_LEFT_BUTTON),
        }
    }
}

impl MouseButtonState {
    pub fn new(d: &RaylibDrawHandle, button: raylib::consts::MouseButton) -> Self {
        Self {
            down: d.is_mouse_button_down(button),
            up: d.is_mouse_button_up(button),
            pressed: d.is_mouse_button_pressed(button),
            released: d.is_mouse_button_released(button),
        }
    }
}