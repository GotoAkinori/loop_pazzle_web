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

#[wasm_bindgen]
pub fn make_puzzle_web2(size_r: isize, size_c: isize) -> String {
    let stage = make_puzzle2(size_r, size_c, &vec![2, 2], &5);
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
    let mut depth = 1;
    make_puzzle_unit(&mut stage, &mut depth);

    println!("Depth: {}", depth);

    return stage;
}

const MP_NUMBER_TRY_COUNT: i32 = 4;
const MAX_DEPTH: i32 = 3;

fn make_puzzle_unit(stage: &mut SolvingStage, solve_depth: &mut i32) -> SolveResult {
    let result = solve(stage, *solve_depth, 0);
    // let result = solve_roughly(stage, 0, 10, &vec![3; 3], &5);
    let mut rng: ThreadRng = rand::thread_rng();

    if result.no_answer {
        // no answer
        return result;
    } else if result.has_answer && !result.multi_answer && !result.over_depth {
        // unieque answer
        return result;
    } else {
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

        let mut try_count = 0;

        loop {
            // over depth / multi answer
            let mut new_stage = stage.clone();

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
                        if *solve_depth < MAX_DEPTH {
                            *solve_depth += 1;
                        }

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
                    stage.marge(&new_stage);
                    return new_result;
                } else if new_result.has_answer {
                    // multi answer (MUL_ANS)
                    stage.marge(&new_stage);
                    try_count = 0;
                    break;
                } else {
                    // over depth (OVER_DEPTH)
                    continue;
                }
            }
        }
    }
}

pub fn make_puzzle2(
    size_r: isize,
    size_c: isize,
    solve_roughly_try_count_list: &Vec<usize>,
    solve_roughly_check_count: &usize,
) -> SolvingStage {
    // constants
    let mut rng: ThreadRng = rand::thread_rng();
    let stage_answer: SolvingStage;

    let min_blocks = size_r * size_c / 2;
    let max_blocks = size_r * size_c * 2 / 3;
    let arround_block: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    // make answer and full number game
    'make_full_number_game: loop {
        // blocks crrespondes to area inside line of a loop puzzle.
        let mut blocks: Vec<Vec<bool>> = vec![vec![false; size_c as usize]; size_r as usize];
        fn block_at(
            blocks: &Vec<Vec<bool>>,
            size_r: &isize,
            size_c: &isize,
            r: isize,
            c: isize,
            dr: isize,
            dc: isize,
        ) -> bool {
            return match r + dr < 0 || r + dr >= *size_r || c + dc < 0 || c + dc >= *size_c {
                true => false,
                false => (&blocks)[(r + dr) as usize][(c + dc) as usize],
            };
        }
        fn block_arround_at(
            blocks: &Vec<Vec<bool>>,
            arround_block: &Vec<(isize, isize)>,
            size_r: &isize,
            size_c: &isize,
            r: isize,
            c: isize,
            i: usize,
        ) -> bool {
            let (dr, dc) = (&arround_block)[i];
            block_at(blocks, size_r, size_c, r, c, dr, dc)
        }

        // set blocks.
        blocks[rng.gen_range(0..size_r) as usize][rng.gen_range(0..size_c) as usize] = true;
        'set_blocks: for iter in 0..max_blocks {
            let mut candidacy: Vec<(isize, isize)> = vec![];
            if iter > min_blocks {
                // add dummy
                candidacy.push((-1, -1));
            }

            for r in 0..size_r {
                for c in 0..size_c {
                    if !blocks[r as usize][c as usize] {
                        let mut block_around_switch_count = 0;
                        let mut block_around_sharing_line_count = 0;

                        for i1 in 0..arround_block.len() {
                            let i2: usize = (i1 + 1) % arround_block.len();
                            if block_arround_at(&blocks, &arround_block, &size_r, &size_c, r, c, i1)
                            {
                                if i1 % 2 == 1 {
                                    block_around_sharing_line_count += 1;
                                }
                                if !block_arround_at(
                                    &blocks,
                                    &arround_block,
                                    &size_r,
                                    &size_c,
                                    r,
                                    c,
                                    i2,
                                ) {
                                    block_around_switch_count += 1;
                                }
                            }
                        }

                        if block_around_switch_count == 1 {
                            match block_around_sharing_line_count {
                                1 => {
                                    // surrounded by only ONE block => this block should be picked in high possibility.
                                    candidacy.push((r, c));
                                    candidacy.push((r, c));
                                    candidacy.push((r, c));
                                }
                                2 => {
                                    // surrounding by 2 blocks => this block should be picked in low possibility.
                                    candidacy.push((r, c));
                                }
                                // surrounding by 3 or 4 blocks => this block should not be picked.
                                _ => (),
                            }
                        }
                    }
                }
            }

            if candidacy.len() == 0 {
                break 'set_blocks;
            }
            let rand_index = rng.gen_range(0..candidacy.len());
            let (tr, tc) = candidacy[rand_index];

            if tr == -1 {
                break;
            } else {
                blocks[tr as usize][tc as usize] = true;
            }
        }

        // translate blocks to puzzle.
        let mut stage_temp: SolvingStage = SolvingStage::new(size_r, size_c);
        for (r, c) in stage_temp.get_numbers_pos().into_iter() {
            let mut count = 0;
            let block_at_center = block_at(&blocks, &size_r, &size_c, r, c, 0, 0);
            count += match block_at_center != block_at(&blocks, &size_r, &size_c, r, c, -1, 0) {
                true => 1,
                false => 0,
            };
            count += match block_at_center != block_at(&blocks, &size_r, &size_c, r, c, 1, 0) {
                true => 1,
                false => 0,
            };
            count += match block_at_center != block_at(&blocks, &size_r, &size_c, r, c, 0, -1) {
                true => 1,
                false => 0,
            };
            count += match block_at_center != block_at(&blocks, &size_r, &size_c, r, c, 0, 1) {
                true => 1,
                false => 0,
            };
            number!(stage_temp, r, c) = count;
        }

        // solve puzzle. if it has multiple or too difficult(means depth > PUZZLE_MAX_DEPTH) answers remake puzzle again.
        stage_temp.first_check();
        let result = solve_roughly(
            &mut stage_temp,
            0,
            50,
            solve_roughly_try_count_list,
            solve_roughly_check_count,
        );
        if result.multi_answer || result.over_depth {
            continue 'make_full_number_game;
        } else {
            stage_answer = stage_temp;
            break;
        }
    }

    // drop numbers
    let mut last_ok_stage = stage_answer;
    fn get_dropped_stage(
        stage: &SolvingStage,
        ratio: f64,
        rng: &mut ThreadRng,
    ) -> Option<SolvingStage> {
        let mut dropped_stage = SolvingStage::new(stage.size_r, stage.size_c);
        let mut dropped = false;
        // copy numbers
        for (r, c) in stage.get_numbers_pos().into_iter() {
            if number!(stage, r, c) == -1 {
                number!(dropped_stage, r, c) = -1;
            } else {
                number!(dropped_stage, r, c) = match rng.gen_bool(ratio) {
                    true => {
                        dropped = true;
                        -1
                    }
                    false => number!(stage, r, c),
                };
            }
        }
        return if dropped { Some(dropped_stage) } else { None };
    }
    let mut drop_ratio = 0.25;
    let mut try_count = 0;
    'droping: loop {
        let dropped_stage_opt = get_dropped_stage(&last_ok_stage, drop_ratio, &mut rng);
        match dropped_stage_opt {
            Some(mut dropped_stage) => {
                dropped_stage.first_check();
                let result = solve_roughly(
                    &mut dropped_stage,
                    0,
                    50,
                    solve_roughly_try_count_list,
                    solve_roughly_check_count,
                );

                if result.multi_answer || result.over_depth {
                    drop_ratio = drop_ratio * 0.5;
                    try_count += 1;
                    if try_count > 5 {
                        break;
                    } else {
                        continue;
                    }
                } else {
                    last_ok_stage = dropped_stage;
                    try_count = 0;
                    continue;
                }
            }
            _ => {
                break 'droping;
            }
        }
    }

    return last_ok_stage;
}
