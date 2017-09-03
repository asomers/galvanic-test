#[macro_use] extern crate galvanic_test;

#[cfg(feature = "galvanic_mock_integration")] extern crate galvanic_mock;

mod basic {
    use galvanic_test::TestFixture;

    fixture!( parameterised_fixture(x: i32, y: i32) -> i32 {
        params {
            vec![(1,2), (2,3), (3,4)].into_iter()
        }
        setup(&self) {
            self.x * self.y
        }
    });

    test!( inject_parameterised_fixtures | parameterised_fixture | {
        let (value, params) = parameterised_fixture.decompose();
        assert_eq!(value, params.x * params.y);
    });

    test!( override_parameterised_fixture | parameterised_fixture(6,7) | {
        assert_eq!(parameterised_fixture.val, 42);
    });
}

mod test_setup_teardown_for_each_parameterisation_of_a_single_fixture {
    use galvanic_test::TestFixture;

    static mut SETUP_COUNT: usize = 0;
    static mut TEAR_DOWN_COUNT: usize = 0;

    fixture!( counting_fixture(it: usize) -> () {
        params {
            vec![1, 2, 3].into_iter()
        }
        setup(&self) {
            unsafe { SETUP_COUNT += 1; }
        }
        tear_down(&self) {
            unsafe { TEAR_DOWN_COUNT += 1; }
        }
    });

    test!( inject_parameterised_fixtures | counting_fixture | {
        let params = counting_fixture.into_params();
        unsafe {
            assert_eq!(SETUP_COUNT, params.it);
            assert_eq!(TEAR_DOWN_COUNT, params.it-1);
        }
    });
}

mod test_setup_teardown_for_each_parameterisation_of_multiple_fixtures {
    use galvanic_test::TestFixture;

    static mut SETUP_COUNT_1: usize = 0;
    static mut TEAR_DOWN_COUNT_1: usize = 0;

    fixture!( counting_fixture_1(it: usize) -> () {
        params {
            vec![1, 2, 3].into_iter()
        }
        setup(&self) {
            unsafe { SETUP_COUNT_1 += 1; }
        }
        tear_down(&self) {
            unsafe { TEAR_DOWN_COUNT_1 += 1; }
        }
    });

    static mut SETUP_COUNT_2: usize = 0;
    static mut TEAR_DOWN_COUNT_2: usize = 0;

    fixture!( counting_fixture_2(it: usize) -> () {
        params {
            vec![1, 2, 3].into_iter()
        }
        setup(&self) {
            unsafe { SETUP_COUNT_2 += 1; }
        }
        tear_down(&self) {
            unsafe { TEAR_DOWN_COUNT_2 += 1; }
        }
    });

    test!( inject_parameterised_fixtures | counting_fixture_1, counting_fixture_2 | {
        let params1 = &counting_fixture_1.params;
        let params2 = &counting_fixture_2.params;
        unsafe {
            assert_eq!(SETUP_COUNT_1, (params1.it - 1)*3 + params2.it);
            assert_eq!(TEAR_DOWN_COUNT_1, (params1.it - 1)*3 + params2.it - 1);

            assert_eq!(SETUP_COUNT_2, (params1.it - 1)*3 + params2.it);
            assert_eq!(TEAR_DOWN_COUNT_2, (params1.it - 1)*3 + params2.it - 1);
        }
    });
}