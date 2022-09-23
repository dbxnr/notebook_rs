#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .register_bin(
            "notebook_rs",
            trycmd::cargo::compile_example("notebook_rs", []),
        )
        .case("tests/cmd/*.trycmd");
}
