use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

#[allow(deprecated)]
#[test]
fn cli_processes_csv_to_json() {
    let tmp = tempdir().unwrap();
    let out = tmp.path().join("out.json");

    let mut cmd = Command::cargo_bin("agent-data-pipeline-rust").unwrap();
    cmd.arg("--input")
        .arg("tests/fixtures/input.csv")
        .arg("--output")
        .arg(out.to_string_lossy().to_string())
        .arg("--min-amount")
        .arg("10")
        .arg("--category")
        .arg("retail")
        .arg("--uppercase-name")
        .assert()
        .success()
        .stdout(contains("pipeline complete: 2 records"));

    let json = std::fs::read_to_string(out).unwrap();
    assert!(json.contains("ALICE"));
    assert!(json.contains("CAROL"));
}
