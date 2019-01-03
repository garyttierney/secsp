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

action "secsp/ci/is-mainline" {
  uses = "tngan/bin/filter@master"
  needs = ["secsp/ci/test"]
  args = ["branch", "master|staging|trying"]
}

action "secsp/ci/heavy-test" {
  uses = "docker://rust:latest"
  needs = [
    "secsp/ci/is-mainline",
  ]
  runs = "cargo afl build"
}
