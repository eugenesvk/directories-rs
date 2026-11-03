# Changelog
All notable changes to this project will be documented in this file

[unreleased]: https://github.com/eugenesvk/directories-rs/compare/6.0.1...HEAD
## [Unreleased]
<!-- - ✨ __Added__ -->
  <!-- + new features -->
<!-- - Δ __Changed__ -->
  <!-- + changes in existing functionality -->
<!-- - 🐞 __Fixed__ -->
  <!-- + bug fixes -->
<!-- - 💩 __Deprecated__ -->
  <!-- + soon-to-be removed features -->
<!-- - 🗑️ __Removed__ -->
  <!-- + now removed features -->
<!-- - 🔒 __Security__ -->
  <!-- + vulnerabilities -->

[6.0.1]: https://github.com/eugenesvk/directories-rs/releases/tag/6.0.1
## [6.0.1]
- ✨ __Added__
  + `cli` crate feature to use more common base dirs (e.g., `~/.config` even on Windows)
- Δ __Changed__
  + `cache_dir` to respect `XDG_CACHE_HOME` on Win/Mac
  + `config_dir` to respect `XDG_CONFIG_HOME` on Win/Mac
    + `config_local_dir` follows XDG on Mac, but not Win
    + `preference_dir` follows XDG on Win, but not Mac
  + `data_dir` to respect `XDG_DATA_HOME` on Win/Mac
    + `data_local_dir` follows XDG on Mac, but not Win
  + `executable_dir` to respect `XDG_BIN_HOME` on `cli`-featured Win/Mac
