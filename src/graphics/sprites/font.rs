use crate::graphics::sprites::sprite_struct::Sprite;
use crate::entities::*;

pub enum FontSize {
    Big,
    Little,
}

pub fn StringToSprites(string: String, size: FontSize) -> Vec<Sprite> {
    let mut text: Vec<Sprite> = Vec::new();
    match size {
        FontSize::Big => {
            for letter in string.chars() {
                match letter {
                    'A' => { text.push(big_A()); },
                    'B' => { text.push(big_B()); },
                    'C' => { text.push(big_C()); },
                    'D' => { text.push(big_D()); },
                    'E' => { text.push(big_E()); },
                    'F' => { text.push(big_F()); },
                    'G' => { text.push(big_G()); },
                    'H' => { text.push(big_H()); },
                    'I' => { text.push(big_I()); },
                    'J' => { text.push(big_J()); },
                    'K' => { text.push(big_K()); },
                    'L' => { text.push(big_L()); },
                    'M' => { text.push(big_M()); },
                    'N' => { text.push(big_N()); },
                    'O' => { text.push(big_O()); },
                    'P' => { text.push(big_P()); },
                    'Q' => { text.push(big_Q()); },
                    'R' => { text.push(big_R()); },
                    'S' => { text.push(big_S()); },
                    'T' => { text.push(big_T()); },
                    'U' => { text.push(big_U()); },
                    'V' => { text.push(big_V()); },
                    'W' => { text.push(big_W()); },
                    'X' => { text.push(big_X()); },
                    'Y' => { text.push(big_Y()); },
                    'Z' => { text.push(big_Z()); },
                    'a' => { text.push(big_a()); },
                    'b' => { text.push(big_b()); },
                    'c' => { text.push(big_c()); },
                    'd' => { text.push(big_d()); },
                    'e' => { text.push(big_e()); },
                    'f' => { text.push(big_f()); },
                    'g' => { text.push(big_g()); },
                    'h' => { text.push(big_h()); },
                    'i' => { text.push(big_i()); },
                    'j' => { text.push(big_j()); },
                    'k' => { text.push(big_k()); },
                    'l' => { text.push(big_l()); },
                    'm' => { text.push(big_m()); },
                    'n' => { text.push(big_n()); },
                    'o' => { text.push(big_o()); },
                    'p' => { text.push(big_p()); },
                    'q' => { text.push(big_q()); },
                    'r' => { text.push(big_r()); },
                    's' => { text.push(big_s()); },
                    't' => { text.push(big_t()); },
                    'u' => { text.push(big_u()); },
                    'v' => { text.push(big_v()); },
                    'w' => { text.push(big_w()); },
                    'x' => { text.push(big_x()); },
                    'y' => { text.push(big_y()); },
                    'z' => { text.push(big_z()); },
                    '0' => { text.push(big_zero()); },
                    '1' => { text.push(big_one()); },
                    '2' => { text.push(big_two()); },
                    '3' => { text.push(big_three()); },
                    '4' => { text.push(big_four()); },
                    '5' => { text.push(big_five()); },
                    '6' => { text.push(big_six()); },
                    '7' => { text.push(big_seven()); },
                    '8' => { text.push(big_eight()); },
                    '9' => { text.push(big_nine()); },
                    '.' => { text.push(big_fs()); },
                    ' ' => { text.push(big_space()); },
                    _ => {},
                }
            }
        },
        FontSize::Little => {
            for letter in string.chars() {
                match letter {
                    'A' | 'a' => { text.push(little_A()); },
                    'B' | 'b' => { text.push(little_B()); },
                    'C' | 'c' => { text.push(little_C()); },
                    'D' | 'd' => { text.push(little_D()); },
                    'E' | 'e' => { text.push(little_E()); },
                    'F' | 'f' => { text.push(little_F()); },
                    'G' | 'g' => { text.push(little_G()); },
                    'H' | 'h' => { text.push(little_H()); },
                    'I' | 'i' => { text.push(little_I()); },
                    'J' | 'j' => { text.push(little_J()); },
                    'K' | 'k' => { text.push(little_K()); },
                    'L' | 'l' => { text.push(little_L()); },
                    'M' | 'm' => { text.push(little_M()); },
                    'N' | 'n' => { text.push(little_N()); },
                    'O' | 'o' => { text.push(little_O()); },
                    'P' | 'p' => { text.push(little_P()); },
                    'Q' | 'q' => { text.push(little_Q()); },
                    'R' | 'r' => { text.push(little_R()); },
                    'S' | 's' => { text.push(little_S()); },
                    'T' | 't' => { text.push(little_T()); },
                    'U' | 'u' => { text.push(little_U()); },
                    'V' | 'v' => { text.push(little_V()); },
                    'W' | 'w' => { text.push(little_W()); },
                    'X' | 'x' => { text.push(little_X()); },
                    'Y' | 'y' => { text.push(little_Y()); },
                    'Z' | 'z' => { text.push(little_Z()); },
                    '0' => { text.push(little_zero()); },
                    '1' => { text.push(little_one()); },
                    '2' => { text.push(little_two()); },
                    '3' => { text.push(little_three()); },
                    '4' => { text.push(little_four()); },
                    '5' => { text.push(little_five()); },
                    '6' => { text.push(little_six()); },
                    '7' => { text.push(little_seven()); },
                    '8' => { text.push(little_eight()); },
                    '9' => { text.push(little_nine()); },
                    '.' => { text.push(little_fs()); },
                    ' ' => { text.push(little_space()); },
                    _ => {},
                }
            }
        },
    }
    text
}

// Font set used is IBM Conv.

pub fn logo() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
    ];
    let width = 52;
    let height = 14;
    Sprite::new(bitmap, width, height)
}

pub fn big_A() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_B() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_C() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_D() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_E() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_F() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_G() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_H() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_I() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_J() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_K() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_L() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_M() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_N() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_O() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_P() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_Q() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 9;
    Sprite::new(bitmap, width, height)
}
pub fn big_R() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_S() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_T() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 0, 1, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_U() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_V() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_W() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_X() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_Y() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 0, 0, 0],
    ];
    let width = 9;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_Z() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}

pub fn big_a() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_b() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 1, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_c() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_d() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0]
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_e() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_f() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_g() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
    ];
    let width = 8;
    let height = 9;
    Sprite::new(bitmap, width, height)
}
pub fn big_h() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_i() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_j() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0],
    ];
    let width = 7;
    let height = 9;
    Sprite::new(bitmap, width, height)
}
pub fn big_k() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_l() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_m() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_n() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_o() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_p() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 9;
    Sprite::new(bitmap, width, height)
}
pub fn big_q() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 9;
    Sprite::new(bitmap, width, height)
}
pub fn big_r() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_s() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_t() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0],
        vec![0, 0, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_u() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_v() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_w() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_x() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 1, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_y() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 0],
    ];
    let width = 8;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_z() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}

pub fn big_zero() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_one() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_two() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_three() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_four() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_five() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_six() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_seven() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_eight() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}
pub fn big_nine() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0],
    ];
    let width = 7;
    let height = 8;
    Sprite::new(bitmap, width, height)
}

pub fn big_fs() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![1, 1, 0],
        vec![1, 1, 0],
    ];
    let width = 3;
    let height = 8;
    Sprite::new(bitmap, width, height)
}

pub fn big_space() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];
    let width = 4;
    let height = 8;
    Sprite::new(bitmap, width, height)
}

pub fn little_A() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_B() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_C() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_D() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_E() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_F() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_G() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_H() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_I() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_J() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_K() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_L() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_M() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_N() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 0, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_O() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_P() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_Q() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_R() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_S() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_T() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_U() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_V() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_W() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_X() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_Y() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_Z() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}

pub fn little_zero() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0, 0],
    ];
    let width = 6;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_one() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0],
        vec![1, 1, 0],
        vec![0, 1, 0],
        vec![0, 1, 0],
        vec![0, 1, 0],
        vec![0, 1, 0],
    ];
    let width = 3;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_two() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_three() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_four() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 0],
        vec![1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_five() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_six() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_seven() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_eight() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}
pub fn little_nine() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
    ];
    let width = 5;
    let height = 6;
    Sprite::new(bitmap, width, height)
}

pub fn little_fs() -> Sprite {
    let bitmap = vec![
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![1, 0],
    ];
    let width = 3;
    let height = 6;
    Sprite::new(bitmap, width, height)
}

pub fn little_space() -> Sprite {
    let bitmap = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ];
    let width = 3;
    let height = 6;
    Sprite::new(bitmap, width, height)
}