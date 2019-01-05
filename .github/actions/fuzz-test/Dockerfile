FROM rustlang/rust:nightly

LABEL "com.github.actions.name"="fuzz-test"
LABEL "com.github.actions.description"="Run AFL using cargo-fuzz"
LABEL "com.github.actions.icon"="mic"
LABEL "com.github.actions.color"="purple"

LABEL "repository"="http://github.com/garyttierney/secsp"
LABEL "homepage"="http://github.com/garyttierney/secsp"
LABEL "maintainer"="Gary Tierney <gary.tierney@fastmail.com>"

ENV RUSTFLAGS="-C codegen-units=1"
ENV AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1
ENV AFL_SKIP_CPUFREQ=1
ENV LDFLAGS="-fno-lto"
ENV CFLAGS="-fno-lto"
RUN cargo install --force afl

WORKDIR "/github/workspace"
ADD entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
