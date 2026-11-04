[![crates.io](https://img.shields.io/crates/v/directories_xdg.svg?style=for-the-badge)](https://crates.io/crates/directories_xdg)
[![API documentation](https://img.shields.io/docsrs/directories_xdg/latest?style=for-the-badge)](https://docs.rs/directories_xdg/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg?style=for-the-badge)
![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-orange.svg?style=for-the-badge)

# `directories_xdg`

## Introduction

This fork fixes one major issue with the original project of [ignoring XDG env vars](https://github.com/dirs-dev/directories-rs/issues/47) on Windows and macOS platforms by respecting them for [BaseDirs](./#BaseDirs) and [ProjectDirs](./#ProjectDirs) (but not [UserDirs](./#UserDirs)). It also adds a `cli` feature that changes the default paths to a common cross-platform fallback, e.g., `~/.config`[^1] for `config_dir`

- a tiny mid-level library with a minimal API
- that provides the platform-specific, user-accessible locations
- for retrieving and storing configuration, cache and other data
- on Linux, Redox, Windows (≥ Vista), macOS and other platforms.

The library provides the location of these directories by leveraging the mechanisms defined by
- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and
  the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux, Windows, and macOS
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows
- the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
  guidelines on macOS

## Platforms

This library is written in Rust, and supports Linux, Redox, macOS and Windows.
Other platforms are also supported; they use the Linux conventions.

_dirs_, the low-level sister library, is available at [dirs-rs](https://codeberg.org/dirs/dirs-rs).

A version of this library running on the JVM is provided by [directories-jvm](https://codeberg.org/dirs/directories-jvm).

## Usage

#### Dependency

Add the library as a dependency to your project by running `cargo add directories_xdg` or inserting

```toml
directories_xdg = "6.0.11"
```

into the `[dependencies]` section of your Cargo.toml file.

If you are upgrading from version 2, please read the [section on breaking changes](#3) first.

#### Example

Library run by user Alice:

```rust
extern crate directories_xdg;
use directories_xdg::{BaseDirs, UserDirs, ProjectDirs};

if let Some(proj_dirs) = ProjectDirs::from("com", "Foo Corp",  "Bar App") {
    proj_dirs.config_dir();
    // Lin: /home/alice/.config/barapp
    // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
    // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
}

if let Some(base_dirs) = BaseDirs::new() {
    base_dirs.executable_dir();
    // Lin: Some(/home/alice/.local/bin)
    // Win: None
    // Mac: None
}

if let Some(user_dirs) = UserDirs::new() {
    user_dirs.audio_dir();
    // Lin: /home/alice/Music
    // Win: C:\Users\Alice\Music
    // Mac: /Users/Alice/Music
}
```

## Design Goals

- The _directories_xdg_ library is designed to provide an accurate snapshot of the system's state at
  the point of invocation of `BaseDirs::new`, `UserDirs::new` or `ProjectDirs::from`.<br/>
  Subsequent changes to the state of the system are not reflected in values created prior to such a change.
- This library does not create directories or check for their existence. The library only provides
  information on what the path to a certain directory _should_ be.<br/>
  How this information is used is a decision that developers need to make based on the requirements
  of each individual application.
- This library is intentionally focused on providing information on user-writable directories only,
  as there is no discernible benefit in returning a path that points to a user-level, writable
  directory on one operating system, but a system-level, read-only directory on another.<br/>
  The confusion and unexpected failure modes of such an approach would be immense.
  - `executable_dir` is specified to provide the path to a user-writable directory for binaries.<br/>
    As such a directory only commonly exists on Linux, it returns `None` on macOS and Windows.
  - `font_dir` is specified to provide the path to a user-writable directory for fonts.<br/>
    As such a directory only exists on Linux and macOS, it returns `None` on Windows.
  - `runtime_dir` is specified to provide the path to a directory for non-essential runtime data.
    It is required that this directory is created when the user logs in, is only accessible by the
    user itself, is deleted when the user logs out, and supports all filesystem features of the
    operating system.<br/>
    As such a directory only commonly exists on Linux, it returns `None` on macOS and Windows.

## Features

### `BaseDirs`

The intended use case for `BaseDirs` is to query the paths of user-invisible standard directories
that have been defined according to the conventions of the operating system the library is running on.

If you want to compute the location of cache, config or data directories for your own application or project, use `ProjectDirs` instead.<br>
(Note: for readability some Windows "Known Folder API" are represented in their env var format, so `$UserProfile` instead of `{FOLDERID_Profile}`)

| Function name      | Value on Linux                                           | Value on Windows                                             | Value on macOS                                            |
|------------------- |----------------------------------------------------------| ------------------------------------------------------------ | --------------------------------------------------------- |
| `home_dir`         | `$HOME`                                                  | `$UserProfile`                                               | `$HOME`                                                   |
| `cache_dir`        | `$XDG_CACHE_HOME`        or `$HOME`/.cache               | `$XDG_CACHE_HOME`     or `$LocalAppData`                     | `$XDG_CACHE_HOME`  or `$HOME`/Library/Caches              |
| `  `↑`cli`≠        |                                                          | `$XDG_CACHE_HOME`     or `$UserProfile`\\.cache              | `$XDG_CACHE_HOME`  or `$HOME`/.cache                      |
| `config_dir`       | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `$XDG_CONFIG_HOME`    or `$AppData`                          | `$XDG_CONFIG_HOME` or `$HOME`/Library/Application Support |
| `  `↑`cli`≠        |                                                          | `$XDG_CONFIG_HOME`    or `$UserProfile`\\.config             | `$XDG_CONFIG_HOME` or `$HOME`/.config                     |
| `config_local_dir` | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `$LocalAppData`                                              | `$XDG_CONFIG_HOME` or `$HOME`/Library/Application Support |
| `data_dir`         | `$XDG_DATA_HOME`         or `$HOME`/.local/share         | `$XDG_DATA_HOME`      or `$AppData`                          | `$XDG_DATA_HOME`   or `$HOME`/Library/Application Support |
| `  `↑`cli`≠        |                                                          | `$XDG_DATA_HOME`      or `$UserProfile`\\.local\\share       | `$XDG_DATA_HOME`   or `$HOME`/.local/share                |
| `data_local_dir`   | `$XDG_DATA_HOME`         or `$HOME`/.local/share         | `$LocalAppData`                                              | `$XDG_DATA_HOME`   or `$HOME`/Library/Application Support |
| `  `↑`cli`≠        |                                                          |                                                              | `$XDG_DATA_HOME`   or `$HOME`/.local/share                |
| `executable_dir`   | `Some($XDG_BIN_HOME)`    or `Some($HOME`/.local/bin`)`   | `None`                                                       | `None`                                                    |
| `  `↑`cli`≠        |                                                          | `Some($XDG_BIN_HOME)` or `Some($UserProfile`\\.local\\bin`)` | `Some($XDG_BIN_HOME)` or `Some($HOME`/.local/bin`)`       |
| `preference_dir`   | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `$XDG_CONFIG_HOME`    or `$AppData`                          | `$HOME`/Library/Preferences                               |
| `runtime_dir`      | `Some($XDG_RUNTIME_DIR)` or `None`                       | `None`                                                       | `None`                                                    |
| `state_dir`        | `Some($XDG_STATE_HOME)`  or `Some($HOME`/.local/state`)` | `None`                                                       | `None`                                                    |

| Fn→<br>↓OS	| `home_dir`    	| `cache_dir`              	| `config_dir`                       	| `config_local_dir`	| `data_dir`                         	| `data_local_dir`	| `executable_dir`                   	| `preference_dir`           	| `runtime_dir`            	| `state_dir`                  	|
|:----------	| --------------	| -------------------------	| -----------------------------------	| ------------------	| -----------------------------------	| ----------------	| -----------------------------------	| ---------------------------	| -------------------------	| -----------------------------	|
| 🐧Lin      	| `$HOME`       	| `$XDG_CACHE_HOME`        	| `$XDG_CONFIG_HOME`                 	| ←=                	| `$XDG_DATA_HOME`                   	| ←=              	| `Some($XDG_BIN_HOME)`              	| `$XDG_CONFIG_HOME`         	| `Some($XDG_RUNTIME_DIR)` 	| `Some($XDG_STATE_HOME)`      	|
| `  `⎇     	|               	| `$HOME`/.cache           	| `$HOME`/.config                    	| ←=                	| `$HOME`/.local/share               	| ←=              	| `Some($HOME`/.local/bin`)`         	| `$HOME`/.config            	| `None`                   	| `Some($HOME`/.local/state`)` 	|
| ⌘Mac      	| `$HOME`       	| `$XDG_CACHE_HOME`        	| `$XDG_CONFIG_HOME`                 	| ←=                	| `$XDG_DATA_HOME`                   	| ←=              	| `None`                             	| `$HOME`/Library/Preferences	| `None`                   	| `None`                       	|
| `  `⎇     	|               	| `$HOME`/Library/Caches   	| `$HOME`/Library/Application Support	| ←=                	| `$HOME`/Library/Application Support	| ←=              	|                                    	|                            	|                          	|                              	|
| `cli`≠    	|               	|                          	|                                    	|                   	|                                    	|                 	| `Some($XDG_BIN_HOME)`              	|                            	|                          	|                              	|
| `  `⎇     	|               	| `$HOME`/.cache           	| `$HOME`/.config                    	|                   	| `$HOME`/.local/share               	| ←=              	| `Some($HOME`/.local/bin`)`         	|                            	|                          	|                              	|
| ❖Win      	| `$UserProfile`	| `$XDG_CACHE_HOME`        	| `$XDG_CONFIG_HOME`                 	| `$LocalAppData`   	| `$XDG_DATA_HOME`                   	| `$LocalAppData` 	| `None`                             	| `$XDG_CONFIG_HOME`         	| `None`                   	| `None`                       	|
| `  `⎇     	|               	| `$LocalAppData`          	| `$AppData`                         	|                   	| `$AppData`                         	|                 	|                                    	| `$AppData`                 	|                          	|                              	|
| `cli`≠    	|               	|                          	|                                    	|                   	|                                    	|                 	| `Some($XDG_BIN_HOME)`              	|                            	|                          	|                              	|
| `  `⎇     	|               	| `$UserProfile`\\.cache   	| `$UserProfile`\\.config            	|                   	| `$UserProfile`\\.local\\share      	|                 	| `Some($UserProfile`\\.local\\bin`)`	|                            	|                          	|                              	|

`cli` feature enabled; `≠` show only if different from the main non-cli path; `⎇` alternative/fallback; Windows `$Env_Vars` are also natively known as `%Env_Vars%`

### `UserDirs`

The intended use case for `UserDirs` is to query the paths of user-facing standard directories
that have been defined according to the conventions of the operating system the library is running on.

| Function name    | Value on Linux                                                         | Value on Windows                 | Value on macOS                 |
| ---------------- | ---------------------------------------------------------------------- | -------------------------------- | ------------------------------ |
| `home_dir`       | `$HOME`                                                                | `$UserProfile`                   | `$HOME`                        |
| `audio_dir`      | `Some(XDG_MUSIC_DIR)`           or `None`                              | `Some({FOLDERID_Music})`         | `Some($HOME`/Music/`)`         |
| `desktop_dir`    | `Some(XDG_DESKTOP_DIR)`         or `None`                              | `Some({FOLDERID_Desktop})`       | `Some($HOME`/Desktop/`)`       |
| `document_dir`   | `Some(XDG_DOCUMENTS_DIR)`       or `None`                              | `Some({FOLDERID_Documents})`     | `Some($HOME`/Documents/`)`     |
| `download_dir`   | `Some(XDG_DOWNLOAD_DIR)`        or `None`                              | `Some({FOLDERID_Downloads})`     | `Some($HOME`/Downloads/`)`     |
| `font_dir`       | `Some($XDG_DATA_HOME`/fonts/`)` or `Some($HOME`/.local/share/fonts/`)` | `None`                           | `Some($HOME`/Library/Fonts/`)` |
| `picture_dir`    | `Some(XDG_PICTURES_DIR)`        or `None`                              | `Some({FOLDERID_Pictures})`      | `Some($HOME`/Pictures/`)`      |
| `public_dir`     | `Some(XDG_PUBLICSHARE_DIR)`     or `None`                              | `Some({FOLDERID_Public})`        | `Some($HOME`/Public/`)`        |
| `template_dir`   | `Some(XDG_TEMPLATES_DIR)`       or `None`                              | `Some({FOLDERID_Templates})`     | `None`                         | 
| `video_dir`      | `Some(XDG_VIDEOS_DIR)`          or `None`                              | `Some({FOLDERID_Videos})`        | `Some($HOME`/Movies/`)`        |

### `ProjectDirs`

The intended use case for `ProjectDirs` is to compute the location of cache, config or data directories for your own application or project,
which are derived from the standard directories.

| Function name      | Value on Linux                                                                     | Value on Windows                                                                           | Value on macOS                                                                                    |
|--------------------|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `cache_dir`        | `$XDG_CACHE_HOME`/`<project_path>`        or `$HOME`/.cache/`<project_path>`       | `$XDG_CACHE_HOME`/`<project_path>`      or `$LocalAppData`/`<project_path>`/cache          | `$XDG_CACHE_HOME`/`<project_path>`        or `$HOME`/Library/Caches/`<project_path>`              |
| `  `↑`cli`≠        |                                                                                    | `$XDG_CACHE_HOME`/`<project_path>`      or `$UserProfile`\\.cache\\`<project_path>`        | `$XDG_CACHE_HOME`/`<project_path>`        or `$HOME`/.cache/`<project_path>`                      |
| `config_dir`       | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/.config/`<project_path>`      | `$XDG_CONFIG_HOME`/`<project_path>`     or `$AppData`/`<project_path>`/config              | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/Library/Application Support/`<project_path>` |
| `  `↑`cli`≠        |                                                                                    | `$XDG_CONFIG_HOME`/`<project_path>`     or `$UserProfile`\\.config\\`<project_path>`       | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/.config/`<project_path>`                     |
| `config_local_dir` | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/.config/`<project_path>`      | `$LocalAppData`/`<project_path>`/config                                                    | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/Library/Application Support/`<project_path>` |
| `data_dir`         | `$XDG_DATA_HOME`/`<project_path>`         or `$HOME`/.local/share/`<project_path>` | `$XDG_DATA_HOME`/`<project_path>`       or `$AppData`/`<project_path>`/data                | `$XDG_DATA_HOME`/`<project_path>`         or `$HOME`/Library/Application Support/`<project_path>` |
| `  `↑`cli`≠        |                                                                                    | `$XDG_DATA_HOME`/`<project_path>`       or `$UserProfile`\\.local\\share\\`<project_path>` | `$XDG_DATA_HOME`/`<project_path>`         or `$HOME`/.local/share/`<project_path>`                |
| `data_local_dir`   | `$XDG_DATA_HOME`/`<project_path>`         or `$HOME`/.local/share/`<project_path>` | `$LocalAppData`/`<project_path>`/data                                                      | `$XDG_DATA_HOME`/`<project_path>`         or `$HOME`/Library/Application Support/`<project_path>` |
| `preference_dir`   | `$XDG_CONFIG_HOME`/`<project_path>`       or `$HOME`/.config/`<project_path>`      | `$XDG_CONFIG_HOME`/`<project_path>`     or `$AppData`/`<project_path>`/config              | `$HOME`/Library/Preferences/`<project_path>`                                                      |
| `runtime_dir`      | `Some($XDG_RUNTIME_DIR`/`<project_path>)`                                          | `None`                                                                                     | `None`                                                                                            |
| `state_dir`        | `Some($XDG_STATE_HOME`/`<project_path>)`  or `$HOME`/.local/state/`<project_path>` | `None`                                                                                     | `None`                                                                                            |

The specific value of `<project_path>` is computed by the

    ProjectDirs::from(qualifier: &str,
                      organization: &str,
                      application: &str)

function and varies across operating systems. As an example, calling

    ProjectDirs::from("org"         /*qualifier*/,
                      "Baz Corp"    /*organization*/,
                      "Foo Bar-App" /*application*/)

results in the following values:

| Value on Linux | Value on Windows         | Value on macOS               |
| -------------- | ------------------------ | ---------------------------- |
| `"foobar-app"` | `"Baz Corp/Foo Bar-App"` | `"org.Baz-Corp.Foo-Bar-App"` |

The `ProjectDirs::from_path` function allows the creation of `ProjectDirs` structs directly from a `PathBuf` value.
This argument is used verbatim and is not adapted to operating system standards.

The use of `ProjectDirs::from_path` is strongly discouraged, as its results will not follow operating system standards on at least two of three platforms.

## Comparison

There are other crates in the Rust ecosystem that try similar or related things.
Here is an overview of them, combined with ratings on properties that guided the design of this crate.

Please take this table with a grain of salt: a different crate might very well be more suitable for your specific use case.
(Of course _my_ crate achieves _my_ design goals better than other crates, which might have had different design goals.)

| Library                                                   | Status         | Lin | Mac | Win |Base|User|Proj|Conv|
| --------------------------------------------------------- | -------------- |:---:|:---:|:---:|:--:|:--:|:--:|:--:|
| **directories**                                           | **Developed**  |  ✔  |  ✔  |  ✔  | ✔  | ✔  | ✔  | ✔  |
| [dirs](https://crates.io/crates/dirs)                     | Developed      |  ✔  |  ✔  |  ✔  | ✔  | ✔  | ✖  | ✔  |
| [app_dirs2](https://crates.io/crates/app_dirs2)           | Maintained     |  ✔  |  ✔  |  ✔  | 🞈  | ✖  | ✔  | ✖  |
| [standard_paths](https://crates.io/crates/standard_paths) | Maintained     |  ✔  |  ✖  |  ✔  | ✔  | ✔  | ✔  | ✖  |
| [xdg](https://crates.io/crates/xdg)                       | Maintained     |  ✔  |  ✖  |  ✖  | ✔  | ✖  | ✔  | 🞈  |
| [app_dirs](https://crates.io/crates/app_dirs)             | Unmaintained   |  ✔  |  ✔  |  ✔  | 🞈  | ✖  | ✔  | ✖  |
| [s_app_dir](https://crates.io/crates/s_app_dir)           | Unmaintained?  |  ✔  |  ✖  |  🞈  | ✖  | ✖  | 🞈  | ✖  |
| [xdg-basedir](https://crates.io/crates/xdg-basedir)       | Unmaintained?  |  ✔  |  ✖  |  ✖  | ✔   | ✖  | ✖  | 🞈  |
| [xdg-rs](https://crates.io/crates/xdg-rs)                 | Obsolete       |  ✔  |  ✖  |  ✖  | ✔   | ✖  | ✖  | 🞈  |

- Lin: Linux support
- Mac: macOS support
- Win: Windows support
- Base: Supports [generic base directories](#basedirs)
- User: Supports [user directories](#userdirs)
- Proj: Supports [project-specific base directories](#projectdirs)
- Conv: Follows naming conventions of the operating system it runs on

## Build

It's possible to cross-compile this library if the necessary toolchains are installed with rustup.
This is helpful to ensure a change has not broken compilation on a different platform.

The following commands will build this library on Linux, macOS and Windows:

```
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=x86_64-pc-windows-gnu
cargo build --target=x86_64-apple-darwin
cargo build --target=x86_64-unknown-redox
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[^1]: `~` is `$HOME` on MacOS, Linux; `%USERPROFILE%` on Windows
