FROM rust:latest

LABEL "com.github.actions.name"="cargo"
LABEL "com.github.actions.description"="Run Cargo commands"
LABEL "com.github.actions.icon"="mic"
LABEL "com.github.actions.color"="purple"

LABEL "repository"="http://github.com/garyttierney/secsp"
LABEL "homepage"="http://github.com/garyttierney/secsp"
LABEL "maintainer"="Gary Tierney <gary.tierney@fastmail.com>"

RUN cargo install critcmp --force
RUN apt-get update && apt-get install jq -y

WORKDIR "/github/workspace"
ADD entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
