
pub mod events {
    use tetra::input::Key::{self, *};
    use tetra::{input, Context};

    pub struct KeyEvent {
        pub keycode: Key,
    }
    impl KeyEvent {
        //it ain't pretty but its all i've got right now
        pub fn handle(&mut self, ctx: &mut Context) {
            let keys = vec![
                A, B, C, D, E,
                F, G, H, I, J,
                K, L, M, N, O,
                P, Q, R, S, T,
                U, V, W, X, Y,
                Z,
                Num0, Num1, Num2,
                Num3, Num4, Num5,
                Num6, Num7, Num8,
                Num9,
                F1, F2, F3, F4,
                F5, F6, F7, F8,
                F9, F10, F11,
                F12, F13, F14,
                F15, F16, F17,
                F18, F19, F20,
                F21, F22, F23,
                F24,
                NumLock, NumPad1,
                NumPad2, NumPad3,
                NumPad4, NumPad5,
                NumPad6, NumPad7,
                NumPad8, NumPad9,
                NumPad0,
                NumPadPlus, NumPadMinus,
                NumPadMultiply, NumPadDivide,
                NumPadEnter, LeftCtrl, LeftShift,
                LeftAlt, RightCtrl, RightShift,
                RightAlt, Up, Down, Left, Right,
                Ampersand, Asterisk, At, Backquote,
                Backslash, Backspace, CapsLock,
                Caret, Colon, Comma, Delete, Dollar,
                DoubleQuote, End, Enter, Equals,
                Escape, Exclaim, GreaterThan, Hash,
                Home, Insert, LeftBracket, LeftParen,
                LessThan, Minus, PageDown, PageUp,
                Pause, Percent, Period, Plus, PrintScreen,
                Question, Quote, RightBracket, RightParen,
                ScrollLock, Semicolon, Slash, Space, Tab,
                Underscore, ];

            for i in keys {
                if input::is_key_down(ctx, i) {
                    self.keycode = i;
                }
                if input::is_key_released(ctx, i) {
                    self.keycode = PrintScreen;
                }
            }
        }
        pub fn new() -> tetra::Result<KeyEvent> {
            Ok(KeyEvent {
                keycode: Key::Equals,
            })
        }
    }
}