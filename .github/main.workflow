workflow "secsp/ci" {
  on = "push"
  resolves = [
    "secsp/ci/test",
    "secsp/ci/heavy-test"
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

action "secsp/ci/is-master" {
  uses = "actions/bin/filter@master"
  needs = ["secsp/ci/build"]
  args = "branch master"
}

action "secsp/ci/is-staging" {
  uses = "actions/bin/filter@b2bea07"
  needs = ["secsp/ci/build"]
  runs = "branch staging"
}

action "secsp/ci/is-trying" {
  uses = "actions/bin/filter@b2bea07"
  needs = ["secsp/ci/build"]
  args = "branch trying"
}

action "secsp/ci/heavy-test" {
  uses = "docker://rust:latest"
  needs = [
    "secsp/ci/is-staging",
    "secsp/ci/is-master",
    "secsp/ci/is-trying",
  ]
  runs = "cargo afl build"
}
