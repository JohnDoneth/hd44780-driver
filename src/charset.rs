use core::ops::{Deref, DerefMut};

pub trait Charset {
	fn code_from_utf8(&self, ch: char) -> Option<u8>;
}

pub trait CharsetWithFallback {
	fn code_from_utf8_with_fallback(&self, ch: char) -> u8;
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct Fallback<C: Charset, const FB: u8>(C);

pub type EmptyFallback<C> = Fallback<C, b' '>;
pub type QuestionFallback<C> = Fallback<C, b'?'>;

impl<C: Charset, const FB: u8> Fallback<C, FB> {
	pub const fn new(c: C) -> Self {
		Self(c)
	}

	pub fn into_inner(self) -> C {
		self.0
	}
}

impl<C: Charset, const FB: u8> Deref for Fallback<C, FB> {
	type Target = C;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<C: Charset, const FB: u8> DerefMut for Fallback<C, FB> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<C: Charset, const FB: u8> CharsetWithFallback for Fallback<C, FB> {
	fn code_from_utf8_with_fallback(&self, ch: char) -> u8 {
		self.0.code_from_utf8(ch).unwrap_or(FB)
	}
}

impl<T: CharsetWithFallback> Charset for T {
	fn code_from_utf8(&self, ch: char) -> Option<u8> {
		Some(self.code_from_utf8_with_fallback(ch))
	}
}

/// Symbols common to both A00 and A02 Charset.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct CharsetUniversal;

impl CharsetUniversal {
	pub const EMPTY_FALLBACK: EmptyFallback<Self> = Fallback(Self);
	pub const QUESTION_FALLBACK: QuestionFallback<Self> = Fallback(Self);
}

impl Charset for CharsetUniversal {
	fn code_from_utf8(&self, ch: char) -> Option<u8> {
		match ch {
			'\\' | '\x10'..='\x1f' => None,
			'\x00'..='\x7d' => Some(ch as u8),
			_ => None,
		}
	}
}

/// Japanese Standard Font Character Set.
//
/// For reference, see page 17 on [the Hitachi datasheet by Sparkfun](https://www.sparkfun.com/datasheets/LCD/HD44780.pdf).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct CharsetA00;

impl CharsetA00 {
	pub const EMPTY_FALLBACK: EmptyFallback<Self> = Fallback(Self);
	pub const QUESTION_FALLBACK: QuestionFallback<Self> = Fallback(Self);
}

impl Charset for CharsetA00 {
	fn code_from_utf8(&self, ch: char) -> Option<u8> {
		match ch {
			// Lower (ASCII)
			'¥' => Some(0x5C),
			'\\' => None,
			'\x10'..='\x1f' => None,
			'\x00'..='\x7d' => Some(ch as u8),
			'\u{2192}' => Some(0x7E), // →
			'\u{2190}' => Some(0x7F), // ←
			'\u{ff01}' => Some(b'!'), // ！ full-width exclamation mark
			'\u{ff1f}' => Some(b'?'), // ！ full-width exclamation mark
			// Upper (Japanese)
			// A0: Empty
			'\u{3002}' => Some(0xA1),              // 。 Kuten
			'\u{300c}' => Some(0xA2),              // 「 Quotation Marks
			'\u{300d}' => Some(0xA3),              // 」
			'\u{3001}' => Some(0xA4),              // 、 Tōten
			'\u{30fb}' => Some(0xA5),              // ・ Nakaguro
			'\u{30f2}' => Some(0xA6),              // ヲ Wo
			'\u{30a1}' => Some(0xA7),              // ァ A (small)
			'\u{30a3}' => Some(0xA8),              // ィ I (small)
			'\u{30a5}' => Some(0xA9),              // ゥ U (small)
			'\u{30a7}' => Some(0xAA),              // ェ E (small)
			'\u{30a9}' => Some(0xAB),              // ォ O (small)
			'\u{30e3}' => Some(0xAC),              // ャ Ya (small)
			'\u{30e5}' => Some(0xAD),              // ュ Yu (small)
			'\u{30e7}' => Some(0xAE),              // ョ Yo (small)
			'\u{30c3}' => Some(0xAF),              // ッ Tu (small)
			'\u{30fc}' => Some(0xB0),              // ー
			'\u{30a2}' => Some(0xB1),              // ア A
			'\u{30a4}' => Some(0xB2),              // イ I
			'\u{30a6}' => Some(0xB3),              // ウ U
			'\u{30a8}' => Some(0xB4),              // エ E
			'\u{30aa}' => Some(0xB5),              // オ O
			'\u{30ab}' => Some(0xB6),              // カ Ka
			'\u{30ad}' => Some(0xB7),              // キ Ki
			'\u{30af}' => Some(0xB8),              // ク Ku
			'\u{30b1}' => Some(0xB9),              // ケ Ke
			'\u{30b3}' => Some(0xBA),              // コ Ko
			'\u{30b5}' => Some(0xBB),              // サ Sa
			'\u{30b7}' => Some(0xBC),              // シ Si
			'\u{30b9}' => Some(0xBD),              // ス Su
			'\u{30bb}' => Some(0xBE),              // セ Se
			'\u{30bd}' => Some(0xBF),              // ソ So
			'\u{30bf}' => Some(0xC0),              // タ Ta
			'\u{30c1}' => Some(0xC1),              // チ Ti
			'\u{30c4}' => Some(0xC2),              // ツ Tu
			'\u{30c6}' => Some(0xC3),              // テ Te
			'\u{30c8}' => Some(0xC4),              // ト To
			'\u{30ca}' => Some(0xC5),              // ナ Na
			'\u{30cb}' => Some(0xC6),              // ニ Ni
			'\u{30cc}' => Some(0xC7),              // ヌ Nu
			'\u{30cd}' => Some(0xC8),              // ネ Ne
			'\u{30ce}' => Some(0xC9),              // ノ No
			'\u{30cf}' => Some(0xCA),              // ハ Ha
			'\u{30d2}' => Some(0xCB),              // ヒ Hi
			'\u{30d5}' => Some(0xCC),              // フ Hu
			'\u{30d8}' => Some(0xCD),              // ヘ He
			'\u{30db}' => Some(0xCE),              // ホ Ho
			'\u{30de}' => Some(0xCF),              // マ Ma
			'\u{30df}' => Some(0xD0),              // ミ Mi
			'\u{30e0}' => Some(0xD1),              // ム Mu
			'\u{30e1}' => Some(0xD2),              // メ Me
			'\u{30e2}' => Some(0xD3),              // モ Mo
			'\u{30e4}' => Some(0xD4),              // ヤ Ya
			'\u{30e6}' => Some(0xD5),              // ユ Yu
			'\u{30e8}' => Some(0xD6),              // ヨ Yo
			'\u{30e9}' => Some(0xD7),              // ラ Ra
			'\u{30ea}' => Some(0xD8),              // リ Ri
			'\u{30eb}' => Some(0xD9),              // ル Ru
			'\u{30ec}' => Some(0xDA),              // レ Re
			'\u{30ed}' => Some(0xDB),              // ロ Ro
			'\u{30ef}' => Some(0xDC),              // ワ Wa
			'\u{30f3}' => Some(0xDD),              // ン N
			'\u{309B}' | '\u{3099}' => Some(0xDE), // ゛ Dakuten
			'\u{309C}' | '\u{309A}' => Some(0xDF), // ゜ Handakuten
			// Upper (5x10 Extra)
			'\u{03b1}' => Some(0xE0), // α Small Alpha
			'\u{00e4}' => Some(0xE1), // ä Small A with Diaeresis
			'\u{03b2}' => Some(0xE2), // β Small Beta
			'\u{03b5}' => Some(0xE3), // ε Small Epsilon
			'\u{00b5}' => Some(0xE4), // µ Small Mu/Micro
			'\u{03c3}' => Some(0xE5), // σ Small Sigma
			'\u{03c1}' => Some(0xE6), // ρ Small Rho
			// E7: Small G (tall version)
			'\u{221a}' => Some(0xE8), // √ Square Root
			// E9: Superscript -1 (has no unicode character)
			// EA: Small J (tall version)
			// EB: Superscript Small X (has no unicode character)
			'\u{00A2}' => Some(0xEC), // ¢ Cent
			'\u{2c60}' => Some(0xED), // Ⱡ Capital L with Double Bar
			'\u{00f1}' => Some(0xEE), // ñ Small N with Tilde
			'\u{00f6}' => Some(0xEF), // ö Small O with Diaeresis
			// F0: Small P (tall version)
			// F1: Small Q (tall version)
			'\u{03b8}' => Some(0xF2), // θ Small Theta
			'\u{221e}' => Some(0xF3), // ∞ Infinity
			'\u{03a9}' => Some(0xF4), // Ω Capital Omega
			'\u{00fc}' => Some(0xF5), // ü Small U with Diaeresis
			'\u{03a3}' => Some(0xF6), // Σ Capital Sigma
			'\u{03c0}' => Some(0xF7), // π Small Pi
			// F8: Small X-Bar (has no unicode character)
			// F9: Small Y (tall version)
			'\u{5343}' => Some(0xFA), // 千 Sen (1,000)
			'\u{4E07}' => Some(0xFB), // 万 Man (10,000)
			'\u{5186}' => Some(0xFC), // 円 Yen/¥
			'\u{00f7}' => Some(0xFD), // ÷ Division
			// FE: Empty
			'\u{2588}' => Some(0xFF), // █ Full Block
			// Unmatched
			ch if ch.is_whitespace() => Some(b' '), // full-width space
			_ => None,
		}
	}
}

/// European Standard Font Character Set.
//
/// For reference, see page 18 on [the Hitachi datasheet by Sparkfun](https://www.sparkfun.com/datasheets/LCD/HD44780.pdf).
#[derive(Debug, Clone, Copy)]
pub struct CharsetA02;

impl CharsetA02 {
	pub const EMPTY_FALLBACK: EmptyFallback<Self> = Fallback(Self);
	pub const QUESTION_FALLBACK: QuestionFallback<Self> = Fallback(Self);
}

impl Charset for CharsetA02 {
	fn code_from_utf8(&self, ch: char) -> Option<u8> {
		match ch {
			'\x00'..='\x0f' | '\x20'..='\x7e' => Some(ch as u8), // CGRAM + ASCII
			// Latin-1 with exceptions
			'\u{A8}' | '\u{AC}' | '\u{AD}' | '\u{AF}' | '\u{B4}' | '\u{B8}' | '\u{D8}' | '\u{F8}' => None,
			'\u{A1}'..='\u{FF}' => Some(ch as u8), // Latin-1 Supplement
			// Lower
			'\u{23F5}' => Some(0x00), // ⏵
			'\u{23F4}' => Some(0x01), // ⏴
			'\u{201C}' => Some(0x02), // “
			'\u{201D}' => Some(0x03), // ”
			'\u{23EB}' => Some(0x04), // ⏫
			'\u{23EC}' => Some(0x05), // ⏬
			'\u{23FA}' => Some(0x06), // ⏺
			'\u{21b2}' => Some(0x07), // ↲
			'\u{2191}' => Some(0x08), // ↑
			'\u{2193}' => Some(0x09), // ↓
			'\u{2192}' => Some(0x0A), // →
			'\u{2190}' => Some(0x0B), // ←
			'\u{2264}' => Some(0x0C), // ≤
			'\u{2265}' => Some(0x0D), // ≥
			'\u{23F6}' => Some(0x0E), // ⏶
			'\u{23F7}' => Some(0x0F), // ⏷
			// Middle
			'\u{2302}' => Some(0x7F), // ⌂
			// Cyrillic
			'\u{0410}' => Some(b'A'), // А
			'\u{0411}' => Some(0x80), // Б
			'\u{0412}' => Some(b'B'), // В
			'\u{0413}' => Some(0x92), // Г
			'\u{0414}' => Some(0x81), // Д
			'\u{0415}' => Some(b'E'), // Е
			'\u{0416}' => Some(0x82), // Ж
			'\u{0417}' => Some(0x83), // З
			'\u{0418}' => Some(0x84), // И
			'\u{0419}' => Some(0x85), // Й
			'\u{041A}' => Some(b'K'), // К
			'\u{041B}' => Some(0x86), // Л
			'\u{041C}' => Some(b'M'), // М
			'\u{041D}' => Some(b'H'), // Н
			'\u{041E}' => Some(b'O'), // О
			'\u{041F}' => Some(0x87), // П
			'\u{0420}' => Some(b'P'), // Р
			'\u{0421}' => Some(b'C'), // С
			'\u{0422}' => Some(b'T'), // Т
			'\u{0423}' => Some(0x88), // У
			//'\u{0424}' => None,     // Ф
			'\u{0425}' => Some(b'X'), // Х
			'\u{0426}' => Some(0x89), // Ц
			'\u{0427}' => Some(0x8A), // Ч
			'\u{0428}' => Some(0x8B), // Ш
			'\u{0429}' => Some(0x8C), // Щ
			'\u{042A}' => Some(0x8D), // Ъ
			'\u{042B}' => Some(0x8E), // Ы
			'\u{042C}' => Some(b'b'), // Ь
			'\u{042D}' => Some(0x8F), // Э
			'\u{042E}' => Some(0xAC), // Ю
			'\u{042F}' => Some(0xAD), // Я
			// Other
			'\u{03B1}' => Some(0x90),  // α Small Alpha
			'\u{266A}' => Some(0x91),  // ♪ Eighth Note
			'\u{03C0}' => Some(0x93),  // π Small Pi
			'\u{03A3}' => Some(0x94),  // Σ Capital Sigma
			'\u{03C3}' => Some(0x95),  // σ Small Sigma
			'\u{266C}' => Some(0x96),  // ♬ Beamed Sixteenth Notes
			'\u{03C4}' => Some(0x97),  // τ Small Tau
			'\u{1F514}' => Some(0x98), // 🔔 Bell
			'\u{03F4}' => Some(0x99),  // ϴ Capital Theta
			'\u{03A9}' => Some(0x9A),  // Ω Capital Omega
			'\u{03B4}' => Some(0x9B),  // δ Small Delta
			'\u{221e}' => Some(0x9C),  // ∞ Infinity
			'\u{2665}' => Some(0x9D),  // ♥ Heart
			'\u{03B5}' => Some(0x9E),  // ε Small Epsilon
			'\u{2229}' => Some(0x9F),  // ∩ Intersection
			'\u{23F8}' => Some(0xA0),  // ⏸ Double Vertical Bar
			'\u{2A0D}' => Some(0xA8),  // ⨍ Finite Part Integral
			// B4: Pt Symbol (has no unicode character)
			'\u{03C9}' => Some(0xB8),  // ω Small Omega
			'\u{0278}' => Some(0xD8),  // ɸ Small Phi
			'\u{222E}' => Some(0xF8),  // ∮ Contour Integral
			'\u{2018}' => Some(0xAF),  // ‘
			'\u{2019}' => Some(b'\''), // ’
			_ => None,
		}
	}
}
