use chrono::Local;

use crate::{error::PackageManagerError, paths};
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Generation {
    /// ID of the generation.
    pub id: u32,
    /// Unix timestamp of the generation's creation date.
    pub created: u64,
}

impl super::PackageManager {
    pub fn commit_state(&self, message: Option<&str>) -> Result<(), PackageManagerError> {
        let state = self.state.as_ref().ok_or(PackageManagerError::StateNotLoaded)?;

        let sig = state.signature()?;

        let tree_id = {
            let mut index = state.index()?;
            index.write_tree()?
        };

        let tree = state.find_tree(tree_id)?;

        state.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("{} | {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message.unwrap_or("Automated commit")),
            &tree,
            &[],
        )?;

        Ok(())
    }

    pub fn read_generation(&self, path: impl AsRef<Path>) -> Result<Generation, PackageManagerError> {
        let path = path.as_ref();

        let id = path.display().to_string().parse::<u32>()?;
        let created = fs::read_to_string(paths::state(&self.root).join(path).join("created"))?
            .trim()
            .parse::<u64>()?;

        Ok(Generation { id, created })
    }

    pub fn list_generations(&self) -> Result<Vec<Generation>, PackageManagerError> {
        let dirs = fs::read_dir(paths::state(&self.root))?.filter_map(Result::ok);
        let mut generations = Vec::new();

        for dir in dirs {
            if dir.file_type()?.is_symlink() {
                continue;
            }

            generations.push(self.read_generation(dir.path())?);
        }

        Ok(generations)
    }

    pub fn current_generation(&self) -> Result<Generation, PackageManagerError> {
        let current = fs::read_link(paths::state(&self.root).join("current"))?;

        self.read_generation(current)
    }

    pub fn make_generation(&self) -> Result<(), PackageManagerError> {
        let current_id = self.current_generation()?.id;

        fs::create_dir(paths::state(&self.root).join((current_id + 1).to_string()))?;

        Ok(())
    }
}
