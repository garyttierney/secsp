workflow "secsp/ci" {
  on = "push"
  resolves = [
    "secsp/ci/test",
  ]
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
  uses = "docker://bash:latest"
  args = [ "-c",  "[ $GITHUB_REF = 'refs/heads/master' ] || [ $GITHUB_REF = 'refs/heads/trying' ] || [ $GITHUB_REF = 'refs/heads/staging' ] || exit 78"]
}

action "secsp/ci-heavy/test" {
  uses = "docker://rust:latest"
  needs = [
    "secsp/ci-heavy/is-mainline",
  ]
  runs = "/bin/sh"
  args = ["-c", "cargo install afl --force"]
}
