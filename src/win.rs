extern crate dirs_sys;

use std::env::var_os;
use std::path::PathBuf;
use std::ffi::OsString;
use std::iter::FromIterator;

use crate::BaseDirs;
use crate::UserDirs;
use crate::ProjectDirs;
use crate::{apply_attr,apply_attr_s};
apply_attr!{#[cfg(    feature="cli") ] // can't use const in non-cli since dirs are dynamically fetched
const d_cache: &str = ".cache"      ;
const d_cfg  : &str = ".config"     ;
const d_data : &str = ".local/share";
const d_bin  : &str = ".local/bin"  ;}





pub fn base_dirs() -> Option<BaseDirs> {
    let home_dir       = dirs_sys::known_folder_profile();
    let app_data_dir   = dirs_sys::known_folder_roaming_app_data(); //XDG_DATA_HOME doesn't override the absence of this dir
    let data_local_dir = dirs_sys::known_folder_local_app_data();
    if let (Some(home_dir), Some(app_data_dir), Some(data_local_dir)) = (home_dir, app_data_dir, data_local_dir) {
        apply_attr_s!{#[cfg(    feature="cli") ]
        let cache_dir        = var_os("XDG_CACHE_HOME" ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_cache));
        let config_dir       = var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_cfg  ));
        let data_dir         = var_os("XDG_DATA_HOME"  ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_data ));
        let executable_dir = Some(var_os("XDG_BIN_HOME").and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_bin  )));}
        apply_attr_s!{#[cfg(not(feature="cli"))]
        let cache_dir        = var_os("XDG_CACHE_HOME" ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| data_local_dir.clone());
        let config_dir       = var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| app_data_dir  .clone());
        let data_dir         = var_os("XDG_DATA_HOME"  ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| app_data_dir  .clone());
        let executable_dir: Option<PathBuf> = None;}
        let config_local_dir =                                                                                  data_local_dir.clone();
        let preference_dir   =                                                                                  config_dir    .clone();

        let base_dirs = BaseDirs {home_dir, cache_dir, config_dir,config_local_dir, data_dir,data_local_dir,
            executable_dir, preference_dir,
            runtime_dir:      None,
            state_dir:        None  };
        Some(base_dirs)
    } else { None }
}

pub fn user_dirs() -> Option<UserDirs> {
    if let Some(home_dir) = dirs_sys::known_folder_profile() {
        let audio_dir     = dirs_sys::known_folder_music();
        let desktop_dir   = dirs_sys::known_folder_desktop();
        let document_dir  = dirs_sys::known_folder_documents();
        let download_dir  = dirs_sys::known_folder_downloads();
        let picture_dir   = dirs_sys::known_folder_pictures();
        let public_dir    = dirs_sys::known_folder_public();
        let template_dir  = dirs_sys::known_folder_templates();
        let video_dir     = dirs_sys::known_folder_videos();

        let user_dirs = UserDirs {home_dir,
            audio_dir, desktop_dir, document_dir, download_dir,
            font_dir:     None,
            picture_dir, public_dir, template_dir, video_dir  };
        Some(user_dirs)
    } else { None }
}

pub fn project_dirs_from_path(project_path: PathBuf) -> Option<ProjectDirs> {
    let app_data_local   = dirs_sys::known_folder_local_app_data();
    let app_data_roaming = dirs_sys::known_folder_roaming_app_data();
    if let (Some(app_data_local), Some(app_data_roaming)) = (app_data_local, app_data_roaming) {
        let prj_data_local   = app_data_local  .join(&project_path);
        let prj_data_roaming = app_data_roaming.join(&project_path);
        apply_attr_s!{#[cfg(    feature="cli") ]
        let cache_dir        = var_os("XDG_CACHE_HOME" ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_cache)).join(&project_path);
        let config_dir       = var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_cfg  )).join(&project_path);
        let data_dir         = var_os("XDG_DATA_HOME"  ).and_then(dirs_sys::is_absolute_path).unwrap_or_else(|| home_dir.join(d_data )).join(&project_path);}
        apply_attr_s!{#[cfg(not(feature="cli"))]
        let cache_dir        = var_os("XDG_CACHE_HOME" ).and_then(dirs_sys::is_absolute_path).map_or_else(|| prj_data_local  .join("cache" ), |p| p.join(&project_path));
        let config_dir       = var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path).map_or_else(|| prj_data_roaming.join("config"), |p| p.join(&project_path));
        let data_dir         = var_os("XDG_DATA_HOME"  ).and_then(dirs_sys::is_absolute_path).map_or_else(|| prj_data_roaming.join("data"  ), |p| p.join(&project_path));}
        let config_local_dir =                                                                               prj_data_local  .join("config");
        let data_local_dir   =                                                                               prj_data_local  .join("data"  );
        let preference_dir   = config_dir.clone();

        let project_dirs = ProjectDirs {project_path, cache_dir, config_dir,config_local_dir, data_dir,data_local_dir, preference_dir,
            runtime_dir:      None,
            state_dir:        None  };
        Some(project_dirs)
    } else { None }
}

pub fn project_dirs_from(_qualifier: &str, organization: &str, application: &str) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::from_iter(&[organization, application]))
}
