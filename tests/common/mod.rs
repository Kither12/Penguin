#[macro_export]
macro_rules! test_code_ok {
    ($test:expr) => {{
        use penguin::*;
        let res = run_code($test);
        assert!(res.is_ok());
    }};
}
#[macro_export]
macro_rules! test_code_failed {
    ($test:expr) => {{
        use penguin::*;
        let res = run_code($test);
        assert!(res.is_err());
    }};
}
