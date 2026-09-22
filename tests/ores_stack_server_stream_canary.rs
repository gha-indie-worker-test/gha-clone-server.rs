use std::{env, fs, path::Path, path::PathBuf, process::Command};

const ORES_STACK_SHA: &str = "fcbf88b17ab0fd777781c4901651d5b1d58d3b2f";
const ORES_STACK_REPOSITORY: &str = "https://github.com/ORESoftware/ores-stack.git";

struct TempCheckout(PathBuf);

impl Drop for TempCheckout {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(root: &Path, program: &str, args: &[&str]) {
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .env("CARGO_TERM_COLOR", "always")
        .status()
        .unwrap_or_else(|error| panic!("could not run {program} {args:?}: {error}"));
    assert!(status.success(), "{program} {args:?} exited with {status}");
}

fn checkout_exact_head() -> TempCheckout {
    let mut root = env::temp_dir();
    root.push(format!(
        "ores-stack-server-stream-canary-{}-{ORES_STACK_SHA}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove stale ores-stack canary checkout");
    }
    fs::create_dir_all(&root).expect("create ores-stack canary checkout");

    run(&root, "git", &["init", "--quiet"]);
    run(
        &root,
        "git",
        &["remote", "add", "origin", ORES_STACK_REPOSITORY],
    );
    run(
        &root,
        "git",
        &["fetch", "--quiet", "--depth=1", "origin", ORES_STACK_SHA],
    );
    run(
        &root,
        "git",
        &["checkout", "--quiet", "--detach", "FETCH_HEAD"],
    );

    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&root)
        .output()
        .expect("read exact ores-stack canary head");
    assert!(output.status.success(), "git rev-parse failed");
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("git head must be utf-8")
            .trim(),
        ORES_STACK_SHA,
        "canary must compile the exact audited ores-stack head"
    );

    TempCheckout(root)
}

#[test]
fn exact_ores_stack_server_stream_head_compiles_and_streams_incrementally() {
    let checkout = checkout_exact_head();

    run(
        &checkout.0,
        "timeout",
        &[
            "15m",
            "cargo",
            "test",
            "--locked",
            "-p",
            "ores-stack-core",
            "--test",
            "rpc_generated_tree_compiles",
            "--test",
            "rpc_stream_admission_contract",
        ],
    );

    run(
        &checkout.0,
        "timeout",
        &["15m", "bash", "scripts/ppr-server-stream-e2e.sh"],
    );
}
