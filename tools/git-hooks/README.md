# Git Hooks for iceoryx-rs

The provided hooks add the github issue number to the commit message
and check for trailing whitespaces and code style violations with `cargo fmt`.

## Installation

The hooks are active when you add the `git-hooks` directory as hooks folder to
your local project git config:

```bash
git config core.hooksPath tools/git-hooks/
```

With that you will also receive the updates of the git hooks in the future.
We recommend doing this in every new clone you did on iceoryx-rs.
