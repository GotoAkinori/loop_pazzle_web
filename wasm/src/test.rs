#[cfg(test)]
mod tests {

    mod rotate {
        use crate::stage::*;
        #[test]
        fn test1() {
            debug_assert!(rot_p(P_RD, 1) == P_DL);
            debug_assert!(rot_p(P_UD, 2) == P_UD);
            debug_assert!(rot_p(P_UL | P_NONE, 1) == P_UR | P_NONE);
        }
    }

    mod check_number {
        use crate::{h_line, number, stage::SolvingStage, v_line};

        fn test_sub_x(num: i8, v1: bool, v2: bool, h1: bool, h2: bool) {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            number!(stage, 1, 1) = num;
            v_line!(stage, 1, 1) = if v1 { 1 } else { -1 };
            v_line!(stage, 1, 2) = if v2 { 1 } else { -1 };
            h_line!(stage, 1, 1) = if h1 { 1 } else { -1 };
            h_line!(stage, 2, 1) = if h2 { 1 } else { -1 };
            stage.dump();
            let score = stage.check_number(1, 1);
            stage.dump();
            println!("Score: {score}");
            debug_assert!(v_line!(stage, 1, 1) == if v1 { 1 } else { 0 });
            debug_assert!(v_line!(stage, 1, 2) == if v2 { 1 } else { 0 });
            debug_assert!(h_line!(stage, 1, 1) == if h1 { 1 } else { 0 });
            debug_assert!(h_line!(stage, 2, 1) == if h2 { 1 } else { 0 });
        }

        #[test]
        fn test1() {
            test_sub_x(1, true, false, false, false);
        }

        #[test]
        fn test2() {
            test_sub_x(1, false, true, false, false);
        }

        #[test]
        fn test3() {
            test_sub_x(1, false, false, true, false);
        }

        #[test]
        fn test4() {
            test_sub_x(1, false, false, false, true);
        }

        #[test]
        fn test5() {
            test_sub_x(2, true, true, false, false);
        }

        #[test]
        fn test6() {
            test_sub_x(2, true, false, true, false);
        }

        #[test]
        fn test7() {
            test_sub_x(3, true, false, true, true);
        }

        #[test]
        fn test8() {
            test_sub_x(3, true, true, true, false);
        }
    }

    mod check_point {
        use crate::stage::*;
        use crate::{h_line, point, v_line};

        #[test]
        fn test1() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            v_line!(stage, 1, 1) = 1;
            h_line!(stage, 1, 1) = 1;
            stage.dump();
            let score = stage.check_point(1, 1);
            stage.dump();
            println!("Score: {score}");
            debug_assert!(v_line!(stage, 0, 1) == 0);
            debug_assert!(h_line!(stage, 1, 0) == 0);
        }

        #[test]
        fn test2() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            point!(stage, 1, 1) = P_UR | P_UL | P_RL | P_NONE;
            stage.dump();
            let score = stage.check_point(1, 1);
            stage.dump();
            println!("Score: {score}");
            debug_assert!(v_line!(stage, 1, 1) == 0);
        }

        #[test]
        fn test3() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            point!(stage, 1, 1) = P_LINE_R;
            stage.dump();
            let score = stage.check_point(1, 1);
            stage.dump();
            println!("Score: {score}");
            debug_assert!(h_line!(stage, 1, 1) == 1);
        }

        #[test]
        fn test4() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            v_line!(stage, 1, 2) = 1;
            h_line!(stage, 2, 1) = 1;
            stage.dump();
            let score = stage.check_point(2, 2);
            stage.dump();
            println!("Score: {score}");
            debug_assert!(v_line!(stage, 2, 2) == 0);
            debug_assert!(h_line!(stage, 2, 2) == 0);
        }
    }

    mod check_group {
        use crate::{h_line, stage::*, v_line};

        #[test]
        fn test1() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            v_line!(stage, 1, 2) = 0;
            h_line!(stage, 2, 1) = 1;
            v_line!(stage, 3, 2) = 0;
            h_line!(stage, 3, 1) = 1;
            v_line!(stage, 1, 3) = 1;
            h_line!(stage, 2, 3) = 0;
            v_line!(stage, 3, 3) = 0;
            stage.dump();
            stage.check_group();
            stage.dump();
            stage.dump_group();
            debug_assert!(h_line!(stage, 3, 3) == 1);
        }
    }

    mod complex {
        use crate::solve::*;
        use crate::stage::*;
        use crate::{h_line, number, point, v_line};

        #[test]
        fn test_3_3() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 1, 1) = 3;
            number!(stage, 2, 2) = 3;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();
            debug_assert!(v_line!(stage, 1, 1) == 1);
            debug_assert!(h_line!(stage, 1, 1) == 1);
            debug_assert!(v_line!(stage, 2, 3) == 1);
            debug_assert!(h_line!(stage, 3, 2) == 1);

            debug_assert!(v_line!(stage, 0, 1) == 0);
            debug_assert!(h_line!(stage, 1, 0) == 0);
            debug_assert!(v_line!(stage, 3, 3) == 0);
            debug_assert!(h_line!(stage, 3, 3) == 0);
        }

        #[test]
        fn test_1_1_3() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 1, 1) = 1;
            number!(stage, 2, 2) = 1;
            number!(stage, 1, 2) = 3;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            debug_assert!(stage.result);
            debug_assert!(v_line!(stage, 1, 1) == 0);
            debug_assert!(h_line!(stage, 1, 1) == 0);
            debug_assert!(v_line!(stage, 2, 3) == 0);
            debug_assert!(h_line!(stage, 3, 2) == 0);
        }

        #[test]
        fn test_3_2_3() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 3, 1) = 3;
            number!(stage, 2, 2) = 2;
            number!(stage, 1, 3) = 3;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            debug_assert!(stage.result);
        }

        #[test]
        fn test_corner() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 0, 4) = 3;
            number!(stage, 4, 0) = 1;
            number!(stage, 4, 4) = 2;
            number!(stage, 1, 0) = 0;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();
            debug_assert!(stage.result);
            debug_assert!(v_line!(stage, 0, 5) == 1);
            debug_assert!(h_line!(stage, 0, 4) == 1);
            debug_assert!(v_line!(stage, 3, 5) == 1);
            debug_assert!(h_line!(stage, 5, 3) == 1);
            debug_assert!(v_line!(stage, 4, 0) == 0);
            debug_assert!(h_line!(stage, 5, 0) == 0);
            debug_assert!(v_line!(stage, 0, 0) == 0);
            debug_assert!(v_line!(stage, 2, 0) == 0);
            debug_assert!(h_line!(stage, 2, 0) == 0);
        }

        #[test]
        fn test_corner_line() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            v_line!(stage, 0, 0) = 0;
            v_line!(stage, 0, 5) = 1;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();
            debug_assert!(stage.result);
            debug_assert!(h_line!(stage, 0, 0) == 0);
            debug_assert!(h_line!(stage, 0, 4) == 1);
        }

        #[test]
        fn test_0_1() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 1, 1) = 0;
            number!(stage, 2, 2) = 1;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();
            debug_assert!(v_line!(stage, 2, 2) == 0);
            debug_assert!(h_line!(stage, 2, 2) == 0);
        }

        #[test]
        fn test_1_or() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 1, 1) = 1;
            point!(stage, 1, 1) = P_DIAGONAL_RD;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            println!("{}", p_dump(P_DIAGONAL_RD));
            println!("{}", p_dump(point!(stage, 1, 1)));
            println!("{}", in_possibility(point!(stage, 1, 1), P_DIAGONAL_RD));
            debug_assert!(v_line!(stage, 1, 2) == 0);
            debug_assert!(h_line!(stage, 2, 1) == 0);
        }

        #[test]
        fn complex1() {
            let mut stage: SolvingStage = SolvingStage::new(5, 5);
            number!(stage, 1, 1) = 0;
            number!(stage, 0, 2) = 2;
            number!(stage, 1, 3) = 1;
            number!(stage, 2, 4) = 1;
            number!(stage, 2, 1) = 2;
            number!(stage, 3, 2) = 3;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            debug_assert!(v_line!(stage, 1, 0) == 1);
            debug_assert!(v_line!(stage, 1, 1) == 0);
            debug_assert!(v_line!(stage, 1, 2) == 0);
            debug_assert!(v_line!(stage, 1, 4) == 0);
            debug_assert!(v_line!(stage, 2, 0) == 0);
            debug_assert!(v_line!(stage, 2, 1) == 1);
            debug_assert!(v_line!(stage, 2, 4) == 0);
            debug_assert!(v_line!(stage, 3, 3) == 1);
            debug_assert!(v_line!(stage, 3, 5) == 1);
            debug_assert!(v_line!(stage, 4, 3) == 0);
            debug_assert!(h_line!(stage, 0, 1) == 1);
            debug_assert!(h_line!(stage, 0, 3) == 0);
            debug_assert!(h_line!(stage, 1, 1) == 0);
            debug_assert!(h_line!(stage, 2, 0) == 1);
            debug_assert!(h_line!(stage, 2, 1) == 0);
            debug_assert!(h_line!(stage, 2, 3) == 0);
            debug_assert!(h_line!(stage, 2, 4) == 0);
            debug_assert!(h_line!(stage, 4, 2) == 1);
            debug_assert!(h_line!(stage, 4, 3) == 0);
        }

        #[test]
        fn complex_complete() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            number!(stage, 0, 1) = 1;
            number!(stage, 0, 2) = 2;
            number!(stage, 1, 1) = 0;
            number!(stage, 1, 3) = 3;
            number!(stage, 2, 0) = 1;
            number!(stage, 2, 3) = 3;
            number!(stage, 3, 2) = 2;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            let complete = stage.check_loop();

            debug_assert!(stage.result);
            debug_assert!(complete);
        }

        #[test]
        fn complex_error_multi_loop() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            number!(stage, 0, 0) = 3;
            number!(stage, 0, 1) = 3;
            number!(stage, 0, 3) = 0;
            number!(stage, 3, 3) = 3;
            number!(stage, 2, 3) = 3;
            stage.dump();
            stage.first_check();
            stage.check();
            stage.dump();

            println!("{}", p_dump(point!(stage, 0, 2)));

            let complete = stage.check_loop();

            debug_assert!(!complete);
            debug_assert!(!stage.result);
        }

        #[test]
        fn complex_complete2() {
            let mut stage: SolvingStage = SolvingStage::new(4, 4);
            number!(stage, 1, 3) = 1;
            number!(stage, 2, 1) = 2;
            number!(stage, 2, 2) = 1;
            number!(stage, 2, 3) = 0;
            number!(stage, 3, 3) = 1;

            stage.dump();
            stage.first_check();
            let result = solve::solve(&mut stage, 2, 0);
            stage.dump();

            debug_assert!(result.has_answer);
            debug_assert!(!result.multi_answer);
            debug_assert!(!result.no_answer);
            debug_assert!(!result.over_depth);
        }
    }

    mod create {
        use crate::make_puzzle;
        use std::time::Instant;

        #[test]
        fn make_puzzle_5_5() {
            let mut stage = make_puzzle(5, 5);
            stage.dump();
        }

        #[test]
        fn check_time() {
            let count = 20;

            let time_start = Instant::now();
            for i in 0..count {
                let time_local_start = Instant::now();
                make_puzzle(7, 7);
                let time_local_end = Instant::now();
                println!(
                    "[{}] {:?}",
                    i + 1,
                    time_local_end.duration_since(time_local_start)
                );
            }
            let time_end = Instant::now();

            println!("Total: {:?}", time_end.duration_since(time_start));
        }
    }
}
