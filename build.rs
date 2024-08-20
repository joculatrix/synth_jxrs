fn main() {
    slint_build::compile_with_config(
        "src/app.slint",
        slint_build::CompilerConfiguration::new()
            .with_style("cupertino-dark".into())
    ).unwrap();
}