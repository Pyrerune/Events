
pub mod events {
    use tetra::input::Key;
    use tetra::{input, Context};

    pub struct KeyEvent {
        pub keycode: Key,
    }
    impl KeyEvent {
        //it ain't pretty but its all i've got right now
        pub fn handle(&mut self, ctx: &mut Context) {
            if input::is_key_down(ctx, Key::A) {
                self.keycode = Key::A;
            } else if input::is_key_down(ctx, Key::B) {
                self.keycode = Key::B;
            } else if input::is_key_down(ctx, Key::C) {
                self.keycode = Key::C;
            } else if input::is_key_down(ctx, Key::D) {
                self.keycode = Key::D;
            } else if input::is_key_down(ctx, Key::E) {
                self.keycode = Key::E;
            } else if input::is_key_down(ctx, Key::F) {
                self.keycode = Key::F;
            } else if input::is_key_down(ctx, Key::G) {
                self.keycode = Key::G;
            } else if input::is_key_down(ctx, Key::H) {
                self.keycode = Key::H;
            } else if input::is_key_down(ctx, Key::I) {
                self.keycode = Key::I;
            } else if input::is_key_down(ctx, Key::J) {
                self.keycode = Key::J;
            } else if input::is_key_down(ctx, Key::K) {
                self.keycode = Key::K;
            } else if input::is_key_down(ctx, Key::L) {
                self.keycode = Key::L;
            } else if input::is_key_down(ctx, Key::M) {
                self.keycode = Key::M;
            } else if input::is_key_down(ctx, Key::N) {
                self.keycode = Key::N;
            } else if input::is_key_down(ctx, Key::O) {
                self.keycode = Key::O;
            } else if input::is_key_down(ctx, Key::P) {
                self.keycode = Key::P;
            } else if input::is_key_down(ctx, Key::Q) {
                self.keycode = Key::Q;
            } else if input::is_key_down(ctx, Key::R) {
                self.keycode = Key::R;
            } else if input::is_key_down(ctx, Key::S) {
                self.keycode = Key::S;
            } else if input::is_key_down(ctx, Key::T) {
                self.keycode = Key::T;
            } else if input::is_key_down(ctx, Key::U) {
                self.keycode = Key::U;
            } else if input::is_key_down(ctx, Key::V) {
                self.keycode = Key::V;
            } else if input::is_key_down(ctx, Key::W) {
                self.keycode = Key::W;
            } else if input::is_key_down(ctx, Key::X) {
                self.keycode = Key::X;
            } else if input::is_key_down(ctx, Key::Y) {
                self.keycode = Key::Y;
            } else if input::is_key_down(ctx, Key::Z) {
                self.keycode = Key::Z;
            } else if input::is_key_down(ctx, Key::Num0) {
                self.keycode = Key::Num0;
            } else if input::is_key_down(ctx, Key::Num1) {
                self.keycode = Key::Num1;
            } else if input::is_key_down(ctx, Key::Num2) {
                self.keycode = Key::Num2;
            } else if input::is_key_down(ctx, Key::Num3) {
                self.keycode = Key::Num3;
            } else if input::is_key_down(ctx, Key::Num4) {
                self.keycode = Key::Num4;
            } else if input::is_key_down(ctx, Key::Num5) {
                self.keycode = Key::Num5;
            } else if input::is_key_down(ctx, Key::Num6) {
                self.keycode = Key::Num6;
            } else if input::is_key_down(ctx, Key::Num7) {
                self.keycode = Key::Num7;
            } else if input::is_key_down(ctx, Key::Num8) {
                self.keycode = Key::Num8;
            } else if input::is_key_down(ctx, Key::Num9) {
                self.keycode = Key::Num9;
            } else if input::is_key_down(ctx, Key::Minus) {
                self.keycode = Key::Minus;
            } else if input::is_key_down(ctx, Key::Equals) {
                self.keycode = Key::Equals;
            } else if input::is_key_down(ctx, Key::LeftBracket) {
                self.keycode = Key::LeftBracket;
            } else if input::is_key_down(ctx, Key::RightBracket) {
                self.keycode = Key::RightBracket;
            } else if input::is_key_down(ctx, Key::Backslash) {
                self.keycode = Key::Backslash;
            } else if input::is_key_down(ctx, Key::Semicolon) {
                self.keycode = Key::Semicolon;
            } else if input::is_key_down(ctx, Key::Quote) {
                self.keycode = Key::Quote;
            } else if input::is_key_down(ctx, Key::Period) {
                self.keycode = Key::Period;
            } else if input::is_key_down(ctx, Key::Comma) {
                self.keycode = Key::Comma;
            } else if input::is_key_down(ctx, Key::Slash) {
                self.keycode = Key::Slash;
            } else if input::is_key_down(ctx, Key::LShift) {
                self.keycode = Key::LShift;
            } else if input::is_key_down(ctx, Key::LCtrl) {
                self.keycode = Key::LCtrl;
            } else if input::is_key_down(ctx, Key::LAlt) {
                self.keycode = Key::LAlt;
            } else if input::is_key_down(ctx, Key::RShift) {
                self.keycode = Key::RShift;
            } else if input::is_key_down(ctx, Key::RAlt) {
                self.keycode = Key::RAlt;
            } else if input::is_key_down(ctx, Key::RCtrl) {
                self.keycode = Key::RCtrl;
            } else if input::is_key_down(ctx, Key::Left) {
                self.keycode = Key::Left;
            } else if input::is_key_down(ctx, Key::Right) {
                self.keycode = Key::Right;
            } else if input::is_key_down(ctx, Key::Up) {
                self.keycode = Key::Up;
            } else if input::is_key_down(ctx, Key::Down) {
                self.keycode = Key::Down;
            } else if input::is_key_down(ctx, Key::Escape) {
                self.keycode = Key::Escape;
            } else if input::is_key_down(ctx, Key::Space) {
                self.keycode = Key::Space;
            } else {
                self.keycode = Key::Equals;
            }
        }
        pub fn new() -> tetra::Result<KeyEvent> {
            Ok(KeyEvent {
                keycode: Key::Equals,
            })
        }
    }
}