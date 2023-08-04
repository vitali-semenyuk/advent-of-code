#[macro_export]
macro_rules! check_answers {
    ($fa:literal, $sa:literal) => {
        #[test]
        fn test_first_part_solution() {
            let input = read_task_input!();
            let answer = $fa;

            assert_eq!(answer, solve_first_part(&input))
        }

        #[test]
        fn test_second_part_solution() {
            let input = read_task_input!();
            let answer = $sa;

            assert_eq!(answer, solve_second_part(&input))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_task_input {
    () => {{
        let (_, current_file) = file!().rsplit_once("/").unwrap();
        let input_file = current_file.replace(".rs", ".txt");

        std::fs::read_to_string(format!("./tasks/{}", input_file)).unwrap()
    }};
}
