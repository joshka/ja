# Changelog

All notable changes to this project will be documented in this file.

## [0.1.6](https://github.com/joshka/ja/compare/ja-v0.1.5...ja-v0.1.6) - 2024-01-15

### Other
- remove atty and cleanup clippy lints
- update deps

## [0.1.5](https://github.com/joshka/ja/compare/ja-v0.1.4...ja-v0.1.5) - 2023-11-07

### Fixed
- correctly handle when user selects the first system prompt / model

## [0.1.4](https://github.com/joshka/ja/compare/ja-v0.1.3...ja-v0.1.4) - 2023-11-07

### Other
- add 0.1.3 to CHANGELOG
- configure release-plz

### Ci

- ([e6977565](https://github.com/joshka/ja/commit/e6977565f361825ee6170337f9532188182cd92e))
  Configure release-plz

## [ja-v0.1.3] - 2023-11-07

### Bug Fixes

- ([d361d650](https://github.com/joshka/ja/commit/d361d6501e306bf7e4800dec52b62708e181c164))
  Remove duplicate prompt
- ([fa0dcfef](https://github.com/joshka/ja/commit/fa0dcfeff55edaff9e963bcba388c5a894392cb5))
  Typo

### Documentation

- ([bf79382d](https://github.com/joshka/ja/commit/bf79382de237b0ccf477f2987f5156542681da43))
  Add todos to README
- ([4cbf9118](https://github.com/joshka/ja/commit/4cbf91186a70840cf29a29a08ef3dd29ba7f751e))
  Add demo tape for vhs
- ([3a573468](https://github.com/joshka/ja/commit/3a573468d942f583b01b950d8fdae2863c369105))
  Update readme

### Features

- ([3fafc690](https://github.com/joshka/ja/commit/3fafc690f2e9c3d6cfbb7f1999f69ff2e56edb15))
  Interactive mode, multiple commands, tracing  
  Refactor and large changes towards multiple commands, tracing,
  interactive chat, more cli args, etc.
- ([977981e7](https://github.com/joshka/ja/commit/977981e743adc378fb6db8d34c46f1ff20d5f684))
  Improved system prompt selection  
  - Added `CODE_PROMPT` and `EXPERTS_PROMPT` constants for promptselection
  - Replaced hardcoded prompts in `interactive_mode` with constants
  - Added `include_str!` macro for importing prompt text from external
    files
  - Added `get_system_prompt()` function for prompt selection
  - Added `get_user_input()` function for reading user input in
    interactive mode
  - Refactored changes to function calls in `interactive_mode`
  - Changed `to_chat()` method to `to_chat_builder()`
  - Refactored the creation of `ChatCompletionRequestMessage` instances
    using `push()` in various places
  - Minor formatting adjustments
- ([6efee764](https://github.com/joshka/ja/commit/6efee7642a8a3177f4d99a56d11961a6f3053cc8))
  Add prompt engineer prompt
- ([83834436](https://github.com/joshka/ja/commit/83834436ac9a451a99c89a634bd874a736b61d42))
  Add code review prompt and update prompt-engineer.md  
  - Add a new code-review.md file containing guidelines for code review.
  - Update prompt-engineer.md to include changes regarding the Large
    Language Model and revised section descriptions.
  - Modify main.rs to include the new code-review.md file and adjust the
    selection menu to accommodate the new prompt.
- ([6083b838](https://github.com/joshka/ja/commit/6083b838ed36adb8fe41246e097d791975e50ee2))
  Update models
- ([c2109253](https://github.com/joshka/ja/commit/c2109253f032ac5a701129c8713deeb476762cde))
  Add model selection to interactive mode

### Miscellaneous Tasks

- ([471d415e](https://github.com/joshka/ja/commit/471d415ea0c1e1fb5ed5da7a1f6494f40a4b3f53))
  Bump webpki from 0.22.0 to 0.22.2  
  Bumps [webpki](https://github.com/briansmith/webpki) from 0.22.0 to 0.22.2.
  - [Commits](https://github.com/briansmith/webpki/commits)

  ---
  updated-dependencies:
  - dependency-name: webpki
    dependency-type: indirect
  ...
- ([9518c348](https://github.com/joshka/ja/commit/9518c348afd37724bd0063cefdae467bdcc1a56d))
  Bump all deps

### Refactor

- ([fb98cdd9](https://github.com/joshka/ja/commit/fb98cdd971d03cfb34a43a79e792afc63b75354b))
  Move CLI functionality to cli module  
  - Move the CLI related code from `lib.rs` to a new file `src/cli.rs`.
  - Update import paths in `src/main.rs` to reflect these changes.
  - Rename `chat` function argument from `ChatCompletionArgs`
    to`ChatCommandArgs` for clarity.
  - Update `interactive_mode` and `cli_mode` functions to use the new
    `ChatCommandArgs`.
  - Change the log message in the `_ =>` branch to use `bail!` instead of
    a `warn!`.

### Build

- ([d9c4d139](https://github.com/joshka/ja/commit/d9c4d139135cb1f07caaf25ac1707d8d26bf31f8))
  Add xtask stub

### Ci

- ([62255a6e](https://github.com/joshka/ja/commit/62255a6e6cbfb9c614466245b22abbbea8040174))
  Add release-plz workflow
- ([bcab2756](https://github.com/joshka/ja/commit/bcab2756cdb7cb1b7b237fb1841475e6e55d468d))
  Add ci workflow

## [0.1.2] - 2023-05-09

### Miscellaneous Tasks

- ([8949e603](https://github.com/joshka/ja/commit/8949e603e005ba3ada2f64e78ee0429212399bb2))
  Pass version to git cliff from cargo release  
  Because the replacements run before the hook, this was leaving the
  unreleased tag in the changelog instead of updating it as part of the
  release.
- ([e4e6af85](https://github.com/joshka/ja/commit/e4e6af85972f712f0f20b1fa349a2516d188cf1e))
  Show commit link and body  
  - update cliff.toml to add the a link to the commit on GitHub and the
  body of the commit.
- ([73d10b0e](https://github.com/joshka/ja/commit/73d10b0e5ac3889864be8c43962d3a4ec462d42c))
  Release ja version 0.1.2

## [0.1.1] - 2023-05-09

### Features

- ([931b26c4](https://github.com/joshka/ja/commit/931b26c4409088202de83bf95f781b7d216fed3a))
  Add input handling and error handling  
  - Added a new function `get_input` to handle input from command line
    arguments or standard input.
  - Added error handling for empty input and reading from standard input.
  - Added error handling for failure to create cache directory.

### Miscellaneous Tasks

- ([267c1d63](https://github.com/joshka/ja/commit/267c1d6376906b270ad4e7c976b4fa39b55f296e))
  Add changelog and git-cliff config
- ([be98dea7](https://github.com/joshka/ja/commit/be98dea71f800d02986d6ddfa0c5cfd61f8b1673))
  Configure cargo release
- ([5d5bb4a9](https://github.com/joshka/ja/commit/5d5bb4a94557946e1f74f758ccd5dcfcb161eeaf))
  Release ja version 0.1.1

## [0.1.0] - 2023-05-08

### Features

- ([0c068225](https://github.com/joshka/ja/commit/0c068225029b651b3e82653b0d48118155e49287))
  Init
- ([e8b4e940](https://github.com/joshka/ja/commit/e8b4e9408594141d41d567885f90c9b8eab96bd5))
  Initial implementation

<!-- generated by git-cliff -->
