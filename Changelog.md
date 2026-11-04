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

- Δ __Changed__
  + crate name to `directories_xdg` from `directories`

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

### 6

- Update `dirs-sys` dependency to `0.5.0`, which in turn updates `windows-sys` dependency to `0.59.0`.

### 5

- Update `dirs-sys` dependency to `0.4.0`.
- Add `config_local_dir` for non-roaming configuration on Windows. On non-Windows platforms the behavior is identical to `config dir`.

### 4

- **BREAKING CHANGE** The behavior of `executable_dir` has been adjusted to not depend on `$XDG_DATA_HOME`.
  Code, which assumed that setting the `$XDG_DATA_HOME` environment variable also impacted `executable_dir` if
  the `$XDG_BIN_HOME` environment variable was not set, requires adjustment.
- Add support for `XDG_STATE_HOME`.

### 3

- **BREAKING CHANGE** The behavior of the `BaseDirs::config_dir` and `ProjectDirs::config_dir`
    on macOS has been adjusted (thanks to [everyone involved](https://codeberg.org/dirs/directories-rs/issues/62)):
  - The existing `config_dir` functions have been changed to return the `Application Support`
    directory on macOS, as suggested by Apple documentation.
  - The behavior of the `config_dir` functions on non-macOS platforms has not been changed.
  - If you have used the `config_dir` functions to store files, it may be necessary to write code
    that migrates the files to the new location on macOS.<br/>
    (Alternative: change uses of the `config_dir` functions to uses of the `preference_dir` functions
    to retain the old behavior.)
- The newly added `BaseDirs::preference_dir` and `ProjectDirs::preference_dir` functions returns
  the `Preferences` directory on macOS now, which – according to Apple documentation – shall only
  be used to store .plist files using Apple-proprietary APIs.
  – `preference_dir` and `config_dir` behave identical on non-macOS platforms.

### 2

**BREAKING CHANGE** The behavior of deactivated, missing or invalid [_XDG User Dirs_](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
entries on Linux has been improved (contributed by @tmiasko, thank you!):

- Version 1 returned the user's home directory (`Some($HOME)`) for such faulty entries, except for a faulty `XDG_DESKTOP_DIR` entry which returned (`Some($HOME/Desktop)`).
- Version 2 returns `None` for such entries.
