use rand::{rngs::ThreadRng, Rng};
use solve::solve::*;
use stage::*;
use wasm_bindgen::prelude::*;

mod solve;
mod stage;
mod test;

#[wasm_bindgen]
pub fn make_puzzle_web(size_r: isize, size_c: isize) -> String {
    let stage = make_puzzle(size_r, size_c);
    let mut ret = "".to_string();
    for r in 0..size_r {
        if r != 0 {
            ret += "|";
        }
        for c in 0..size_c {
            ret += match number!(stage, r, c) {
                -1 => " ",
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                _ => "",
            }
        }
    }

    return ret;
}

pub fn make_puzzle(size_r: isize, size_c: isize) -> SolvingStage {
    // make phase
    let mut stage = SolvingStage::new(size_r, size_c);
    make_puzzle_unit(&mut stage, &mut 1);
    return stage;
}

const MP_NUMBER_TRY_COUNT: i32 = 4;

fn make_puzzle_unit(stage: &mut SolvingStage, solve_depth: &mut i32) -> SolveResult {
    let result = solve(stage, *solve_depth, 0);
    let mut rng: ThreadRng = rand::thread_rng();

    if result.no_answer {
        // no answer
        return result;
    } else if result.has_answer && !result.multi_answer && !result.over_depth {
        // unieque answer
        return result;
    } else {
        // multi answer or over depth.

        let join_stage = |stage1: &mut SolvingStage, stage2: &SolvingStage| {
            for r2 in 0..stage1.size_r {
                for c2 in 0..stage1.size_c {
                    number!(stage1, r2, c2) = number!(stage2, r2, c2);
                }
            }
        };

        // here's double loop
        //   1st-level-loop: make new stage
        //   2nd-level-loop: add new number
        // After check new_stage,
        //  - no answer (NO_ANS)
        //    => set another number.
        //       If iteration exceeds MP_NUMBER_TRY_COUNT,
        //       return to 1-st level loop.
        //  - unique answer (UQ_ANS)
        //    => add new_stage numbers to stage number,
        //       and return to caller of this function.
        //  - multi answer (MUL_ANS)
        //    => add new_stage numbers to stage number,
        //       and return to 1-st level loop.
        //  - over depth (OVER_DEPTH)
        //    => return to 2-nd level loop in order to new number.

        loop {
            // over depth / multi answer
            let mut new_stage = stage.clone();
            let mut try_count = 0;

            loop {
                // Add new number
                let r = rng.gen_range(0..stage.size_r);
                let c = rng.gen_range(0..stage.size_c);

                if number!(stage, r, c) != -1 {
                    continue;
                }

                // check posibility
                let mut line_count = 0;
                let mut no_line_count = 0;
                for rot in 0..4 {
                    let ln = rot_rc(idx_v_line(r, c), idx_number(r, c), rot);
                    match value!(stage, ln.r, ln.c) {
                        1 => line_count += 1,
                        0 => no_line_count += 1,
                        _ => {}
                    }
                }

                if line_count + no_line_count == 4 {
                    continue;
                }

                let n = rng.gen_range(line_count..(4 - no_line_count));
                number!(new_stage, r, c) = n;
                new_stage.first_check_number(r, c);
                let new_result = make_puzzle_unit(&mut new_stage, solve_depth);

                if new_result.no_answer {
                    // no answer (NO_ANS)
                    try_count += 1;
                    if try_count <= MP_NUMBER_TRY_COUNT {
                        break;
                    } else {
                        *solve_depth += 1;
                        return SolveResult {
                            has_answer: false,
                            multi_answer: false,
                            no_answer: true,
                            over_depth: false,
                        };
                    }
                } else if new_result.has_answer
                    && !new_result.multi_answer
                    && !new_result.over_depth
                {
                    // unique answer (UQ_ANS)
                    join_stage(stage, &new_stage);
                    return new_result;
                } else if new_result.has_answer {
                    // multi answer (MUL_ANS)
                    join_stage(stage, &new_stage);
                    break;
                } else {
                    // over depth (OVER_DEPTH)
                    continue;
                }
            }
        }
    }
}
