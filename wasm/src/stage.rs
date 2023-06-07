// =======================
// Variables
// =======================

const K_POINT_POS: u32 = 1;
const K_LINE: u32 = 3;

pub const P_NONE: i8 = 1;
pub const P_UR: i8 = 2;
pub const P_UD: i8 = 4;
pub const P_UL: i8 = 8;
pub const P_RD: i8 = 16;
pub const P_RL: i8 = 32;
pub const P_DL: i8 = 64;

pub const P_ALL: i8 = P_NONE | P_UR | P_UD | P_UL | P_RD | P_RL | P_DL;

//   |or
// --x--
// or|
pub const P_DIAGONAL_RU: i8 = P_UD | P_UL | P_RD | P_RL;

// or|
// --x--
//   |or
pub const P_DIAGONAL_RD: i8 = P_UR | P_UD | P_RL | P_DL;

//   x
// ? + ?
//   ?
pub const P_EDGE_U: i8 = P_RD | P_RL | P_DL | P_NONE;

//   ?
// ? + x
//   ?
// pub const P_EDGE_R: i8 = P_UD | P_UL | P_DL | P_NONE;

//   ?
// ? + ?
//   x
// pub const P_EDGE_D: i8 = P_UR | P_UL | P_RL | P_NONE;

//   ?
// x + ?
//   ?
pub const P_EDGE_L: i8 = P_UR | P_UD | P_RD | P_NONE;

//   |
// ? + ?
//   ?
pub const P_LINE_U: i8 = P_UR | P_UD | P_UL;

//   ?
// ? +--
//   ?
pub const P_LINE_R: i8 = P_UR | P_RD | P_RL;

//   ?
// ? + ?
//   |
pub const P_LINE_D: i8 = P_UD | P_RD | P_DL;

//   ?
// --+ ?
//   ?
pub const P_LINE_L: i8 = P_UL | P_RL | P_DL;

// pub const P_OR_UR: i8 = P_LINE_U | P_LINE_R;
// pub const P_OR_RD: i8 = P_LINE_R | P_LINE_D;
// pub const P_OR_DL: i8 = P_LINE_D | P_LINE_L;
pub const P_OR_UL: i8 = P_LINE_U | P_LINE_L;
// pub const P_CORNER_UR: i8 = P_EDGE_U & P_EDGE_R;
// pub const P_CORNER_RD: i8 = P_EDGE_R & P_EDGE_D;
// pub const P_CORNER_DL: i8 = P_EDGE_D & P_EDGE_L;
pub const P_CORNER_UL: i8 = P_EDGE_U & P_EDGE_L;

pub const ALL_P: [i8; 7] = [P_NONE, P_UR, P_UD, P_UL, P_RD, P_RL, P_DL];

// =======================
// Structs
// =======================

// (Row, Col)
#[derive(Clone, Copy, Debug)]
pub struct RowCol {
    pub r: isize,
    pub c: isize,
}

#[derive(Clone)]
pub struct SolvingStage {
    pub stage: Vec<Vec<i8>>,
    pub size_r: isize,
    pub size_c: isize,
    pub group: Vec<Vec<i64>>,
    pub result: bool,
    // 0: not checked, 1: checked but not completed, 2:completed.
    pub checked: Vec<Vec<u8>>,
    pub score: u32,
}

pub fn idx_point(r: isize, c: isize) -> RowCol {
    return RowCol { r: r * 2, c: c * 2 };
}
pub fn idx_v_line(r: isize, c: isize) -> RowCol {
    return RowCol {
        r: r * 2 + 1,
        c: c * 2,
    };
}
pub fn idx_h_line(r: isize, c: isize) -> RowCol {
    return RowCol {
        r: r * 2,
        c: c * 2 + 1,
    };
}
pub fn idx_number(r: isize, c: isize) -> RowCol {
    return RowCol {
        r: r * 2 + 1,
        c: c * 2 + 1,
    };
}

#[macro_export]
macro_rules! value {
    ($self:ident, $r:expr, $c:expr) => {
        $self.stage[($r) as usize][($c) as usize]
    };
}

#[macro_export]
macro_rules! point {
    ($self:ident, $r:expr, $c:expr) => {
        $self.stage[(($r) * 2) as usize][(($c) * 2) as usize]
    };
}

#[macro_export]
macro_rules! v_line {
    ($self:ident, $r:expr, $c:expr) => {
        $self.stage[(($r) * 2 + 1) as usize][(($c) * 2) as usize]
    };
}

#[macro_export]
macro_rules! h_line {
    ($self:ident, $r:expr, $c:expr) => {
        $self.stage[(($r) * 2) as usize][(($c) * 2 + 1) as usize]
    };
}

#[macro_export]
macro_rules! number {
    ($self:ident, $r:expr, $c:expr) => {
        $self.stage[(($r) * 2 + 1) as usize][(($c) * 2 + 1) as usize]
    };
}

impl SolvingStage {
    pub fn new(size_r: isize, size_c: isize) -> SolvingStage {
        let mut stage = SolvingStage {
            stage: vec![vec![-1; (size_c * 2 + 1) as usize]; (size_r * 2 + 1) as usize],
            size_c,
            size_r,
            group: vec![vec![0; size_c as usize]; size_r as usize],
            result: true,
            checked: vec![vec![0; size_c as usize]; size_r as usize],
            score: 0,
        };

        for r in 0..size_r {
            for c in 0..size_c {
                stage.group[r as usize][c as usize] = (r * size_c + c + 2) as i64;
            }
        }

        for r in 0..size_r + 1 {
            for c in 0..size_c + 1 {
                point!(stage, r, c) = P_ALL;
            }
        }

        return stage;
    }

    pub fn dump(&self) {
        println!("===============================");
        println!("");
        for r in 0..(self.size_r + 1) {
            let mut line1 = "+".to_string();
            for c in 0..(self.size_c) {
                line1 += match h_line!(self, r, c) {
                    -1 => "   +",
                    0 => " x +",
                    1 => "---+",
                    _ => " ? +",
                }
            }
            println!("{line1}");

            if r < self.size_r {
                let mut line2: String = String::from("");
                for c in 0..(self.size_c + 1) {
                    line2 += match v_line!(self, r, c) {
                        -1 => " ",
                        0 => "x",
                        1 => "|",
                        _ => "?",
                    };

                    if c < self.size_c {
                        line2 += match number!(self, r, c) {
                            -1 => "   ",
                            0 => " 0 ",
                            1 => " 1 ",
                            2 => " 2 ",
                            3 => " 3 ",
                            _ => " ? ",
                        };
                    }
                }

                println!("{line2}");
            }
        }
    }

    #[cfg(test)]
    pub fn dump_group(&mut self) {
        println!("===============================");
        println!("");
        for r in 0..(self.size_r + 1) {
            let mut line1 = "+".to_string();
            for c in 0..(self.size_c) {
                line1 += match h_line!(self, r, c) {
                    -1 => "    +",
                    0 => " x  +",
                    1 => "----+",
                    _ => " ?  +",
                }
            }
            println!("{line1}");

            if r < self.size_r {
                let mut line2: String = String::from("");
                for c in 0..(self.size_c + 1) {
                    line2 += match v_line!(self, r, c) {
                        -1 => " ",
                        0 => "x",
                        1 => "|",
                        _ => "?",
                    };

                    if c < self.size_c {
                        let mut num = self.group[r as usize][c as usize].to_string();
                        num += match num.chars().count() {
                            1 => "   ",
                            2 => "  ",
                            3 => " ",
                            _ => "",
                        };
                        line2 += &num;
                    }
                }

                println!("{line2}");
            }
        }
    }

    pub fn marge(&mut self, stage: &Self) {
        for r in 0..(self.size_r) {
            for c in 0..(self.size_c) {
                if number!(self, r, c) != number!(stage, r, c) && number!(self, r, c) == -1 {
                    number!(self, r, c) = number!(stage, r, c);
                    self.checked[r as usize][c as usize] = 0;
                }
            }
        }
        for r in 0..(self.size_r) {
            for c in 0..(self.size_c + 1) {
                if v_line!(self, r, c) != v_line!(stage, r, c) && v_line!(self, r, c) == -1 {
                    let line_pos = idx_v_line(r, c);
                    self.set_line(line_pos, v_line!(self, r, c));
                }
            }
        }
        for r in 0..(self.size_r + 1) {
            for c in 0..(self.size_c) {
                if h_line!(self, r, c) != h_line!(stage, r, c) && h_line!(self, r, c) == -1 {
                    let line_pos = idx_h_line(r, c);
                    self.set_line(line_pos, h_line!(self, r, c));
                }
            }
        }
        for r in 0..(self.size_r + 1) {
            for c in 0..(self.size_c + 1) {
                if point!(self, r, c) != point!(stage, r, c) {
                    let pos = idx_point(r, c);
                    self.restrict_point_possibility_bat(pos, point!(stage, r, c));
                }
            }
        }
    }

    pub fn get_lines_pos(&self) -> Vec<(isize, isize, bool)> {
        let mut lines: Vec<(isize, isize, bool)> = vec![];

        for r in 0..(self.size_r) {
            for c in 0..(self.size_c + 1) {
                lines.push((r, c, true));
            }
        }
        for r in 0..(self.size_r + 1) {
            for c in 0..(self.size_c) {
                lines.push((r, c, false));
            }
        }

        return lines;
    }

    pub fn get_numbers_pos(&self) -> Vec<(isize, isize)> {
        let mut nums: Vec<(isize, isize)> = vec![];

        for r in 0..(self.size_r) {
            for c in 0..(self.size_c) {
                nums.push((r, c));
            }
        }

        return nums;
    }

    pub fn get_points_pos(&self) -> Vec<(isize, isize)> {
        let mut nums: Vec<(isize, isize)> = vec![];

        for r in 0..(self.size_r) {
            for c in 0..(self.size_c) {
                nums.push((r, c));
            }
        }

        return nums;
    }

    #[cfg(test)]
    pub fn dump_assert(&self) {
        // assert v_line
        for r in 0..(self.size_r) {
            for c in 0..(self.size_c + 1) {
                if v_line!(self, r, c) == 1 {
                    println!("debug_assert!(v_line!(stage, {r}, {c}) == 1);");
                } else if v_line!(self, r, c) == 0 {
                    println!("debug_assert!(v_line!(stage, {r}, {c}) == 0);");
                }
            }
        }

        // assert h_line
        for r in 0..(self.size_r + 1) {
            for c in 0..(self.size_c) {
                if h_line!(self, r, c) == 1 {
                    println!("debug_assert!(h_line!(stage, {r}, {c}) == 1);");
                } else if h_line!(self, r, c) == 0 {
                    println!("debug_assert!(h_line!(stage, {r}, {c}) == 0);");
                }
            }
        }

        // assert point
        for r in 0..(self.size_r + 1) {
            for c in 0..(self.size_c + 1) {
                if point!(self, r, c) != P_ALL {
                    println!(
                        "debug_assert!(point!(stage, {r}, {c}) == {});",
                        p_dump(point!(self, r, c))
                    );
                }
            }
        }
    }
}

#[cfg(test)]
pub fn p_dump(p: i8) -> String {
    let mut p_vec: Vec<&str> = vec![];
    if p & P_UR == P_UR {
        p_vec.push("P_UR");
    }
    if p & P_UD == P_UD {
        p_vec.push("P_UD");
    }
    if p & P_UL == P_UL {
        p_vec.push("P_UL");
    }
    if p & P_RD == P_RD {
        p_vec.push("P_RD");
    }
    if p & P_RL == P_RL {
        p_vec.push("P_RL");
    }
    if p & P_DL == P_DL {
        p_vec.push("P_DL");
    }
    if p & P_NONE == P_NONE {
        p_vec.push("P_NONE");
    }
    return p_vec.join(" | ");
}

// =======================
// Rotation
// =======================

#[inline]
pub fn rot_p(p: i8, rot: u8) -> i8 {
    fn map(p: i8) -> i8 {
        match p {
            P_NONE => P_NONE,
            P_UR => P_RD,
            P_UD => P_RL,
            P_UL => P_UR,
            P_RD => P_DL,
            P_RL => P_UD,
            P_DL => P_UL,
            _ => P_NONE,
        }
    }

    fn map_rep(p: i8, rot: u8) -> i8 {
        let mut p2 = p;
        for _ in 0..(rot % 4) {
            p2 = map(p2);
        }
        return p2;
    }

    let mut res = 0;
    for p2 in ALL_P {
        if p2 & p == p2 {
            res |= map_rep(p2, rot);
        }
    }
    return res;
}

#[inline]
pub fn rot_rc(target: RowCol, center: RowCol, rot: u8) -> RowCol {
    let dr: isize = target.r - center.r;
    let dc: isize = target.c - center.c;

    return RowCol {
        r: match rot % 4 {
            0 => center.r + dr,
            1 => center.r + dc,
            2 => center.r - dr,
            3 => center.r - dc,
            _ => center.r,
        },
        c: match rot % 4 {
            0 => center.c + dc,
            1 => center.c - dr,
            2 => center.c - dc,
            3 => center.c + dr,
            _ => center.c,
        },
    };
}

// =======================
// Solve Unit
// =======================

pub fn in_possibility(x: i8, p: i8) -> bool {
    return (x & p) == x;
}

impl SolvingStage {
    // ==================== Set / Add / Remove

    pub fn remove_point_possibility(&mut self, target: RowCol, p: i8) -> u32 {
        if (value!(self, target.r, target.c) & p) == p {
            value!(self, target.r, target.c) &= !p;

            if target.r < self.size_r * 2
                && target.c < self.size_c * 2
                && self.checked[(target.r / 2) as usize][(target.c / 2) as usize] == 1
            {
                self.checked[(target.r / 2) as usize][(target.c / 2) as usize] = 0;
            }
            if target.r < self.size_r * 2
                && target.c > 0
                && self.checked[(target.r / 2) as usize][(target.c / 2 - 1) as usize] == 1
            {
                self.checked[(target.r / 2) as usize][(target.c / 2 - 1) as usize] = 0;
            }
            if target.r > 0
                && target.c < self.size_c * 2
                && self.checked[(target.r / 2 - 1) as usize][(target.c / 2) as usize] == 1
            {
                self.checked[(target.r / 2 - 1) as usize][(target.c / 2) as usize] = 0;
            }
            if target.c > 0
                && target.r > 0
                && self.checked[(target.r / 2 - 1) as usize][(target.c / 2 - 1) as usize] == 1
            {
                self.checked[(target.r / 2 - 1) as usize][(target.c / 2 - 1) as usize] = 0;
            }

            self.score += K_POINT_POS;
            return K_POINT_POS;
        } else {
            return 0;
        }
    }

    pub fn remove_point_possibility_bat(&mut self, target: RowCol, p: i8) -> u32 {
        let mut counter: u32 = 0;
        for p2 in ALL_P {
            if (p & p2) == p2 {
                counter += self.remove_point_possibility(target, p2);
            }
        }
        return counter;
    }

    pub fn restrict_point_possibility_bat(&mut self, target: RowCol, p: i8) -> u32 {
        return self.remove_point_possibility_bat(target, P_ALL & !p);
    }

    pub fn set_line(&mut self, target: RowCol, line: i8) -> u32 {
        if value!(self, target.r, target.c) == -1 {
            value!(self, target.r, target.c) = line;

            if target.r % 2 == 1 {
                // virtical line
                if target.c < self.size_c * 2
                    && self.checked[((target.r - 1) / 2) as usize][(target.c / 2) as usize] == 1
                {
                    self.checked[((target.r - 1) / 2) as usize][(target.c / 2) as usize] = 0;
                }
                if target.c > 0
                    && self.checked[((target.r - 1) / 2) as usize][(target.c / 2 - 1) as usize] == 1
                {
                    self.checked[((target.r - 1) / 2) as usize][(target.c / 2 - 1) as usize] = 0;
                }
            } else {
                // horizontal line
                if target.r < self.size_r * 2
                    && self.checked[(target.r / 2) as usize][((target.c - 1) / 2) as usize] == 1
                {
                    self.checked[(target.r / 2) as usize][((target.c - 1) / 2) as usize] = 0;
                }
                if target.r > 0
                    && self.checked[(target.r / 2 - 1) as usize][((target.c - 1) / 2) as usize] == 1
                {
                    self.checked[(target.r / 2 - 1) as usize][((target.c - 1) / 2) as usize] = 0;
                }
            }

            self.score += K_LINE;
            return K_LINE;
        } else if value!(self, target.r, target.c) == line {
            return 0;
        } else {
            self.result = false;
            return 0;
        }
    }

    // ==================== Check

    pub fn in_bound(&self, rc: RowCol) -> bool {
        return rc.r >= 0 && rc.r < self.size_r * 2 + 1 && rc.c >= 0 && rc.c < self.size_c * 2 + 1;
    }
    pub fn first_check_number(&mut self, r: isize, c: isize) {
        let center = idx_number(r, c);

        if number!(self, r, c) == 0 {
            //  o---+
            //  | 0 |
            //  +---+
            for rot in 0..4 {
                self.remove_point_possibility_bat(
                    rot_rc(idx_point(r, c), center, rot),
                    rot_p(P_LINE_R | P_LINE_D, rot),
                );
                v_line!(self, r, c) = 0;
                v_line!(self, r, c + 1) = 0;
                h_line!(self, r, c) = 0;
                h_line!(self, r + 1, c) = 0;
            }
            self.checked[r as usize][c as usize] = 2;
        } else if number!(self, r, c) == 1 {
            //  o---+
            //  | 1 |
            //  +---+
            for rot in 0..4 {
                self.remove_point_possibility(
                    rot_rc(idx_point(r, c), center, rot),
                    rot_p(P_RD, rot),
                );
            }
        } else if number!(self, r, c) == 3 {
            //  o---+
            //  | 3 |
            //  +---+
            for rot in 0..4 {
                self.remove_point_possibility_bat(
                    rot_rc(idx_point(r, c), center, rot),
                    rot_p(P_UL | P_NONE, rot),
                );
            }
        }
    }

    pub fn first_check_point_up(&mut self, c: isize) -> u32 {
        return self.remove_point_possibility_bat(idx_point(0, c), P_LINE_U);
    }

    pub fn first_check_point_down(&mut self, c: isize) -> u32 {
        return self.remove_point_possibility_bat(idx_point(self.size_r, c), P_LINE_D);
    }

    pub fn first_check_point_left(&mut self, r: isize) -> u32 {
        return self.remove_point_possibility_bat(idx_point(r, 0), P_LINE_L);
    }

    pub fn first_check_point_right(&mut self, r: isize) -> u32 {
        return self.remove_point_possibility_bat(idx_point(r, self.size_c), P_LINE_R);
    }

    pub fn check_number(&mut self, r: isize, c: isize) -> u32 {
        let mut counter: u32 = 0;
        let num = number!(self, r, c);
        let center = idx_number(r, c);
        if self.checked[r as usize][c as usize] != 0 {
            return 0;
        }

        {
            // check if line is filled around the number
            if num == 1 || num == 2 || num == 3 {
                let mut line_count = 0;
                if v_line!(self, r, c) == 1 {
                    line_count += 1;
                }
                if v_line!(self, r, c + 1) == 1 {
                    line_count += 1;
                }
                if h_line!(self, r, c) == 1 {
                    line_count += 1;
                }
                if h_line!(self, r + 1, c) == 1 {
                    line_count += 1;
                }

                if line_count > num {
                    self.result = false;
                } else if line_count == num {
                    if v_line!(self, r, c) == -1 {
                        let ln = idx_v_line(r, c);
                        counter += self.set_line(ln, 0);
                    }
                    if v_line!(self, r, c + 1) == -1 {
                        let ln = idx_v_line(r, c + 1);
                        counter += self.set_line(ln, 0);
                    }
                    if h_line!(self, r, c) == -1 {
                        let ln = idx_h_line(r, c);
                        counter += self.set_line(ln, 0);
                    }
                    if h_line!(self, r + 1, c) == -1 {
                        let ln = idx_h_line(r + 1, c);
                        counter += self.set_line(ln, 0);
                    }
                }
            }
        }

        {
            // check if none-line is filled around the number
            if num == 1 || num == 2 || num == 3 {
                let mut none_line_count = 0;
                if v_line!(self, r, c) == 0 {
                    none_line_count += 1;
                }
                if v_line!(self, r, c + 1) == 0 {
                    none_line_count += 1;
                }
                if h_line!(self, r, c) == 0 {
                    none_line_count += 1;
                }
                if h_line!(self, r + 1, c) == 0 {
                    none_line_count += 1;
                }

                if none_line_count > 4 - num {
                    self.result = false;
                } else if none_line_count == 4 - num {
                    if v_line!(self, r, c) == -1 {
                        let ln = idx_v_line(r, c);
                        counter += self.set_line(ln, 1);
                    }
                    if v_line!(self, r, c + 1) == -1 {
                        let ln = idx_v_line(r, c + 1);
                        counter += self.set_line(ln, 1);
                    }
                    if h_line!(self, r, c) == -1 {
                        let ln = idx_h_line(r, c);
                        counter += self.set_line(ln, 1);
                    }
                    if h_line!(self, r + 1, c) == -1 {
                        let ln = idx_h_line(r + 1, c);
                        counter += self.set_line(ln, 1);
                    }
                }
            }
        }

        {
            if num == 1 {
                for rot in 0..4 {
                    // ?   +      ?   +
                    //   1    >>    1 x
                    // +   +      + x +
                    let pt = rot_rc(idx_point(r, c), center, rot);

                    if in_possibility(value!(self, pt.r, pt.c), rot_p(P_DIAGONAL_RD, rot)) {
                        counter += self.set_line(rot_rc(idx_h_line(r + 1, c), center, rot), 0);
                        counter += self.set_line(rot_rc(idx_v_line(r, c + 1), center, rot), 0);
                    }

                    // + x +      + x +
                    // x 1    >>  x 1
                    // +   +      +  xor
                    let ln1 = rot_rc(idx_h_line(r, c), center, rot);
                    let ln2 = rot_rc(idx_v_line(r, c), center, rot);
                    if value!(self, ln1.r, ln1.c) == 0 && value!(self, ln2.r, ln2.c) == 0 {
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r + 1, c + 1), center, rot),
                            rot_p(P_DIAGONAL_RD, rot),
                        );
                    }
                }
            } else if num == 2 {
                for rot in 0..4 {
                    let pt = rot_rc(idx_point(r, c), center, rot);

                    // xor  +     xor  +
                    //    2    >>    2
                    //  +   +      +  xor
                    if in_possibility(value!(self, pt.r, pt.c), rot_p(P_DIAGONAL_RD, rot)) {
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r + 1, c + 1), center, rot),
                            rot_p(P_DIAGONAL_RD, rot),
                        );
                    }
                    // p   +      p   +  :  or|
                    //   2    >>    2    : ---p---
                    // +   +      +   p  :    | 2
                    else if in_possibility(value!(self, pt.r, pt.c), rot_p(P_OR_UL, rot)) {
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r + 1, c + 1), center, rot),
                            rot_p(P_OR_UL, rot),
                        )
                    }
                    // @   +      @  xor :    x         |
                    //   2    >>    2    :  x @ ?  or --@ x
                    // +   +     xor  @  :    ? 2       x 2
                    else if in_possibility(
                        value!(self, pt.r, pt.c),
                        rot_p(P_CORNER_UL | P_UL, rot),
                    ) {
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r + 1, c + 1), center, rot),
                            rot_p(P_UL | P_RD | P_NONE, rot),
                        );
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r + 1, c), center, rot),
                            rot_p(P_DIAGONAL_RU, rot),
                        );
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r, c + 1), center, rot),
                            rot_p(P_DIAGONAL_RU, rot),
                        );
                    }
                }
            } else if num == 3 {
                for rot in 0..4 {
                    let pt = rot_rc(idx_point(r, c), center, rot);
                    //  p   +     xor  +
                    //    3    >>    3 |
                    //  +   +      +---+
                    if in_possibility(value!(self, pt.r, pt.c), rot_p(P_OR_UL, rot)) {
                        counter += self.restrict_point_possibility_bat(
                            rot_rc(idx_point(r, c), center, rot),
                            rot_p(P_DIAGONAL_RD, rot),
                        );
                        counter += self.set_line(rot_rc(idx_v_line(r, c + 1), center, rot), 1);
                        counter += self.set_line(rot_rc(idx_h_line(r + 1, c), center, rot), 1);
                    }
                    // c   +      c---+
                    //   3    >>  | 3
                    // +   +      +   +
                    else if in_possibility(value!(self, pt.r, pt.c), rot_p(P_CORNER_UL, rot)) {
                        counter += self.set_line(rot_rc(idx_v_line(r, c), center, rot), 1);
                        counter += self.set_line(rot_rc(idx_h_line(r, c), center, rot), 1);
                    }
                }
            }
        }

        self.checked[r as usize][c as usize] = 1;

        return counter;
    }

    pub fn check_point(&mut self, r: isize, c: isize) -> u32 {
        let mut counter = 0;
        let center = idx_point(r, c);

        for rot in 0..4 {
            let ln = rot_rc(idx_v_line(r - 1, c), center, rot);
            if ln.r >= 0 && ln.r < self.size_r * 2 + 1 && ln.c >= 0 && ln.c < self.size_c * 2 + 1 {
                //  ?  >>  |
                //  +  >>  +
                if in_possibility(point!(self, r, c), rot_p(P_LINE_U, rot)) {
                    counter += self.set_line(ln, 1);
                }

                //   ?  >>  x
                //   +  >>  +
                if in_possibility(point!(self, r, c), rot_p(P_EDGE_U, rot)) {
                    counter += self.set_line(ln, 0);
                }

                //   |  >>  |
                //   +  >>  *
                if value!(self, ln.r, ln.c) == 1 {
                    counter += self.restrict_point_possibility_bat(center, rot_p(P_LINE_U, rot))
                }

                //   x  >>  x
                //   +  >>  *
                if value!(self, ln.r, ln.c) == 0 {
                    counter += self.restrict_point_possibility_bat(center, rot_p(P_EDGE_U, rot))
                }
            }
        }

        {
            let mut count_line = 0;
            let mut count_x = 0;

            for rot in 0..4 {
                let ln = rot_rc(idx_v_line(r - 1, c), center, rot);
                if !self.in_bound(ln) || value!(self, ln.r, ln.c) == 0 {
                    count_x += 1;
                } else if value!(self, ln.r, ln.c) == 1 {
                    count_line += 1;
                }
            }

            if count_line >= 3 || (count_x == 3 && count_line == 1) {
                self.result = false;
            } else if count_line == 2 {
                for rot in 0..4 {
                    let ln = rot_rc(idx_v_line(r - 1, c), center, rot);
                    if self.in_bound(ln) && value!(self, ln.r, ln.c) == -1 {
                        counter += self.set_line(ln, 0);
                    }
                }
            } else if count_x == 3 {
                for rot in 0..4 {
                    let ln = rot_rc(idx_v_line(r - 1, c), center, rot);
                    if self.in_bound(ln) && value!(self, ln.r, ln.c) == -1 {
                        counter += self.set_line(ln, 0);
                    }
                }
            } else if count_line == 1 && count_x == 2 {
                for rot in 0..4 {
                    let ln = rot_rc(idx_v_line(r - 1, c), center, rot);
                    if self.in_bound(ln) && value!(self, ln.r, ln.c) == -1 {
                        counter += self.set_line(ln, 1);
                    }
                }
            }
        }

        return counter;
    }

    pub fn first_check(&mut self) {
        for r in 0..(self.size_r + 1) {
            self.first_check_point_left(r);
            self.first_check_point_right(r);
        }
        for c in 0..(self.size_c + 1) {
            self.first_check_point_up(c);
            self.first_check_point_down(c);
        }

        for r in 0..self.size_r {
            for c in 0..self.size_c {
                self.first_check_number(r, c);
            }
        }
    }

    pub fn check(&mut self) -> u32 {
        let mut counter = 0;

        loop {
            let mut local_counter = 0;
            for r in 0..self.size_r {
                for c in 0..self.size_c {
                    local_counter += self.check_number(r, c);
                }
            }
            for r in 0..self.size_r + 1 {
                for c in 0..self.size_c + 1 {
                    local_counter += self.check_point(r, c);
                }
            }

            local_counter += self.check_group();
            counter += local_counter;

            if local_counter == 0 {
                break;
            }
        }

        return counter;
    }

    pub fn check_group(&mut self) -> u32 {
        let mut counter = 0;
        fn get_group(stage: &SolvingStage, r: isize, c: isize) -> i64 {
            if r < 0 || r >= stage.size_r || c < 0 || c >= stage.size_c {
                return -1;
            } else {
                return stage.group[r as usize][c as usize];
            }
        }

        fn join_group(stage: &mut SolvingStage, g1: i64, g2: i64) {
            if g1.abs() == g2.abs() {
                return;
            }
            let v1: i64;
            let v2: i64;
            if g1.abs() < g2.abs() {
                v1 = g1;
                v2 = g2;
            } else {
                v1 = g2;
                v2 = g1;
            }

            for r in 0..stage.size_r {
                for c in 0..stage.size_c {
                    if get_group(stage, r, c) == v2 {
                        stage.group[r as usize][c as usize] = v1;
                    } else if get_group(stage, r, c) == -v2 {
                        stage.group[r as usize][c as usize] = -v1;
                    }
                }
            }
        }

        // set group number
        for r in 0..self.size_r {
            for c in 0..self.size_c + 1 {
                //    +
                //  g | h -> -g = h
                //    +
                if v_line!(self, r, c) == 1 {
                    join_group(self, get_group(self, r, c), -get_group(self, r, c - 1));
                }
                //    +
                //  g x h ->  g = h
                //    +
                else if v_line!(self, r, c) == 0 {
                    join_group(self, get_group(self, r, c), get_group(self, r, c - 1));
                }
            }
        }
        for r in 0..self.size_r + 1 {
            for c in 0..self.size_c {
                //    g
                //  +---+ -> -g = h
                //    h
                if h_line!(self, r, c) == 1 {
                    join_group(self, get_group(self, r - 1, c), -get_group(self, r, c));
                }
                //    g
                //  + x + ->  g = h
                //    h
                else if h_line!(self, r, c) == 0 {
                    join_group(self, get_group(self, r - 1, c), get_group(self, r, c));
                }
            }
        }
        for r in 1..self.size_r {
            for c in 1..self.size_c {
                //  g
                //    +   -> -g = h
                //      h
                if in_possibility(point!(self, r, c), P_DIAGONAL_RU) {
                    join_group(self, get_group(self, r - 1, c - 1), -get_group(self, r, c));
                }
                //  g
                //    +   ->  g = h
                //      h
                else if in_possibility(point!(self, r, c), P_DL | P_UR | P_NONE) {
                    join_group(self, get_group(self, r - 1, c - 1), get_group(self, r, c));
                }
                //      g
                //    +   -> -g = h
                //  h
                if in_possibility(point!(self, r, c), P_DIAGONAL_RD) {
                    join_group(self, get_group(self, r, c - 1), -get_group(self, r - 1, c));
                }
                //      g
                //    +   ->  g = h
                //  h
                else if in_possibility(point!(self, r, c), P_UL | P_RD | P_NONE) {
                    join_group(self, get_group(self, r, c - 1), get_group(self, r - 1, c));
                }
            }
        }

        // set lines and points number
        for r in 0..self.size_r {
            for c in 0..self.size_c + 1 {
                //    +
                //  g   -g
                //    +
                if get_group(self, r, c) == -get_group(self, r, c - 1) {
                    counter += self.set_line(idx_v_line(r, c), 1);
                }
                //    +
                //  g   g
                //    +
                if get_group(self, r, c) == get_group(self, r, c - 1) {
                    counter += self.set_line(idx_v_line(r, c), 0);
                }
            }
        }
        for r in 0..self.size_r + 1 {
            for c in 0..self.size_c {
                //    g
                //  +   +
                //   -g
                if get_group(self, r - 1, c) == -get_group(self, r, c) {
                    counter += self.set_line(idx_h_line(r, c), 1);
                }
                //    g
                //  + x + ->  g = h
                //    h
                if get_group(self, r - 1, c) == get_group(self, r, c) {
                    counter += self.set_line(idx_h_line(r, c), 0);
                }
            }
        }
        for r in 1..self.size_r {
            for c in 1..self.size_c {
                //  g
                //    +
                //     -g
                if get_group(self, r - 1, c - 1) == -get_group(self, r, c) {
                    counter += self.restrict_point_possibility_bat(idx_point(r, c), P_DIAGONAL_RU);
                }
                //  g
                //    +
                //      g
                else if get_group(self, r - 1, c - 1) == get_group(self, r, c) {
                    counter +=
                        self.restrict_point_possibility_bat(idx_point(r, c), P_DL | P_UR | P_NONE);
                }
                //      g
                //    +
                // -g
                if get_group(self, r, c - 1) == -get_group(self, r - 1, c) {
                    counter += self.restrict_point_possibility_bat(idx_point(r, c), P_DIAGONAL_RD);
                }
                //      g
                //    +
                //  g
                else if get_group(self, r, c - 1) == get_group(self, r - 1, c) {
                    counter +=
                        self.restrict_point_possibility_bat(idx_point(r, c), P_UL | P_RD | P_NONE);
                }
            }
        }
        return counter;
    }

    pub fn check_completed(
        &mut self,
        checked_v: Vec<Vec<bool>>,
        checked_h: Vec<Vec<bool>>,
    ) -> bool {
        // check numbers
        for r in 0..(self.size_r as usize) {
            for c in 0..(self.size_c as usize) {
                if number!(self, r, c) != -1 {
                    let mut line = 0;
                    if v_line!(self, r, c) == 1 {
                        line += 1;
                    }
                    if h_line!(self, r, c) == 1 {
                        line += 1;
                    }
                    if v_line!(self, r, c + 1) == 1 {
                        line += 1;
                    }
                    if h_line!(self, r + 1, c) == 1 {
                        line += 1;
                    }

                    if line != number!(self, r, c) {
                        return false;
                    }
                }
            }
        }

        // check lines(vertical)
        for r in 0..(self.size_r as usize) {
            for c in 0..(self.size_c + 1) as usize {
                if v_line!(self, r, c) == 1 && !checked_v[r][c] {
                    return false;
                }
            }
        }

        // check lines(horizontal)
        for r in 0..(self.size_r + 1) as usize {
            for c in 0..(self.size_c as usize) {
                if h_line!(self, r, c) == 1 && !checked_h[r][c] {
                    return false;
                }
            }
        }

        return true;
    }

    // complete => reutrn true
    // no error => return false & self.result = true
    // error    => return false & self.result = false
    pub fn check_loop(&mut self) -> bool {
        let mut checked_v: Vec<Vec<bool>> =
            vec![vec![false; (self.size_c + 1) as usize]; self.size_r as usize];

        let mut checked_h: Vec<Vec<bool>> =
            vec![vec![false; self.size_c as usize]; (self.size_r + 1) as usize];

        for r in 0..(self.size_r as usize) {
            for c in 0..(self.size_c + 1) as usize {
                if !checked_v[r][c] && v_line!(self, r, c) == 1 {
                    let mut rot: u8 = 0;
                    let mut r2 = r;
                    let mut c2 = c;
                    checked_v[r][c] = true;

                    loop {
                        let center = idx_point(r2 as isize, c2 as isize);
                        let mut link = false;

                        for rot2 in 3..6 {
                            let ln = rot_rc(
                                idx_v_line(r2 as isize - 1, c2 as isize),
                                center,
                                rot + rot2,
                            );
                            if self.in_bound(ln) && value!(self, ln.r, ln.c) == 1 {
                                rot = (rot + rot2) % 4;
                                link = true;

                                match rot {
                                    0 => {
                                        checked_v[r2 - 1][c2] = true;
                                        r2 -= 1;
                                    }
                                    1 => {
                                        checked_h[r2][c2] = true;
                                        c2 += 1;
                                    }
                                    2 => {
                                        checked_v[r2][c2] = true;
                                        r2 += 1;
                                    }
                                    3 => {
                                        checked_h[r2][c2 - 1] = true;
                                        c2 -= 1;
                                    }
                                    _ => {}
                                }
                                break;
                            }
                        }

                        if !link {
                            break;
                        } else {
                            if r2 == r + 1 && c2 == c {
                                if self.check_completed(checked_v, checked_h) {
                                    return true;
                                } else {
                                    self.result = false;
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        return false;
    }
}
