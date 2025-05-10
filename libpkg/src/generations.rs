use crate::error::{Context, PackageManagerError};
use std::{env, fs, os::unix::fs::symlink, path::Path};

type GenerationId = u32;

#[derive(Debug)]
pub struct Generation {
    /// ID of the generation.
    pub id: GenerationId,
    /// Unix timestamp of the generation's creation date.
    pub created: u64,
}

impl super::PackageManager {
    pub fn commit_generation(&self) -> Result<(), PackageManagerError> {
        todo!("update state");
    }

    pub fn set_current_generation(&self, id: GenerationId) -> Result<(), PackageManagerError> {
        let old_cwd = env::current_dir().context("set_current_generation: read current directory")?;

        env::set_current_dir(self.generations()).context("set_current_generation: set current directory to generations")?;
        symlink(id.to_string(), "current").context("set_current_generation: update the current generation symlink")?;
        env::set_current_dir(old_cwd).context("set_current_generation: set current directory back to old one")?;

        Ok(())
    }

    pub fn read_generation(&self, path: impl AsRef<Path>) -> Result<Generation, PackageManagerError> {
        let path = path.as_ref();

        let id = path.display().to_string().parse::<u32>()?;
        let created = fs::read_to_string(self.generations().join(path).join("created"))
            .context("read_generation: read creation date for generation")?
            .trim()
            .parse::<u64>()?;

        Ok(Generation { id, created })
    }

    pub fn current_generation(&self) -> Result<Generation, PackageManagerError> {
        let current = fs::read_link(self.generations().join("current")).context("current_generation: dereference the 'current' symlink")?;

        self.read_generation(current)
    }

    pub fn list_generations(&self) -> Result<Vec<Generation>, PackageManagerError> {
        let dirs = fs::read_dir(self.generations())
            .context("list_generations: list the directories in the generations")?
            .filter_map(Result::ok);
        let mut generations = Vec::new();

        for dir in dirs {
            if !dir.file_type().context("list_generations: skip entries that aren't directories")?.is_dir() {
                continue;
            }

            generations.push(self.read_generation(dir.path())?);
        }

        Ok(generations)
    }

    pub fn make_generation(&self) -> Result<(), PackageManagerError> {
        let current_id = self.current_generation()?.id;

        fs::create_dir(self.generations().join((current_id + 1).to_string())).context("make_generation: create directory for new directory for the new generation")?;

        Ok(())
    }
}
