use anyhow::{anyhow, Result};
use gix::{
    bstr::{BString, ByteSlice},
    objs::tree::EntryKind,
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

fn box_err<T: std::fmt::Debug>(e: T) -> anyhow::Error {
    anyhow!("error: {:?}", e)
}

pub fn run(
    theme: &Option<Vec<String>>,
    all: bool,
    repo_url: String,
    dir_path: String,
    branch: String,
) -> Result<()> {
    // Handle the easy flags.
    if all {
        return download_all_themes(&repo_url, &dir_path, &branch);
    }
    if theme.is_none() || theme.as_ref().unwrap().is_empty() {
        return Err(anyhow!("Must either pass a theme name or --all."));
    }

    // Try to find the given themes.
    // Search order for each theme is:
    // 1. Assume it is a path, which may be absolute or relative.
    //    a. If the path is a directory, copy all of the toml files over.
    //    b. If the path ends in toml, copy it over.
    // 2. Remote repository.
    let theme_dir = tca_types::user_themes_path()?;
    let themes: HashSet<&String> = HashSet::from_iter(theme.as_ref().unwrap());
    let mut found_themes: HashSet<&String> = HashSet::new();
    for path in &themes {
        let theme_path = PathBuf::from(path);
        if theme_path.is_dir() {
            // Copy all .toml files to user theme path.
            for entry in WalkDir::new(theme_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "yaml"))
            {
                let src = entry.path();
                let dest = theme_dir.join(entry.file_name());
                println!("Copying {:#?} to {:#?}", src, dest);
                std::fs::copy(src, dest)?;
                found_themes.insert(path);
            }
        } else if theme_path.extension().is_some_and(|ext| ext == "yaml") {
            // Copy file to user theme path if it is a toml.
            let dest = theme_dir.join(
                theme_path
                    .file_name()
                    .ok_or_else(|| anyhow!("Can't parse filename."))?,
            );
            println!("Copying {:#?} to {:#?}", theme_path, dest);
            if theme_path.exists() && std::fs::copy(theme_path, dest).is_ok() {
                found_themes.insert(path);
            }
        }
    }
    let remaining_themes: HashSet<&String> = themes.difference(&found_themes).copied().collect();
    if !remaining_themes.is_empty() {
        println!("Didn't find all given themes as local files. Checking remote repository...");
        copy_from_repo(remaining_themes, &repo_url, &dir_path, &branch)?;
    }

    Ok(())
}

pub fn download_all_themes(repo_url: &String, dir_path: &str, branch: &String) -> Result<()> {
    let theme_dir = tca_types::user_themes_path()?;
    let tmp_dir = tempfile::tempdir()?;
    let repo = fetch_repo(repo_url, branch, &tmp_dir)?;
    download_directory(&repo, dir_path, &theme_dir).map_err(box_err)?;
    Ok(())
}

fn copy_from_repo(
    theme: HashSet<&String>,
    repo_url: &String,
    dir_path: &String,
    branch: &String,
) -> Result<()> {
    let theme_names = theme.iter().map(|t| {
        let mut theme_name = heck::AsKebabCase(t).to_string();

        if !theme_name.ends_with(".yaml") {
            theme_name.push_str(".yaml");
        }
        theme_name
    });
    let mut theme_names: HashSet<String> = HashSet::from_iter(theme_names);
    let tmp_dir = tempfile::tempdir()?;
    let theme_dir = tca_types::user_themes_path()?;
    let repo = fetch_repo(repo_url, branch, &tmp_dir)?;
    let head = repo.head_commit()?;
    let tree = head.tree()?;

    // Navigate to the subdirectory (or use root if dir_path is empty)
    let subtree = if dir_path.is_empty() {
        tree
    } else {
        tree.lookup_entry_by_path(dir_path)?
            .ok_or_else(|| format!("path not found: {dir_path}"))
            .map_err(box_err)?
            .object()?
            .into_tree()
    };
    println!("Looking for {:#?}...", theme_names);
    for entry in subtree.iter() {
        let entry = entry?;
        let filename = entry.filename().to_str()?;
        let dest_path = theme_dir.join(filename);

        if ((entry.mode().kind() == EntryKind::Blob)
            || (entry.mode().kind() == EntryKind::BlobExecutable))
            && theme_names.contains(filename)
        {
            let blob = entry.object()?.into_blob();
            std::fs::write(&dest_path, &blob.data)?;
            println!("Saving {} to {:#?}", filename, theme_dir);
            theme_names.remove(filename);
        }
    }

    if !theme_names.is_empty() {
        println!("Was not able to find a match for {:#?}", theme_names);
    }

    Ok(())
}

fn fetch_repo(
    repo_url: &String,
    branch: &String,
    dir: &tempfile::TempDir,
) -> Result<gix::Repository> {
    println!("Accessing repo...");
    let refspec = BString::from(format!("refs/heads/{branch}:refs/heads/{branch}"));
    // Blobless 1 depth clone: fetches latest tree but not file contents, so should be quick.
    let (repo, _) = gix::prepare_clone(repo_url.to_string(), dir.path())?
        .with_shallow(gix::remote::fetch::Shallow::DepthAtRemote(1.try_into()?))
        .configure_remote(move |mut remote| {
            remote.replace_refspecs(Some(&refspec), gix::remote::Direction::Fetch)?;
            Ok(remote)
        })
        .fetch_only(gix::progress::Discard, &false.into())?;
    println!("Ok!");
    Ok(repo)
}

fn download_directory(repo: &gix::Repository, dir_path: &str, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)?;

    let head = repo.head_commit()?;
    let tree = head.tree()?;

    // Navigate to the subdirectory (or use root if dir_path is empty)
    let subtree = if dir_path.is_empty() {
        tree
    } else {
        tree.lookup_entry_by_path(dir_path)?
            .ok_or_else(|| format!("path not found: {dir_path}"))
            .map_err(box_err)?
            .object()?
            .into_tree()
    };

    for entry in subtree.iter() {
        let entry = entry?;
        let filename = entry.filename().to_str()?;
        let dest_path = dest.join(filename);

        match entry.mode().kind() {
            EntryKind::Blob | EntryKind::BlobExecutable => {
                let blob = entry.object()?.into_blob();
                std::fs::write(&dest_path, &blob.data)?;
                println!("wrote {}", dest_path.display());
            }

            _ => {}
        }
    }

    Ok(())
}
