#[macro_export]
macro_rules! define_year_mod {
    () => {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
        pub mod day09;
        pub mod day10;
        pub mod day11;
        pub mod day12;
        pub mod day13;
        pub mod day14;
        pub mod day15;
        pub mod day16;
        pub mod day17;
        pub mod day18;
        pub mod day19;
        pub mod day20;
        pub mod day21;
        pub mod day22;
        pub mod day23;
        pub mod day24;
        pub mod day25;

        pub(super) fn retrieve_problem(day: u32, problem: u8) -> String {
            match day {
                1 => day01::retrieve_problem(problem),
                2 => day02::retrieve_problem(problem),
                3 => day03::retrieve_problem(problem),
                4 => day04::retrieve_problem(problem),
                5 => day05::retrieve_problem(problem),
                6 => day06::retrieve_problem(problem),
                7 => day07::retrieve_problem(problem),
                8 => day08::retrieve_problem(problem),
                9 => day09::retrieve_problem(problem),
                10 => day10::retrieve_problem(problem),
                11 => day11::retrieve_problem(problem),
                12 => day12::retrieve_problem(problem),
                13 => day13::retrieve_problem(problem),
                14 => day14::retrieve_problem(problem),
                15 => day15::retrieve_problem(problem),
                16 => day16::retrieve_problem(problem),
                17 => day17::retrieve_problem(problem),
                18 => day18::retrieve_problem(problem),
                19 => day19::retrieve_problem(problem),
                20 => day20::retrieve_problem(problem),
                21 => day21::retrieve_problem(problem),
                22 => day22::retrieve_problem(problem),
                23 => day23::retrieve_problem(problem),
                24 => day24::retrieve_problem(problem),
                25 => day25::retrieve_problem(problem),
                _ => panic!("Day not found"),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(super) fn retrieve_code(day: u32, code: u8) -> String {
            match day {
                1 => day01::retrieve_code(code),
                2 => day02::retrieve_code(code),
                3 => day03::retrieve_code(code),
                4 => day04::retrieve_code(code),
                5 => day05::retrieve_code(code),
                6 => day06::retrieve_code(code),
                7 => day07::retrieve_code(code),
                8 => day08::retrieve_code(code),
                9 => day09::retrieve_code(code),
                10 => day10::retrieve_code(code),
                11 => day11::retrieve_code(code),
                12 => day12::retrieve_code(code),
                13 => day13::retrieve_code(code),
                14 => day14::retrieve_code(code),
                15 => day15::retrieve_code(code),
                16 => day16::retrieve_code(code),
                17 => day17::retrieve_code(code),
                18 => day18::retrieve_code(code),
                19 => day19::retrieve_code(code),
                20 => day20::retrieve_code(code),
                21 => day21::retrieve_code(code),
                22 => day22::retrieve_code(code),
                23 => day23::retrieve_code(code),
                24 => day24::retrieve_code(code),
                25 => day25::retrieve_code(code),
                _ => panic!("Day not found"),
            }
        }

        #[cfg(target_arch = "wasm32")]
        pub(super) fn retrieve_html(day: u32, code: u8, dark: bool) -> String {
            match day {
                1 => day01::retrieve_html(code, dark),
                2 => day02::retrieve_html(code, dark),
                3 => day03::retrieve_html(code, dark),
                4 => day04::retrieve_html(code, dark),
                5 => day05::retrieve_html(code, dark),
                6 => day06::retrieve_html(code, dark),
                7 => day07::retrieve_html(code, dark),
                8 => day08::retrieve_html(code, dark),
                9 => day09::retrieve_html(code, dark),
                10 => day10::retrieve_html(code, dark),
                11 => day11::retrieve_html(code, dark),
                12 => day12::retrieve_html(code, dark),
                13 => day13::retrieve_html(code, dark),
                14 => day14::retrieve_html(code, dark),
                15 => day15::retrieve_html(code, dark),
                16 => day16::retrieve_html(code, dark),
                17 => day17::retrieve_html(code, dark),
                18 => day18::retrieve_html(code, dark),
                19 => day19::retrieve_html(code, dark),
                20 => day20::retrieve_html(code, dark),
                21 => day21::retrieve_html(code, dark),
                22 => day22::retrieve_html(code, dark),
                23 => day23::retrieve_html(code, dark),
                24 => day24::retrieve_html(code, dark),
                25 => day25::retrieve_html(code, dark),
                _ => panic!("Day not found"),
            }
        }

        pub(super) fn solve(input: &str, day: u32, problem: u8) -> String {
            match day {
                1 => day01::solve(input, problem),
                2 => day02::solve(input, problem),
                3 => day03::solve(input, problem),
                4 => day04::solve(input, problem),
                5 => day05::solve(input, problem),
                6 => day06::solve(input, problem),
                7 => day07::solve(input, problem),
                8 => day08::solve(input, problem),
                9 => day09::solve(input, problem),
                10 => day10::solve(input, problem),
                11 => day11::solve(input, problem),
                12 => day12::solve(input, problem),
                13 => day13::solve(input, problem),
                14 => day14::solve(input, problem),
                15 => day15::solve(input, problem),
                16 => day16::solve(input, problem),
                17 => day17::solve(input, problem),
                18 => day18::solve(input, problem),
                19 => day19::solve(input, problem),
                20 => day20::solve(input, problem),
                21 => day21::solve(input, problem),
                22 => day22::solve(input, problem),
                23 => day23::solve(input, problem),
                24 => day24::solve(input, problem),
                25 => day25::solve(input, problem),
                _ => panic!("Day not found"),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(super) fn input(day: u32) -> String {
            match day {
                1 => day01::input(),
                2 => day02::input(),
                3 => day03::input(),
                4 => day04::input(),
                5 => day05::input(),
                6 => day06::input(),
                7 => day07::input(),
                8 => day08::input(),
                9 => day09::input(),
                10 => day10::input(),
                11 => day11::input(),
                12 => day12::input(),
                13 => day13::input(),
                14 => day14::input(),
                15 => day15::input(),
                16 => day16::input(),
                17 => day17::input(),
                18 => day18::input(),
                19 => day19::input(),
                20 => day20::input(),
                21 => day21::input(),
                22 => day22::input(),
                23 => day23::input(),
                24 => day24::input(),
                25 => day25::input(),
                _ => panic!("Day not found"),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(super) fn solve_all() {
            (1..=25).for_each(|x| {
                println!("  Day {}", x);
                (1..=2).for_each(|y| {
                    println!("    Problem {}: {}", y, solve(&input(x), x, y));
                });
            });
        }
    };
}
