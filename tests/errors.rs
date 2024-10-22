#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rs_isl::{run_isl, IslParams};

    #[test]
    fn wrong_thread_number_errors() {
        let op = |_num: &f32, _nb: Vec<Option<&f32>>| 0.0;
        let init = |_x: usize, _y: usize| 0.0;

        let params = IslParams::new(
            (100, 100),
            op,
            17,
            init,
            1,
            1,
            vec![],
            PathBuf::from("raw"),
        );

        let data = run_isl(params);

        assert!(data.is_err());
    }
}
