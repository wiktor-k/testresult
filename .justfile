#!/usr/bin/env -S just --working-directory . --justfile
# Load project-specific properties from the `.env` file

set dotenv-load := true

# Since this is a first recipe it's being run by default.
# Faster checks need to be executed first for better UX.  For example
# codespell is very fast. cargo fmt does not need to download crates etc.

# Perform all checks
check: spelling formatting docs lints dependencies tests

# Checks common spelling mistakes
spelling:
    codespell

# Checks source code formatting
formatting:
    just --unstable --fmt --check
    # We're using nightly to properly group imports, see .rustfmt.toml
    cargo +nightly fmt --all -- --check

# Lints the source code
lints:
    cargo clippy --workspace --no-deps --all-targets -- -D warnings

# Checks for issues with dependencies
dependencies:
    cargo deny check

# Runs all unit tests. By default ignored tests are not run. Run with `ignored=true` to run only ignored tests
tests:
    cargo test --all

# Build docs for this crate only
docs:
    cargo doc --no-deps

# Installs packages required to build
[linux]
install-packages:
    sudo apt-get install --assume-yes --no-install-recommends $UBUNTU_PACKAGES

[macos]
[windows]
install-packages:
    echo no-op

# Checks for commit messages
check-commits REFS='main..':
    #!/usr/bin/env bash
    set -euo pipefail
    for commit in $(git rev-list "{{ REFS }}"); do
      MSG="$(git show -s --format=%B "$commit")"
      CODESPELL_RC="$(mktemp)"
      git show "$commit:.codespellrc" > "$CODESPELL_RC"
      if ! grep -q "Signed-off-by: " <<< "$MSG"; then
        printf "Commit %s lacks \"Signed-off-by\" line.\n" "$commit"
        printf "%s\n" \
            "  Please use:" \
            "    git rebase --signoff main && git push --force-with-lease" \
            "  See https://developercertificate.org/ for more details."
        exit 1;
      elif ! codespell --config "$CODESPELL_RC" - <<< "$MSG"; then
        printf "The spelling in commit %s needs improvement.\n" "$commit"
        exit 1;
      else
        printf "Commit %s is good.\n" "$commit"
      fi
    done

# Fixes common issues. Files need to be git add'ed
fix:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! git diff-files --quiet ; then
        echo "Working tree has changes. Please stage them: git add ."
        exit 1
    fi

    codespell --write-changes
    just --unstable --fmt
    # try to fix rustc issues
    cargo fix --allow-staged
    # try to fix clippy issues
    cargo clippy --fix --allow-staged

    # fmt must be last as clippy's changes may break formatting
    cargo +nightly fmt --all
