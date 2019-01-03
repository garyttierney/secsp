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

action "secsp/ci-heavy/is-master" {
  uses = "actions/bin/filter@master"
  args = ["branch", "master"]
}

action "secsp/ci-heavy/is-staging" {
  uses = "actions/bin/filter@master"
  args = ["branch", "is-staging"]
}

action "secsp/ci-heavy/is-trying" {
  uses = "actions/bin/filter@master"
  args = ["branch", "trying"]
}

action "secsp/ci-heavy/test" {
  uses = "docker://rust:latest"
  needs = [
    "secsp/ci-heavy/is-master",
    "secsp/ci-heavy/is-staging",
    "secsp/ci-heavy/is-trying",
  ]
  runs = "cargo afl build"
}
