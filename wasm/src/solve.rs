pub mod solve {
    use crate::{h_line, stage::*, v_line};

    #[derive(Clone, Copy, Debug)]
    pub struct SolveResult {
        pub has_answer: bool,
        pub multi_answer: bool,
        pub no_answer: bool,
        pub over_depth: bool,
    }

    pub fn solve(stage: &mut SolvingStage, max_depth: i32, depth: i32) -> SolveResult {
        loop {
            stage.check();
            let complete = stage.check_loop();
            let mut break_loop = false;

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

                let mut check_result = |new_stage: SolvingStage,
                                        result: SolveResult,
                                        v_checked: &mut Vec<Vec<bool>>,
                                        h_checked: &mut Vec<Vec<bool>>|
                 -> Option<SolveResult> {
                    if result.over_depth {
                        over_depth = true;
                    }

                    for r in 0..(new_stage.size_r) {
                        for c in 0..(new_stage.size_c + 1) {
                            v_checked[r as usize][c as usize] = v_line!(new_stage, r, c) != -1;
                        }
                    }

                    for r in 0..(stage.size_r + 1) {
                        for c in 0..(stage.size_c) {
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

                // v_line
                for r in 0..(stage.size_r) {
                    for c in 0..(stage.size_c + 1) {
                        if v_line!(stage, r, c) != -1 || v_checked[r as usize][c as usize] {
                            continue;
                        }

                        // line
                        {
                            let mut new_stage = stage.clone();
                            v_line!(new_stage, r, c) = 1;
                            let result = solve(&mut new_stage, max_depth, depth + 1);
                            if result.no_answer {
                                v_line!(stage, r, c) = 0;
                                break_loop = true;
                                break;
                            } else {
                                let return_result =
                                    check_result(new_stage, result, &mut v_checked, &mut h_checked);
                                if !return_result.is_none() {
                                    return return_result.unwrap();
                                }
                            }
                        }

                        // no-line
                        {
                            let mut new_stage = stage.clone();
                            v_line!(new_stage, r, c) = 0;
                            let result = solve(&mut new_stage, max_depth, depth + 1);
                            if result.no_answer {
                                v_line!(stage, r, c) = 1;
                                break_loop = true;
                                break;
                            } else {
                                let return_result =
                                    check_result(new_stage, result, &mut v_checked, &mut h_checked);
                                if !return_result.is_none() {
                                    return return_result.unwrap();
                                }
                            }
                        }
                    }

                    if break_loop {
                        break;
                    }
                }

                if break_loop {
                    continue;
                }

                // h_line
                for r in 0..(stage.size_r + 1) {
                    for c in 0..(stage.size_c) {
                        if h_line!(stage, r, c) != -1 || h_checked[r as usize][c as usize] {
                            continue;
                        }

                        // line
                        {
                            let mut new_stage = stage.clone();
                            h_line!(new_stage, r, c) = 1;
                            let result = solve(&mut new_stage, max_depth, depth + 1);
                            if result.no_answer {
                                h_line!(stage, r, c) = 0;
                                break_loop = true;
                                break;
                            } else {
                                let return_result =
                                    check_result(new_stage, result, &mut v_checked, &mut h_checked);
                                if !return_result.is_none() {
                                    return return_result.unwrap();
                                }
                            }
                        }

                        // no-line
                        {
                            let mut new_stage = stage.clone();
                            h_line!(new_stage, r, c) = 0;
                            let result = solve(&mut new_stage, max_depth, depth + 1);
                            if result.no_answer {
                                h_line!(stage, r, c) = 1;
                                break_loop = true;
                                break;
                            } else {
                                let return_result =
                                    check_result(new_stage, result, &mut v_checked, &mut h_checked);
                                if !return_result.is_none() {
                                    return return_result.unwrap();
                                }
                            }
                        }
                    }

                    if break_loop {
                        break;
                    }
                }

                if break_loop {
                    continue;
                }

                return SolveResult {
                    has_answer: has_answer,
                    multi_answer: false,
                    no_answer: false,
                    over_depth: over_depth,
                };
            }
        }
    }
}
