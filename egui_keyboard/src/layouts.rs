use super::Key;

pub enum KeyboardLayout {
    Qwerty,
    Colemak,
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        Self::Qwerty
    }
}

impl KeyboardLayout {
    pub(crate) fn get_layout(&self, uppercase: bool) -> Vec<Vec<Key>> {
        match (self,uppercase) {
            (KeyboardLayout::Qwerty, false) => { qwerty() }
            (KeyboardLayout::Qwerty, true) => { qwerty_upper() }
            (KeyboardLayout::Colemak, false) => { colemak() }
            (KeyboardLayout::Colemak, true) => { colemak_upper() }
        }
    }
}

pub(crate) fn qwerty() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::Text("1"),
            Key::Text("2"),
            Key::Text("3"),
            Key::Text("4"),
            Key::Text("5"),
            Key::Text("6"),
            Key::Text("7"),
            Key::Text("8"),
            Key::Text("9"),
            Key::Text("0"),
        ],
        vec![
            Key::Text("q"),
            Key::Text("w"),
            Key::Text("e"),
            Key::Text("r"),
            Key::Text("t"),
            Key::Text("y"),
            Key::Text("u"),
            Key::Text("i"),
            Key::Text("o"),
            Key::Text("p"),
        ],
        vec![
            Key::Text("a"),
            Key::Text("s"),
            Key::Text("d"),
            Key::Text("f"),
            Key::Text("g"),
            Key::Text("h"),
            Key::Text("j"),
            Key::Text("k"),
            Key::Text("l"),
            Key::Text(":"),
        ],
        vec![
            Key::Upper,
            Key::Text("z"),
            Key::Text("x"),
            Key::Text("c"),
            Key::Text("v"),
            Key::Text("b"),
            Key::Text("n"),
            Key::Text("m"),
            Key::Text(","),
            Key::Text("."),
            Key::Text("/"),
            Key::Backspace,
        ],
        vec![Key::Text(" ")],
    ]
}

pub(crate) fn qwerty_upper() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::Text("!"),
            Key::Text("@"),
            Key::Text("#"),
            Key::Text("$"),
            Key::Text("%"),
            Key::Text("^"),
            Key::Text("&"),
            Key::Text("*"),
            Key::Text("("),
            Key::Text(")"),
        ],
        vec![
            Key::Text("Q"),
            Key::Text("W"),
            Key::Text("E"),
            Key::Text("R"),
            Key::Text("T"),
            Key::Text("Y"),
            Key::Text("U"),
            Key::Text("I"),
            Key::Text("O"),
            Key::Text("P"),
        ],
        vec![
            Key::Text("A"),
            Key::Text("S"),
            Key::Text("D"),
            Key::Text("F"),
            Key::Text("G"),
            Key::Text("H"),
            Key::Text("J"),
            Key::Text("K"),
            Key::Text("L"),
            Key::Text(";"),
        ],
        vec![
            Key::Upper,
            Key::Text("Z"),
            Key::Text("X"),
            Key::Text("C"),
            Key::Text("V"),
            Key::Text("B"),
            Key::Text("N"),
            Key::Text("M"),
            Key::Text("<"),
            Key::Text(">"),
            Key::Text("?"),
            Key::Backspace,
        ],
        vec![Key::Text(" ")],
    ]
}

pub(crate) fn colemak() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::Text("1"),
            Key::Text("2"),
            Key::Text("3"),
            Key::Text("4"),
            Key::Text("5"),
            Key::Text("6"),
            Key::Text("7"),
            Key::Text("8"),
            Key::Text("9"),
            Key::Text("0"),
        ],
        vec![
            Key::Text("q"),
            Key::Text("w"),
            Key::Text("f"),
            Key::Text("p"),
            Key::Text("g"),
            Key::Text("j"),
            Key::Text("l"),
            Key::Text("u"),
            Key::Text("y"),
            Key::Text(";"),
        ],
        vec![
            Key::Text("a"),
            Key::Text("r"),
            Key::Text("s"),
            Key::Text("t"),
            Key::Text("d"),
            Key::Text("h"),
            Key::Text("n"),
            Key::Text("e"),
            Key::Text("i"),
            Key::Text("o"),
        ],
        vec![
            Key::Upper,
            Key::Text("z"),
            Key::Text("x"),
            Key::Text("c"),
            Key::Text("v"),
            Key::Text("b"),
            Key::Text("k"),
            Key::Text("m"),
            Key::Text(","),
            Key::Text("."),
            Key::Text("/"),
            Key::Backspace,
        ],
        vec![Key::Text(" ")],
    ]
}

pub(crate) fn colemak_upper() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::Text("!"),
            Key::Text("@"),
            Key::Text("#"),
            Key::Text("$"),
            Key::Text("%"),
            Key::Text("^"),
            Key::Text("&"),
            Key::Text("*"),
            Key::Text("("),
            Key::Text(")"),
        ],
        vec![
            Key::Text("Q"),
            Key::Text("W"),
            Key::Text("F"),
            Key::Text("P"),
            Key::Text("G"),
            Key::Text("J"),
            Key::Text("L"),
            Key::Text("U"),
            Key::Text("Y"),
            Key::Text(";"),
        ],
        vec![
            Key::Text("A"),
            Key::Text("R"),
            Key::Text("S"),
            Key::Text("T"),
            Key::Text("D"),
            Key::Text("H"),
            Key::Text("N"),
            Key::Text("E"),
            Key::Text("I"),
            Key::Text("O"),
        ],
        vec![
            Key::Upper,
            Key::Text("Z"),
            Key::Text("X"),
            Key::Text("C"),
            Key::Text("V"),
            Key::Text("B"),
            Key::Text("K"),
            Key::Text("M"),
            Key::Text("<"),
            Key::Text(">"),
            Key::Text("?"),
            Key::Backspace,
        ],
        vec![Key::Text(" ")],
    ]
}
