workflow "secsp/ci" {
  on = "push"
  resolves = ["secsp/ci/test", "secsp/ci/heavy-test"]
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

action "secsp/ci/is-bors-branch" {
  uses = "actions/bin/filter@master"
  needs = ["secsp/ci/build"]
  args = ["branch", "master|trying|staging"]
}

action "secsp/ci/heavy-test" {
  uses = "docker://rust:latest"
  needs = ["secsp/ci/is-bors-branch"]
  runs = "cargo afl build"
}
