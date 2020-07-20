/* \file, derived from iupkey.h
 * \brief Keyboard Keys definitions.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::os::raw::{c_int, c_char};

/* from 32 to 126, all character sets are equal,
   the key code is the same as the ASCii character code. */

pub const K_SP: c_int =          ' ' as c_int;   /* 32 (0x20) */
pub const K_exclam: c_int =      '!' as c_int;   /* 33 */
pub const K_quotedbl: c_int =    '\"' as c_int;  /* 34 */
pub const K_numbersign: c_int =  '#'  as c_int;  /* 35 */
pub const K_dollar: c_int =      '$' as c_int;   /* 36 */
pub const K_percent: c_int =     '%' as c_int;   /* 37 */
pub const K_ampersand: c_int =   '&' as c_int;   /* 38 */
pub const K_apostrophe: c_int =  '\'' as c_int;  /* 39 */
pub const K_parentleft: c_int =  '(' as c_int;   /* 40 */
pub const K_parentright: c_int = ')' as c_int;   /* 41 */
pub const K_asterisk: c_int =    '*' as c_int;   /* 42 */
pub const K_plus: c_int =        '+' as c_int;   /* 43 */
pub const K_comma: c_int =       ',' as c_int;   /* 44 */
pub const K_minus: c_int =       '-' as c_int;   /* 45 */
pub const K_period: c_int =      '.' as c_int;   /* 46 */
pub const K_slash: c_int =       '/' as c_int;   /* 47 */
pub const K_0: c_int =           '0' as c_int;   /* 48 (0x30) */
pub const K_1: c_int =           '1' as c_int;   /* 49 */
pub const K_2: c_int =           '2' as c_int;   /* 50 */
pub const K_3: c_int =           '3' as c_int;   /* 51 */
pub const K_4: c_int =           '4' as c_int;   /* 52 */
pub const K_5: c_int =           '5' as c_int;   /* 53 */
pub const K_6: c_int =           '6' as c_int;   /* 54 */
pub const K_7: c_int =           '7' as c_int;   /* 55 */
pub const K_8: c_int =           '8' as c_int;   /* 56 */
pub const K_9: c_int =           '9' as c_int;   /* 57 */
pub const K_colon: c_int =       ':' as c_int;   /* 58 */
pub const K_semicolon: c_int =   ';' as c_int;   /* 59 */
pub const K_less: c_int =        '<' as c_int;   /* 60 */
pub const K_equal: c_int =       '=' as c_int;   /* 61 */
pub const K_greater: c_int =     '>' as c_int;   /* 62 */
pub const K_question: c_int =    '?' as c_int;   /* 63 */
pub const K_at: c_int =          '@' as c_int;   /* 64 */
pub const K_A: c_int =           'A' as c_int;   /* 65 (0x41) */
pub const K_B : c_int =          'B' as c_int;   /* 66 */
pub const K_C: c_int =           'C' as c_int;   /* 67 */
pub const K_D: c_int =           'D' as c_int;   /* 68 */
pub const K_E: c_int =           'E' as c_int;   /* 69 */
pub const K_F: c_int =           'F' as c_int;   /* 70 */
pub const K_G: c_int =           'G' as c_int;   /* 71 */
pub const K_H: c_int =           'H' as c_int;   /* 72 */
pub const K_I: c_int =           'I' as c_int;   /* 73 */
pub const K_J: c_int =           'J' as c_int;   /* 74 */
pub const K_K: c_int =           'K' as c_int;   /* 75 */
pub const K_L: c_int =           'L' as c_int;   /* 76 */
pub const K_M: c_int =           'M' as c_int;   /* 77 */
pub const K_N: c_int =           'N' as c_int;   /* 78 */
pub const K_O: c_int =           'O' as c_int;   /* 79 */
pub const K_P: c_int =           'P' as c_int;   /* 80 */
pub const K_Q: c_int =           'Q' as c_int;   /* 81 */
pub const K_R: c_int =           'R' as c_int;   /* 82 */
pub const K_S: c_int =           'S' as c_int;   /* 83 */
pub const K_T: c_int =           'T' as c_int;   /* 84 */
pub const K_U: c_int =           'U' as c_int;   /* 85 */
pub const K_V: c_int =           'V' as c_int;   /* 86 */
pub const K_W: c_int =           'W' as c_int;   /* 87 */
pub const K_X: c_int =           'X' as c_int;   /* 88 */
pub const K_Y: c_int =           'Y' as c_int;   /* 89 */
pub const K_Z: c_int =           'Z' as c_int;   /* 90 */
pub const K_bracketleft: c_int = '[' as c_int;   /* 91 */
pub const K_backslash: c_int =   '\\' as c_int;  /* 92 */
pub const K_bracketright: c_int = ']' as c_int;  /* 93 */
pub const K_circum: c_int =      '^' as c_int;   /* 94 */
pub const K_underscore: c_int =  '_' as c_int;   /* 95 */
pub const K_grave: c_int =       '`' as c_int;   /* 96 */
pub const K_a: c_int =           'a' as c_int;   /* 97 (0x61) */
pub const K_b: c_int =           'b' as c_int;   /* 98 */
pub const K_c: c_int =           'c' as c_int;   /* 99 */
pub const K_d: c_int =           'd' as c_int;   /* 100 */
pub const K_e: c_int =           'e' as c_int;   /* 101 */
pub const K_f: c_int =           'f' as c_int;   /* 102 */
pub const K_g: c_int =           'g' as c_int;   /* 103 */
pub const K_h: c_int =           'h' as c_int;   /* 104 */
pub const K_i: c_int =           'i' as c_int;   /* 105 */
pub const K_j: c_int =           'j' as c_int;   /* 106 */
pub const K_k: c_int =           'k' as c_int;   /* 107 */
pub const K_l: c_int =           'l' as c_int;   /* 108 */
pub const K_m: c_int =           'm' as c_int;   /* 109 */
pub const K_n: c_int =           'n' as c_int;   /* 110 */
pub const K_o: c_int =           'o' as c_int;   /* 111 */
pub const K_p: c_int =           'p' as c_int;   /* 112 */
pub const K_q: c_int =           'q' as c_int;   /* 113 */
pub const K_r: c_int =           'r' as c_int;   /* 114 */
pub const K_s: c_int =           's' as c_int;   /* 115 */
pub const K_t: c_int =           't' as c_int;   /* 116 */
pub const K_u: c_int =           'u' as c_int;   /* 117 */
pub const K_v: c_int =           'v' as c_int;   /* 118 */
pub const K_w: c_int =           'w' as c_int;   /* 119 */
pub const K_x: c_int =           'x' as c_int;   /* 120 */
pub const K_y: c_int =           'y' as c_int;   /* 121 */
pub const K_z: c_int =           'z' as c_int;   /* 122 */
pub const K_braceleft: c_int =   '{' as c_int;   /* 123 */
pub const K_bar: c_int =         '|' as c_int;   /* 124 */
pub const K_braceright: c_int =  '}' as c_int;   /* 125 */
pub const K_tilde: c_int =       '~' as c_int;   /* 126 (0x7E) */

/* Printable ASCii keys */

#[inline(always)]
pub fn iup_isprint(c: i32) -> bool   { c > 31 && c < 127 }

/* also define the escape sequences that have keys associated */

pub const K_BS: c_int =                 8;       /* 8 */  //  '\b' as c_int
pub const K_TAB: c_int =    '\t' as c_int;       /* 9 */
pub const K_LF: c_int =     '\n' as c_int;       /* 10 (0x0A) not a real key, is a combination of CR with a modifier, just to document */
pub const K_CR: c_int =     '\r' as c_int;       /* 13 (0x0D) */

/* backward compatible definitions */

pub const K_quoteleft: c_int =   K_grave;
pub const K_quoteright: c_int =  K_apostrophe;
// #define  isxkey        iup_isXkey

/* IUP Extended Key Codes, range start at 128      */

#[inline(always)]
pub const fn iup_isXkey(c: i32) -> bool   { c >= 128 }

/* These use the same definition as X11 and GDK.
   This also means that any X11 or GDK definition can also be used. */

pub const K_PAUSE: c_int =    0xFF13;
pub const K_ESC: c_int =      0xFF1B;
pub const K_HOME: c_int =     0xFF50;
pub const K_LEFT: c_int =     0xFF51;
pub const K_UP: c_int =       0xFF52;
pub const K_RIGHT: c_int =    0xFF53;
pub const K_DOWN: c_int =     0xFF54;
pub const K_PGUP: c_int =     0xFF55;
pub const K_PGDN: c_int =     0xFF56;
pub const K_END: c_int =      0xFF57;
pub const K_MIDDLE: c_int =   0xFF0B;
pub const K_Print: c_int =    0xFF61;
pub const K_INS: c_int =      0xFF63;
pub const K_Menu: c_int =     0xFF67;
pub const K_DEL: c_int =      0xFFFF;
pub const K_F1: c_int =       0xFFBE;
pub const K_F2: c_int =       0xFFBF;
pub const K_F3: c_int =       0xFFC0;
pub const K_F4: c_int =       0xFFC1;
pub const K_F5: c_int =       0xFFC2;
pub const K_F6: c_int =       0xFFC3;
pub const K_F7: c_int =       0xFFC4;
pub const K_F8: c_int =       0xFFC5;
pub const K_F9: c_int =       0xFFC6;
pub const K_F10: c_int =      0xFFC7;
pub const K_F11: c_int =      0xFFC8;
pub const K_F12: c_int =      0xFFC9;
pub const K_F13: c_int =      0xFFCA;
pub const K_F14: c_int =      0xFFCB;
pub const K_F15: c_int =      0xFFCC;
pub const K_F16: c_int =      0xFFCD;
pub const K_F17: c_int =      0xFFCE;
pub const K_F18: c_int =      0xFFCF;
pub const K_F19: c_int =      0xFFD0;
pub const K_F20: c_int =      0xFFD1;

/* no Shift/Ctrl/Alt */
pub const K_LSHIFT: c_int =   0xFFE1;
pub const K_RSHIFT: c_int =   0xFFE2;
pub const K_LCTRL: c_int =    0xFFE3;
pub const K_RCTRL: c_int =    0xFFE4;
pub const K_LALT: c_int =     0xFFE9;
pub const K_RALT: c_int =     0xFFEA;

pub const K_NUM: c_int =      0xFF7F;
pub const K_SCROLL: c_int =   0xFF14;
pub const K_CAPS: c_int =     0xFFE5;

/* Mac clear button. Value randomly picked trying to avoid clashing with an existing value. */
pub const K_CLEAR: c_int =    0xFFD2;
/* Help button if anybody has it. Value randomly picked trying to avoid clashing with an existing value. */
pub const K_HELP: c_int =     0xFFD3;

/* Also, these are the same as the Latin-1 definition */

pub const K_ccedilla: c_int =  0x00E7;
pub const K_Ccedilla: c_int =  0x00C7;
pub const K_acute: c_int =     0x00B4;  /* no Shift/Ctrl/Alt */
pub const K_diaeresis: c_int = 0x00A8;

/******************************************************/
/* Modifiers use last 4 bits. Since IUP 3.9           */
/* These modifiers definitions are specific to IUP    */
/******************************************************/

#[inline(always)]
pub const fn iup_isShiftXkey(c: c_int) -> bool { (c & 0x10000000) != 0 }
#[inline(always)]
pub const fn iup_isCtrlXkey(c: c_int) -> bool  { (c & 0x20000000) != 0 }
#[inline(always)]
pub const fn iup_isAltXkey(c: c_int) -> bool   { (c & 0x40000000) != 0 }
#[allow(overflowing_literals)]
#[inline(always)]
pub const fn iup_isSysXkey(c: c_int) -> bool   { (c & 0x80000000) != 0 }

#[inline(always)]
pub const fn iup_XkeyBase(c: c_int) -> c_int  { c & 0x0FFFFFFF }
#[inline(always)]
pub const fn iup_XkeyShift(c: c_int) -> c_int { c | 0x10000000 }   /* Shift  */
#[inline(always)]
pub const fn iup_XkeyCtrl(c: c_int) -> c_int  { c | 0x20000000 }   /* Ctrl   */
#[inline(always)]
pub const fn iup_XkeyAlt(c: c_int) -> c_int   { c | 0x40000000 }   /* Alt    */
#[allow(overflowing_literals)]
#[inline(always)]
pub const fn iup_XkeySys(c: c_int) -> c_int   { c | 0x80000000 }   /* Sys (Win or Apple) - notice that using "int" will display a negative value */

/* These definitions are here for backward compatibility
   and to simplify some key combination usage.
   But since IUP 3.9, modifiers can be combined with any key
   and they can be mixed together. */

pub const K_sHOME: c_int =    iup_XkeyShift(K_HOME   );
pub const K_sUP: c_int =      iup_XkeyShift(K_UP     );
pub const K_sPGUP: c_int =    iup_XkeyShift(K_PGUP   );
pub const K_sLEFT: c_int =    iup_XkeyShift(K_LEFT   );
pub const K_sMIDDLE: c_int =  iup_XkeyShift(K_MIDDLE );
pub const K_sRIGHT: c_int =   iup_XkeyShift(K_RIGHT  );
pub const K_sEND: c_int =     iup_XkeyShift(K_END    );
pub const K_sDOWN: c_int =    iup_XkeyShift(K_DOWN   );
pub const K_sPGDN: c_int =    iup_XkeyShift(K_PGDN   );
pub const K_sINS: c_int =     iup_XkeyShift(K_INS    );
pub const K_sDEL: c_int =     iup_XkeyShift(K_DEL    );
pub const K_sSP: c_int =      iup_XkeyShift(K_SP     );
pub const K_sTAB: c_int =     iup_XkeyShift(K_TAB    );
pub const K_sCR: c_int =      iup_XkeyShift(K_CR     );
pub const K_sBS: c_int =      iup_XkeyShift(K_BS     );
pub const K_sPAUSE: c_int =   iup_XkeyShift(K_PAUSE  );
pub const K_sESC: c_int =     iup_XkeyShift(K_ESC    );
pub const K_sCLEAR: c_int =   iup_XkeyShift(K_CLEAR  );
pub const K_sF1: c_int =      iup_XkeyShift(K_F1     );
pub const K_sF2: c_int =      iup_XkeyShift(K_F2     );
pub const K_sF3: c_int =      iup_XkeyShift(K_F3     );
pub const K_sF4: c_int =      iup_XkeyShift(K_F4     );
pub const K_sF5: c_int =      iup_XkeyShift(K_F5     );
pub const K_sF6: c_int =      iup_XkeyShift(K_F6     );
pub const K_sF7: c_int =      iup_XkeyShift(K_F7     );
pub const K_sF8: c_int =      iup_XkeyShift(K_F8     );
pub const K_sF9: c_int =      iup_XkeyShift(K_F9     );
pub const K_sF10: c_int =     iup_XkeyShift(K_F10    );
pub const K_sF11: c_int =     iup_XkeyShift(K_F11    );
pub const K_sF12: c_int =     iup_XkeyShift(K_F12    );
pub const K_sF13: c_int =     iup_XkeyShift(K_F13    );
pub const K_sF14: c_int =     iup_XkeyShift(K_F14    );
pub const K_sF15: c_int =     iup_XkeyShift(K_F15    );
pub const K_sF16: c_int =     iup_XkeyShift(K_F16    );
pub const K_sF17: c_int =     iup_XkeyShift(K_F17    );
pub const K_sF18: c_int =     iup_XkeyShift(K_F18    );
pub const K_sF19: c_int =     iup_XkeyShift(K_F19    );
pub const K_sF20: c_int =     iup_XkeyShift(K_F20    );
pub const K_sPrint: c_int =   iup_XkeyShift(K_Print  );
pub const K_sMenu: c_int =    iup_XkeyShift(K_Menu   );

pub const K_cHOME: c_int =     iup_XkeyCtrl(K_HOME    );
pub const K_cUP: c_int =       iup_XkeyCtrl(K_UP      );
pub const K_cPGUP: c_int =     iup_XkeyCtrl(K_PGUP    );
pub const K_cLEFT: c_int =     iup_XkeyCtrl(K_LEFT    );
pub const K_cMIDDLE: c_int =   iup_XkeyCtrl(K_MIDDLE  );
pub const K_cRIGHT: c_int =    iup_XkeyCtrl(K_RIGHT   );
pub const K_cEND: c_int =      iup_XkeyCtrl(K_END     );
pub const K_cDOWN: c_int =     iup_XkeyCtrl(K_DOWN    );
pub const K_cPGDN: c_int =     iup_XkeyCtrl(K_PGDN    );
pub const K_cINS: c_int =      iup_XkeyCtrl(K_INS     );
pub const K_cDEL: c_int =      iup_XkeyCtrl(K_DEL     );
pub const K_cSP: c_int =       iup_XkeyCtrl(K_SP      );
pub const K_cTAB: c_int =      iup_XkeyCtrl(K_TAB     );
pub const K_cCR: c_int =       iup_XkeyCtrl(K_CR      );
pub const K_cBS: c_int =       iup_XkeyCtrl(K_BS      );
pub const K_cPAUSE: c_int =    iup_XkeyCtrl(K_PAUSE   );
pub const K_cESC: c_int =      iup_XkeyCtrl(K_ESC     );
pub const K_cCLEAR: c_int =    iup_XkeyCtrl(K_CLEAR   );
pub const K_cCcedilla: c_int = iup_XkeyCtrl(K_Ccedilla);
pub const K_cF1: c_int =       iup_XkeyCtrl(K_F1      );
pub const K_cF2: c_int =       iup_XkeyCtrl(K_F2      );
pub const K_cF3: c_int =       iup_XkeyCtrl(K_F3      );
pub const K_cF4: c_int =       iup_XkeyCtrl(K_F4      );
pub const K_cF5: c_int =       iup_XkeyCtrl(K_F5      );
pub const K_cF6: c_int =       iup_XkeyCtrl(K_F6      );
pub const K_cF7: c_int =       iup_XkeyCtrl(K_F7      );
pub const K_cF8: c_int =       iup_XkeyCtrl(K_F8      );
pub const K_cF9: c_int =       iup_XkeyCtrl(K_F9      );
pub const K_cF10: c_int =      iup_XkeyCtrl(K_F10     );
pub const K_cF11: c_int =      iup_XkeyCtrl(K_F11     );
pub const K_cF12: c_int =      iup_XkeyCtrl(K_F12     );
pub const K_cF13: c_int =      iup_XkeyCtrl(K_F13     );
pub const K_cF14: c_int =      iup_XkeyCtrl(K_F14     );
pub const K_cF15: c_int =      iup_XkeyCtrl(K_F15     );
pub const K_cF16: c_int =      iup_XkeyCtrl(K_F16     );
pub const K_cF17: c_int =      iup_XkeyCtrl(K_F17     );
pub const K_cF18: c_int =      iup_XkeyCtrl(K_F18     );
pub const K_cF19: c_int =      iup_XkeyCtrl(K_F19     );
pub const K_cF20: c_int =      iup_XkeyCtrl(K_F20     );
pub const K_cPrint: c_int =    iup_XkeyCtrl(K_Print   );
pub const K_cMenu: c_int =     iup_XkeyCtrl(K_Menu    );

pub const K_mHOME: c_int =     iup_XkeyAlt(K_HOME    );
pub const K_mUP: c_int =       iup_XkeyAlt(K_UP      );
pub const K_mPGUP: c_int =     iup_XkeyAlt(K_PGUP    );
pub const K_mLEFT: c_int =     iup_XkeyAlt(K_LEFT    );
pub const K_mMIDDLE: c_int =   iup_XkeyAlt(K_MIDDLE  );
pub const K_mRIGHT: c_int =    iup_XkeyAlt(K_RIGHT   );
pub const K_mEND: c_int =      iup_XkeyAlt(K_END     );
pub const K_mDOWN: c_int =     iup_XkeyAlt(K_DOWN    );
pub const K_mPGDN: c_int =     iup_XkeyAlt(K_PGDN    );
pub const K_mINS: c_int =      iup_XkeyAlt(K_INS     );
pub const K_mDEL: c_int =      iup_XkeyAlt(K_DEL     );
pub const K_mSP: c_int =       iup_XkeyAlt(K_SP      );
pub const K_mTAB: c_int =      iup_XkeyAlt(K_TAB     );
pub const K_mCR: c_int =       iup_XkeyAlt(K_CR      );
pub const K_mBS: c_int =       iup_XkeyAlt(K_BS      );
pub const K_mPAUSE: c_int =    iup_XkeyAlt(K_PAUSE   );
pub const K_mESC: c_int =      iup_XkeyAlt(K_ESC     );
pub const K_mCLEAR: c_int =    iup_XkeyAlt(K_CLEAR   );
pub const K_mCcedilla: c_int = iup_XkeyAlt(K_Ccedilla);
pub const K_mF1: c_int =       iup_XkeyAlt(K_F1      );
pub const K_mF2: c_int =       iup_XkeyAlt(K_F2      );
pub const K_mF3: c_int =       iup_XkeyAlt(K_F3      );
pub const K_mF4: c_int =       iup_XkeyAlt(K_F4      );
pub const K_mF5: c_int =       iup_XkeyAlt(K_F5      );
pub const K_mF6: c_int =       iup_XkeyAlt(K_F6      );
pub const K_mF7: c_int =       iup_XkeyAlt(K_F7      );
pub const K_mF8: c_int =       iup_XkeyAlt(K_F8      );
pub const K_mF9: c_int =       iup_XkeyAlt(K_F9      );
pub const K_mF10: c_int =      iup_XkeyAlt(K_F10     );
pub const K_mF11: c_int =      iup_XkeyAlt(K_F11     );
pub const K_mF12: c_int =      iup_XkeyAlt(K_F12     );
pub const K_mF13: c_int =      iup_XkeyAlt(K_F13     );
pub const K_mF14: c_int =      iup_XkeyAlt(K_F14     );
pub const K_mF15: c_int =      iup_XkeyAlt(K_F15     );
pub const K_mF16: c_int =      iup_XkeyAlt(K_F16     );
pub const K_mF17: c_int =      iup_XkeyAlt(K_F17     );
pub const K_mF18: c_int =      iup_XkeyAlt(K_F18     );
pub const K_mF19: c_int =      iup_XkeyAlt(K_F19     );
pub const K_mF20: c_int =      iup_XkeyAlt(K_F20     );
pub const K_mPrint: c_int =    iup_XkeyAlt(K_Print   );
pub const K_mMenu: c_int =     iup_XkeyAlt(K_Menu    );

pub const K_yHOME: c_int =     iup_XkeySys(K_HOME    );
pub const K_yUP: c_int =       iup_XkeySys(K_UP      );
pub const K_yPGUP: c_int =     iup_XkeySys(K_PGUP    );
pub const K_yLEFT: c_int =     iup_XkeySys(K_LEFT    );
pub const K_yMIDDLE: c_int =   iup_XkeySys(K_MIDDLE  );
pub const K_yRIGHT: c_int =    iup_XkeySys(K_RIGHT   );
pub const K_yEND: c_int =      iup_XkeySys(K_END     );
pub const K_yDOWN: c_int =     iup_XkeySys(K_DOWN    );
pub const K_yPGDN: c_int =     iup_XkeySys(K_PGDN    );
pub const K_yINS: c_int =      iup_XkeySys(K_INS     );
pub const K_yDEL: c_int =      iup_XkeySys(K_DEL     );
pub const K_ySP: c_int =       iup_XkeySys(K_SP      );
pub const K_yTAB: c_int =      iup_XkeySys(K_TAB     );
pub const K_yCR: c_int =       iup_XkeySys(K_CR      );
pub const K_yBS: c_int =       iup_XkeySys(K_BS      );
pub const K_yPAUSE: c_int =    iup_XkeySys(K_PAUSE   );
pub const K_yESC: c_int =      iup_XkeySys(K_ESC     );
pub const K_yCLEAR: c_int =    iup_XkeySys(K_CLEAR   );
pub const K_yCcedilla: c_int = iup_XkeySys(K_Ccedilla);
pub const K_yF1: c_int =       iup_XkeySys(K_F1      );
pub const K_yF2: c_int =       iup_XkeySys(K_F2      );
pub const K_yF3: c_int =       iup_XkeySys(K_F3      );
pub const K_yF4: c_int =       iup_XkeySys(K_F4      );
pub const K_yF5: c_int =       iup_XkeySys(K_F5      );
pub const K_yF6: c_int =       iup_XkeySys(K_F6      );
pub const K_yF7: c_int =       iup_XkeySys(K_F7      );
pub const K_yF8: c_int =       iup_XkeySys(K_F8      );
pub const K_yF9: c_int =       iup_XkeySys(K_F9      );
pub const K_yF10: c_int =      iup_XkeySys(K_F10     );
pub const K_yF11: c_int =      iup_XkeySys(K_F11     );
pub const K_yF12: c_int =      iup_XkeySys(K_F12     );
pub const K_yF13: c_int =      iup_XkeySys(K_F13     );
pub const K_yF14: c_int =      iup_XkeySys(K_F14     );
pub const K_yF15: c_int =      iup_XkeySys(K_F15     );
pub const K_yF16: c_int =      iup_XkeySys(K_F16     );
pub const K_yF17: c_int =      iup_XkeySys(K_F17     );
pub const K_yF18: c_int =      iup_XkeySys(K_F18     );
pub const K_yF19: c_int =      iup_XkeySys(K_F19     );
pub const K_yF20: c_int =      iup_XkeySys(K_F20     );
pub const K_yPrint: c_int =    iup_XkeySys(K_Print   );
pub const K_yMenu: c_int =     iup_XkeySys(K_Menu    );

pub const K_sPlus: c_int =          iup_XkeyShift(K_plus    );
pub const K_sComma: c_int =         iup_XkeyShift(K_comma   );
pub const K_sMinus: c_int =         iup_XkeyShift(K_minus   );
pub const K_sPeriod: c_int =        iup_XkeyShift(K_period  );
pub const K_sSlash: c_int =         iup_XkeyShift(K_slash   );
pub const K_sAsterisk: c_int =      iup_XkeyShift(K_asterisk);

pub const K_cA: c_int =      iup_XkeyCtrl(K_A);
pub const K_cB: c_int =      iup_XkeyCtrl(K_B);
pub const K_cC: c_int =      iup_XkeyCtrl(K_C);
pub const K_cD: c_int =      iup_XkeyCtrl(K_D);
pub const K_cE: c_int =      iup_XkeyCtrl(K_E);
pub const K_cF: c_int =      iup_XkeyCtrl(K_F);
pub const K_cG: c_int =      iup_XkeyCtrl(K_G);
pub const K_cH: c_int =      iup_XkeyCtrl(K_H);
pub const K_cI: c_int =      iup_XkeyCtrl(K_I);
pub const K_cJ: c_int =      iup_XkeyCtrl(K_J);
pub const K_cK: c_int =      iup_XkeyCtrl(K_K);
pub const K_cL: c_int =      iup_XkeyCtrl(K_L);
pub const K_cM: c_int =      iup_XkeyCtrl(K_M);
pub const K_cN: c_int =      iup_XkeyCtrl(K_N);
pub const K_cO: c_int =      iup_XkeyCtrl(K_O);
pub const K_cP: c_int =      iup_XkeyCtrl(K_P);
pub const K_cQ: c_int =      iup_XkeyCtrl(K_Q);
pub const K_cR: c_int =      iup_XkeyCtrl(K_R);
pub const K_cS: c_int =      iup_XkeyCtrl(K_S);
pub const K_cT: c_int =      iup_XkeyCtrl(K_T);
pub const K_cU: c_int =      iup_XkeyCtrl(K_U);
pub const K_cV: c_int =      iup_XkeyCtrl(K_V);
pub const K_cW: c_int =      iup_XkeyCtrl(K_W);
pub const K_cX: c_int =      iup_XkeyCtrl(K_X);
pub const K_cY: c_int =      iup_XkeyCtrl(K_Y);
pub const K_cZ: c_int =      iup_XkeyCtrl(K_Z);
pub const K_c1: c_int =      iup_XkeyCtrl(K_1);
pub const K_c2: c_int =      iup_XkeyCtrl(K_2);
pub const K_c3: c_int =      iup_XkeyCtrl(K_3);
pub const K_c4: c_int =      iup_XkeyCtrl(K_4);
pub const K_c5: c_int =      iup_XkeyCtrl(K_5);
pub const K_c6: c_int =      iup_XkeyCtrl(K_6);
pub const K_c7: c_int =      iup_XkeyCtrl(K_7);
pub const K_c8: c_int =      iup_XkeyCtrl(K_8);
pub const K_c9: c_int =      iup_XkeyCtrl(K_9);
pub const K_c0: c_int =      iup_XkeyCtrl(K_0);
pub const K_cPlus: c_int =         iup_XkeyCtrl(K_plus        );
pub const K_cComma: c_int =        iup_XkeyCtrl(K_comma       );
pub const K_cMinus: c_int =        iup_XkeyCtrl(K_minus       );
pub const K_cPeriod: c_int =       iup_XkeyCtrl(K_period      );
pub const K_cSlash: c_int =        iup_XkeyCtrl(K_slash       );
pub const K_cSemicolon: c_int =    iup_XkeyCtrl(K_semicolon   );
pub const K_cEqual: c_int =        iup_XkeyCtrl(K_equal       );
pub const K_cBracketleft: c_int =  iup_XkeyCtrl(K_bracketleft );
pub const K_cBracketright: c_int = iup_XkeyCtrl(K_bracketright);
pub const K_cBackslash: c_int =    iup_XkeyCtrl(K_backslash   );
pub const K_cAsterisk: c_int =     iup_XkeyCtrl(K_asterisk    );

pub const K_mA: c_int =     iup_XkeyAlt(K_A);
pub const K_mB: c_int =     iup_XkeyAlt(K_B);
pub const K_mC: c_int =     iup_XkeyAlt(K_C);
pub const K_mD: c_int =     iup_XkeyAlt(K_D);
pub const K_mE: c_int =     iup_XkeyAlt(K_E);
pub const K_mF: c_int =     iup_XkeyAlt(K_F);
pub const K_mG: c_int =     iup_XkeyAlt(K_G);
pub const K_mH: c_int =     iup_XkeyAlt(K_H);
pub const K_mI: c_int =     iup_XkeyAlt(K_I);
pub const K_mJ: c_int =     iup_XkeyAlt(K_J);
pub const K_mK: c_int =     iup_XkeyAlt(K_K);
pub const K_mL: c_int =     iup_XkeyAlt(K_L);
pub const K_mM: c_int =     iup_XkeyAlt(K_M);
pub const K_mN: c_int =     iup_XkeyAlt(K_N);
pub const K_mO: c_int =     iup_XkeyAlt(K_O);
pub const K_mP: c_int =     iup_XkeyAlt(K_P);
pub const K_mQ: c_int =     iup_XkeyAlt(K_Q);
pub const K_mR: c_int =     iup_XkeyAlt(K_R);
pub const K_mS: c_int =     iup_XkeyAlt(K_S);
pub const K_mT: c_int =     iup_XkeyAlt(K_T);
pub const K_mU: c_int =     iup_XkeyAlt(K_U);
pub const K_mV: c_int =     iup_XkeyAlt(K_V);
pub const K_mW: c_int =     iup_XkeyAlt(K_W);
pub const K_mX: c_int =     iup_XkeyAlt(K_X);
pub const K_mY: c_int =     iup_XkeyAlt(K_Y);
pub const K_mZ: c_int =     iup_XkeyAlt(K_Z);
pub const K_m1: c_int =     iup_XkeyAlt(K_1);
pub const K_m2: c_int =     iup_XkeyAlt(K_2);
pub const K_m3: c_int =     iup_XkeyAlt(K_3);
pub const K_m4: c_int =     iup_XkeyAlt(K_4);
pub const K_m5: c_int =     iup_XkeyAlt(K_5);
pub const K_m6: c_int =     iup_XkeyAlt(K_6);
pub const K_m7: c_int =     iup_XkeyAlt(K_7);
pub const K_m8: c_int =     iup_XkeyAlt(K_8);
pub const K_m9: c_int =     iup_XkeyAlt(K_9);
pub const K_m0: c_int =     iup_XkeyAlt(K_0);
pub const K_mPlus: c_int =         iup_XkeyAlt(K_plus        );
pub const K_mComma: c_int =        iup_XkeyAlt(K_comma       );
pub const K_mMinus: c_int =        iup_XkeyAlt(K_minus       );
pub const K_mPeriod: c_int =       iup_XkeyAlt(K_period      );
pub const K_mSlash: c_int =        iup_XkeyAlt(K_slash       );
pub const K_mSemicolon: c_int =    iup_XkeyAlt(K_semicolon   );
pub const K_mEqual: c_int =        iup_XkeyAlt(K_equal       );
pub const K_mBracketleft: c_int =  iup_XkeyAlt(K_bracketleft );
pub const K_mBracketright: c_int = iup_XkeyAlt(K_bracketright);
pub const K_mBackslash: c_int =    iup_XkeyAlt(K_backslash   );
pub const K_mAsterisk: c_int =     iup_XkeyAlt(K_asterisk    );

pub const K_yA: c_int =     iup_XkeySys(K_A);
pub const K_yB: c_int =     iup_XkeySys(K_B);
pub const K_yC: c_int =     iup_XkeySys(K_C);
pub const K_yD: c_int =     iup_XkeySys(K_D);
pub const K_yE: c_int =     iup_XkeySys(K_E);
pub const K_yF: c_int =     iup_XkeySys(K_F);
pub const K_yG: c_int =     iup_XkeySys(K_G);
pub const K_yH: c_int =     iup_XkeySys(K_H);
pub const K_yI: c_int =     iup_XkeySys(K_I);
pub const K_yJ: c_int =     iup_XkeySys(K_J);
pub const K_yK: c_int =     iup_XkeySys(K_K);
pub const K_yL: c_int =     iup_XkeySys(K_L);
pub const K_yM: c_int =     iup_XkeySys(K_M);
pub const K_yN: c_int =     iup_XkeySys(K_N);
pub const K_yO: c_int =     iup_XkeySys(K_O);
pub const K_yP: c_int =     iup_XkeySys(K_P);
pub const K_yQ: c_int =     iup_XkeySys(K_Q);
pub const K_yR: c_int =     iup_XkeySys(K_R);
pub const K_yS: c_int =     iup_XkeySys(K_S);
pub const K_yT: c_int =     iup_XkeySys(K_T);
pub const K_yU: c_int =     iup_XkeySys(K_U);
pub const K_yV: c_int =     iup_XkeySys(K_V);
pub const K_yW: c_int =     iup_XkeySys(K_W);
pub const K_yX: c_int =     iup_XkeySys(K_X);
pub const K_yY: c_int =     iup_XkeySys(K_Y);
pub const K_yZ: c_int =     iup_XkeySys(K_Z);
pub const K_y1: c_int =     iup_XkeySys(K_1);
pub const K_y2: c_int =     iup_XkeySys(K_2);
pub const K_y3: c_int =     iup_XkeySys(K_3);
pub const K_y4: c_int =     iup_XkeySys(K_4);
pub const K_y5: c_int =     iup_XkeySys(K_5);
pub const K_y6: c_int =     iup_XkeySys(K_6);
pub const K_y7: c_int =     iup_XkeySys(K_7);
pub const K_y8: c_int =     iup_XkeySys(K_8);
pub const K_y9: c_int =     iup_XkeySys(K_9);
pub const K_y0: c_int =     iup_XkeySys(K_0);
pub const K_yPlus: c_int =           iup_XkeySys(K_plus        );
pub const K_yComma: c_int =          iup_XkeySys(K_comma       );
pub const K_yMinus: c_int =          iup_XkeySys(K_minus       );
pub const K_yPeriod: c_int =         iup_XkeySys(K_period      );
pub const K_ySlash: c_int =          iup_XkeySys(K_slash       );
pub const K_ySemicolon: c_int =      iup_XkeySys(K_semicolon   );
pub const K_yEqual: c_int =          iup_XkeySys(K_equal       );
pub const K_yBracketleft: c_int =    iup_XkeySys(K_bracketleft );
pub const K_yBracketright : c_int =  iup_XkeySys(K_bracketright);
pub const K_yBackslash : c_int =     iup_XkeySys(K_backslash   );
pub const K_yAsterisk: c_int =       iup_XkeySys(K_asterisk    );


/*
 *  function iupKeyCodeToName (for Rust: keycode_to_name) is not part of the official iupkey.h API, just added by binding author,
 *  as some IUP example code makes use of it; iupKeyCodeToName is provided by libiup.so/libiup.a/iup.lib/iup.dll:
*/

pub fn keycode_to_name(code: i32) -> Option<&'static str> {
    extern {
        fn strnlen(cs: *const c_char, maxlen: usize) -> usize;
        #[allow(dead_code)]
        fn iupKeyCodeToName(code: c_int) -> *const c_char;
    }

    const MAXLEN: usize = 32;
    let c_ptr = unsafe { iupKeyCodeToName(code) };
    let c_len = if c_ptr.is_null() {0_usize} else { unsafe { /*libc::*/strnlen(c_ptr, MAXLEN) } };
    if c_len==0 || c_len==MAXLEN { None }
    else                         { Some( unsafe { std::ffi::CStr::from_ptr(c_ptr).to_str().unwrap() } ) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_and_back_to_name() {
        assert_eq!(Some("K_s0xFFFFFFF"), keycode_to_name(-1));
        // println!("Value of K_SP: {}", K_SP);
        assert_eq!(K_SP, 32);
        assert_eq!("K_SP", keycode_to_name(K_SP).unwrap());

        assert_eq!(K_quotedbl, 34);
        assert_eq!("K_quotedbl", keycode_to_name(K_quotedbl).unwrap());

        assert_eq!(K_apostrophe, 39);
        assert_eq!("K_apostrophe", keycode_to_name(K_apostrophe).unwrap());

        assert_eq!(K_backslash, 92);
        assert_eq!("K_backslash", keycode_to_name(K_backslash).unwrap());
    }
}
