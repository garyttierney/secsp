workflow "secsp/ci" {
  on = "push"
  resolves = [
    "secsp/ci/test",
    "secsp/ci/test-coverage",
  ]
}

action "secsp/ci/test-coverage-generate" {
  uses = "docker://xd009642/tarpaulin:develop-nightly"
  runs = "sh"
  args = ["-c", "cargo tarpaulin --out Xml"]
}

action "secsp/ci/test-coverage" {
  uses = "docker://bash:latest"
  needs = ["secsp/ci/test-coverage-generate"]
  secrets = ["CODECOV_TOKEN"]
  args = ["<(curl -s https://codecov.io/bash)"]
}

action "secsp/ci/build" {
  uses = "docker://rust:latest"
  runs = "cargo build"
}

action "secsp/ci/test" {
  uses = "docker://rust:latest"
  needs = ["secsp/ci/build"]
  runs = "cargo test"
}

workflow "secsp/ci-heavy" {
  on = "push"
  resolves = [
    "secsp/ci-heavy/test"
  ]
}

action "secsp/ci-heavy/is-mainline" {
  uses = "actions/bin/filter@master"
  args = "branch master || branch trying || branch staging"
}

action "secsp/ci-heavy/test" {
  uses = "docker://rustlang/rust:nightly"
  needs = [
    "secsp/ci-heavy/is-mainline",
  ]
  runs = "/bin/sh"
  args = ["-c", "cd /github/workspace/secsp_fuzzer && ./tools/run-fuzzer.sh || exit 1"]
}
