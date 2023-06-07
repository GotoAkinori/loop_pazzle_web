pub mod solve {
    use crate::{h_line, stage::*, v_line, value};

    #[derive(Clone, Copy, Debug)]
    pub struct SolveResult {
        pub has_answer: bool,
        pub multi_answer: bool,
        pub no_answer: bool,
        pub over_depth: bool,
    }

    pub fn solve(stage: &mut SolvingStage, max_depth: i32, depth: i32) -> SolveResult {
        'top: loop {
            stage.check();
            let complete = stage.check_loop();

            if !stage.result {
                return SolveResult {
                    has_answer: false,
                    multi_answer: false,
                    no_answer: true,
                    over_depth: false,
                };
            } else if complete {
                return SolveResult {
                    has_answer: true,
                    multi_answer: false,
                    no_answer: false,
                    over_depth: false,
                };
            } else {
                if depth == max_depth {
                    return SolveResult {
                        has_answer: false,
                        multi_answer: false,
                        no_answer: false,
                        over_depth: true,
                    };
                }

                let mut has_answer = false;
                let mut over_depth = true;

                let mut v_checked: Vec<Vec<bool>> =
                    vec![vec![false; (stage.size_c + 1) as usize]; (stage.size_r) as usize];

                let mut h_checked: Vec<Vec<bool>> =
                    vec![vec![false; (stage.size_c) as usize]; (stage.size_r + 1) as usize];

                let mut check_result = |new_stage: &SolvingStage,
                                        result: SolveResult,
                                        v_checked: &mut Vec<Vec<bool>>,
                                        h_checked: &mut Vec<Vec<bool>>|
                 -> Option<SolveResult> {
                    if result.over_depth {
                        over_depth = true;
                    }

                    for (r, c, is_v) in new_stage.get_lines_pos().into_iter() {
                        if is_v {
                            v_checked[r as usize][c as usize] = v_line!(new_stage, r, c) != -1;
                        } else {
                            h_checked[r as usize][c as usize] = h_line!(new_stage, r, c) != -1;
                        }
                    }

                    if result.multi_answer {
                        return Some(SolveResult {
                            has_answer: true,
                            multi_answer: true,
                            no_answer: false,
                            over_depth,
                        });
                    } else if result.has_answer {
                        if has_answer {
                            return Some(SolveResult {
                                has_answer: true,
                                multi_answer: true,
                                no_answer: false,
                                over_depth,
                            });
                        } else {
                            has_answer = true;
                            return None;
                        }
                    }
                    return None;
                };

                // assume a line and solve.
                for (r, c, is_v) in stage.get_lines_pos().into_iter() {
                    let line_pos = if is_v {
                        idx_v_line(r, c)
                    } else {
                        idx_h_line(r, c)
                    };

                    if value!(stage, line_pos.r, line_pos.c) != -1
                        || (is_v && v_checked[r as usize][c as usize])
                        || (!is_v && h_checked[r as usize][c as usize])
                    {
                        continue;
                    }

                    // line
                    {
                        let mut new_stage = stage.clone();
                        new_stage.set_line(line_pos, 1);
                        let result = solve(&mut new_stage, max_depth, depth + 1);
                        if result.no_answer {
                            stage.set_line(line_pos, 0);
                            continue 'top;
                        } else {
                            let return_result =
                                check_result(&new_stage, result, &mut v_checked, &mut h_checked);
                            if !return_result.is_none() {
                                return return_result.unwrap();
                            }
                        }
                    }

                    // no-line
                    {
                        let mut new_stage = stage.clone();
                        new_stage.set_line(line_pos, 0);
                        let result = solve(&mut new_stage, max_depth, depth + 1);
                        if result.no_answer {
                            stage.set_line(line_pos, 1);
                            continue 'top;
                        } else {
                            let return_result =
                                check_result(&new_stage, result, &mut v_checked, &mut h_checked);
                            if !return_result.is_none() {
                                return return_result.unwrap();
                            }
                        }
                    }
                }

                return SolveResult {
                    has_answer,
                    multi_answer: false,
                    no_answer: false,
                    over_depth,
                };
            }
        }
    }

    pub fn solve_roughly(
        stage: &mut SolvingStage,
        depth: i32,
        score_min: u32,
        solve_roughly_try_count_list: &Vec<usize>,
        solve_roughly_check_count: &usize,
    ) -> SolveResult {
        'top: loop {
            stage.check();
            let complete = stage.check_loop();

            if !stage.result {
                return SolveResult {
                    has_answer: false,
                    multi_answer: false,
                    no_answer: true,
                    over_depth: false,
                };
            } else if complete {
                return SolveResult {
                    has_answer: true,
                    multi_answer: false,
                    no_answer: false,
                    over_depth: false,
                };
            } else {
                if depth as usize == (&solve_roughly_try_count_list).len() {
                    return SolveResult {
                        has_answer: false,
                        multi_answer: false,
                        no_answer: false,
                        over_depth: true,
                    };
                }

                let mut has_answer = false;

                let mut v_checked: Vec<Vec<bool>> =
                    vec![vec![false; (stage.size_c + 1) as usize]; (stage.size_r) as usize];

                let mut h_checked: Vec<Vec<bool>> =
                    vec![vec![false; (stage.size_c) as usize]; (stage.size_r + 1) as usize];

                let mut check_result = |new_stage: &SolvingStage,
                                        result: SolveResult,
                                        v_checked: &mut Vec<Vec<bool>>,
                                        h_checked: &mut Vec<Vec<bool>>|
                 -> Option<SolveResult> {
                    for (r, c, is_v) in new_stage.get_lines_pos().into_iter() {
                        if is_v {
                            v_checked[r as usize][c as usize] = v_line!(new_stage, r, c) != -1;
                        } else {
                            h_checked[r as usize][c as usize] = h_line!(new_stage, r, c) != -1;
                        }
                    }

                    if result.multi_answer {
                        return Some(SolveResult {
                            has_answer: true,
                            multi_answer: true,
                            no_answer: false,
                            over_depth: true,
                        });
                    } else if result.has_answer {
                        if has_answer {
                            return Some(SolveResult {
                                has_answer: true,
                                multi_answer: true,
                                no_answer: false,
                                over_depth: true,
                            });
                        } else {
                            has_answer = true;
                            return None;
                        }
                    }
                    return None;
                };

                for _ in 0..solve_roughly_try_count_list[depth as usize] {
                    let mut try_opt_stage1: Option<SolvingStage> = None;
                    let mut try_opt_stage2: Option<SolvingStage> = None;
                    let mut max_score: u32 = 0;
                    let mut try_c: isize = 0;
                    let mut try_r: isize = 0;
                    let mut try_is_virtical: bool = true;
                    for _ in 0..*solve_roughly_check_count {
                        let mut c: isize;
                        let mut r: isize;
                        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                        let is_virtical: bool = rand::Rng::gen_bool(&mut rng, 0.5);

                        'get_rc: loop {
                            if is_virtical {
                                c = rand::Rng::gen_range(&mut rng, 0..(stage.size_c as usize + 1))
                                    as isize;
                                r = rand::Rng::gen_range(&mut rng, 0..(stage.size_r as usize))
                                    as isize;

                                if v_line!(stage, r, c) == -1 {
                                    break 'get_rc;
                                }
                            } else {
                                c = rand::Rng::gen_range(&mut rng, 0..(stage.size_c as usize))
                                    as isize;
                                r = rand::Rng::gen_range(&mut rng, 0..(stage.size_r as usize + 1))
                                    as isize;

                                if h_line!(stage, r, c) == -1 {
                                    break 'get_rc;
                                }
                            }
                        }

                        let line_pos = if is_virtical {
                            idx_v_line(r, c)
                        } else {
                            idx_h_line(r, c)
                        };

                        let mut check_stage1: SolvingStage = stage.clone();
                        check_stage1.set_line(line_pos, 1);

                        let result1 = solve_roughly(
                            &mut check_stage1,
                            depth + 1,
                            score_min,
                            solve_roughly_try_count_list,
                            solve_roughly_check_count,
                        );
                        if result1.no_answer {
                            stage.set_line(line_pos, 0);
                            continue 'top;
                        } else {
                            let return_result = check_result(
                                &check_stage1,
                                result1,
                                &mut v_checked,
                                &mut h_checked,
                            );
                            if !return_result.is_none() {
                                return return_result.unwrap();
                            }
                        }

                        let mut check_stage2: SolvingStage = stage.clone();
                        check_stage2.set_line(line_pos, 0);

                        let result2 = solve_roughly(
                            &mut check_stage2,
                            depth + 1,
                            score_min,
                            solve_roughly_try_count_list,
                            solve_roughly_check_count,
                        );
                        if result2.no_answer {
                            stage.set_line(line_pos, 1);
                            continue 'top;
                        } else {
                            let return_result = check_result(
                                &check_stage2,
                                result2,
                                &mut v_checked,
                                &mut h_checked,
                            );
                            if !return_result.is_none() {
                                return return_result.unwrap();
                            }
                        }

                        // check common line
                        let mut changed = false;
                        for (r, c, is_v) in stage.get_lines_pos().into_iter() {
                            if is_v {
                                if v_line!(stage, r, c) == -1
                                    && v_line!(check_stage1, r, c) == v_line!(check_stage2, r, c)
                                    && v_line!(check_stage1, r, c) != -1
                                {
                                    stage.set_line(idx_v_line(r, c), v_line!(check_stage1, r, c));
                                    changed = true;
                                }
                            } else {
                                if h_line!(stage, r, c) == -1
                                    && h_line!(check_stage1, r, c) == h_line!(check_stage2, r, c)
                                    && h_line!(check_stage1, r, c) != -1
                                {
                                    stage.set_line(idx_h_line(r, c), h_line!(check_stage1, r, c));
                                    changed = true;
                                }
                            }
                        }

                        if changed {
                            continue 'top;
                        }

                        if max_score < check_stage1.score + check_stage2.score - stage.score * 2 {
                            max_score = check_stage1.score + check_stage2.score - stage.score * 2;
                            try_opt_stage1 = Some(check_stage1);
                            try_opt_stage2 = Some(check_stage2);
                            try_c = c;
                            try_r = r;
                            try_is_virtical = is_virtical;
                        }
                    }

                    let line_pos = if try_is_virtical {
                        idx_v_line(try_r, try_c)
                    } else {
                        idx_h_line(try_r, try_c)
                    };

                    let mut try_stage1 = try_opt_stage1.unwrap();
                    let mut try_stage2 = try_opt_stage2.unwrap();

                    let result1 = solve_roughly(
                        &mut try_stage1,
                        depth + 1,
                        score_min,
                        solve_roughly_try_count_list,
                        solve_roughly_check_count,
                    );
                    if result1.no_answer {
                        stage.set_line(line_pos, 0);
                        stage.check();
                        continue 'top;
                    } else {
                        let return_result =
                            check_result(&try_stage1, result1, &mut v_checked, &mut h_checked);
                        if !return_result.is_none() {
                            return return_result.unwrap();
                        }
                    }

                    let result2 = solve_roughly(
                        &mut try_stage2,
                        depth + 1,
                        score_min,
                        solve_roughly_try_count_list,
                        solve_roughly_check_count,
                    );
                    if result2.no_answer {
                        stage.set_line(line_pos, 1);
                        stage.check();
                        continue 'top;
                    } else {
                        let return_result =
                            check_result(&try_stage1, result2, &mut v_checked, &mut h_checked);
                        if !return_result.is_none() {
                            return return_result.unwrap();
                        }
                    }
                }

                return SolveResult {
                    has_answer,
                    multi_answer: false,
                    no_answer: false,
                    over_depth: true,
                };
            }
        }
    }
}
