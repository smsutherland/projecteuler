macro_rules! mk_test {
    ($num:literal, $module:ident, $expected:expr) => {
        mod $module {
            #![allow(unused)]
            include!(concat!("../../euler-", stringify!($num), "/src/main.rs"));

            #[test]
            fn $module() {
                assert_eq!(run(), $expected);
            }
        }
    };
}

mk_test!(1, euler_1, 233168);
mk_test!(2, euler_2, 4613732);
mk_test!(3, euler_3, 6857);
mk_test!(4, euler_4, 906609);
mk_test!(5, euler_5, 232792560);
mk_test!(6, euler_6, 25164150);
mk_test!(7, euler_7, 104743);
mk_test!(8, euler_8, 23514624000);
mk_test!(9, euler_9, 31875000);
mk_test!(10, euler_10, 142913828922);
mk_test!(11, euler_11, 70600674);
mk_test!(12, euler_12, 76576500);
mk_test!(13, euler_13, "5537376230");
mk_test!(14, euler_14, 837799);
mk_test!(15, euler_15, 137846528820);
mk_test!(16, euler_16, 1366);
mk_test!(17, euler_17, 21124);
mk_test!(18, euler_18, 1074);
mk_test!(19, euler_19, 171);
mk_test!(20, euler_20, 648);
mk_test!(21, euler_21, 31626);
mk_test!(22, euler_22, 871198282);
mk_test!(23, euler_23, 4179871);
mk_test!(24, euler_24, 2783915460);
mk_test!(25, euler_25, 4782);
mk_test!(26, euler_26, 983);