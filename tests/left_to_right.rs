#[cfg(test)]
mod tests {
    use parking_lot::RwLockReadGuard;
    use rs_isl::{run_isl, IslParams};

    #[test]
    fn valid_result() {
        let size = (4, 1);
        let neighbours = vec![(-1, 0)];

        let op = |_num: RwLockReadGuard<f64>, nb: &Vec<Option<RwLockReadGuard<f64>>>| {
            if nb.first().unwrap().is_some() {
                let f = **nb[0].as_ref().unwrap();

                if f != 0.0 {
                    return 1.0;
                }
            }
            0.0
        };

        let init = |x: usize, _y: usize| {
            if x == 0 {
                return 1.0;
            }
            0.0
        };

        let params = IslParams::new(
            size,
            op,
            1,
            init,
            4,
            4,
            neighbours,
            rs_isl::OutputType::RawData,
        );

        let data = run_isl(params);

        let expected = vec![
            vec![vec![0.0, 1.0, 0.0, 0.0]],
            vec![vec![0.0, 0.0, 1.0, 0.0]],
            vec![vec![0.0, 0.0, 0.0, 1.0]],
            vec![vec![0.0, 0.0, 0.0, 0.0]],
        ];

        match data.unwrap() {
            rs_isl::IslOutput::RawData(vec) => assert_eq!(vec, expected),
            rs_isl::IslOutput::String(_vec) => {
                panic!("wrong data type was returned")
            }
        }
    }

    #[test]
    fn wrong_thread_number_errors() {
        let op = |_num: RwLockReadGuard<f64>, _nb: &Vec<Option<RwLockReadGuard<f64>>>| 0.0;
        let init = |_x: usize, _y: usize| 0.0;

        let params = IslParams::new(
            (100, 100),
            op,
            17,
            init,
            1,
            1,
            vec![],
            rs_isl::OutputType::RawData,
        );

        let data = run_isl(params);

        assert!(data.is_err());
    }
}
