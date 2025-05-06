use rust2go::RegenArgs;

fn main() {
    rust2go::Builder::new()
        .with_go_src("./go")
        .with_regen_arg(RegenArgs {
            src: "./src/Excange.rs".into(),
            dst: "./go/gen_ex.go".into(),
            ..Default::default()
        })
        .with_regen_arg(RegenArgs {
            src: "./src/types.rs".into(),
            dst: "./go/gen_types.go".into(),
            ..Default::default()
        })
        .build();
}