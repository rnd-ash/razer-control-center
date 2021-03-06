use crate::effects::EffectLayer;

use super::effects;

pub const DEFAULT_KEY_MATRIX: effects::EffectLayer<15, 6> = EffectLayer::create_blank([[true; 15]; 6]);

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keys {
    /// Special case for keys that have no lighting
    KEY_BLANK,
    KEY_ESC,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_F11,
    KEY_F12,
    KEY_INS,
    KEY_DEL,
    KEY_BACKTICK,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_10,
    KEY_MINUS,
    KEY_PLUS,
    KEY_BACKSPACE,
    KEY_TAB,
    KEY_Q,
    KEY_W,
    KEY_E,
    KEY_R,
    KEY_T,
    KEY_Y,
    KEY_U,
    KEY_I,
    KEY_O,
    KEY_P,
    KEY_BRACKET_OPEN,
    KEY_BRACKET_CLOSE,
    KEY_ENTER,
    CAPS,
    KEY_A,
    KEY_S,
    KEY_D,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_SEMI_COLON,
    KEY_APOSTROPHE,
    KEY_HASH,
    KEY_L_SHIFT,
    KEY_R_SHIFT,
    KEY_BACKSLASH,
    KEY_Z,
    KEY_X,
    KEY_C,
    KEY_V,
    KEY_B,
    KEY_N,
    KEY_M,
    KEY_COMMA,
    KEY_PERIOD,
    KEY_QUESTION,
    KEY_ARROW_UP,
    KEY_ARROW_DOWN,
    KEY_ARROW_LEFT,
    KEY_ARROW_RIGHT,
    KEY_FN_LEFT,
    KEY_CTRL_LEFT,
    KEY_WINDOWS,
    KEY_ALT_LEFT,
    KEY_SPACE,
    KEY_ALT_RIGHT,
    KEY_CTRL_RIGHT,
    KEY_FN_RIGHT,
    KEY_CONTEXT_MENU,
    KEY_END,
    KEY_PG_UP,
    KEY_PG_DOWN,
    KEY_HOME,
    KEY_SCROLL_LOCK,
    KEY_PRT_SC,
    KEY_PAUSE_BREAK,
}


// Keyboard matrix size (x, y)
