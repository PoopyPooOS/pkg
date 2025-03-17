use crate::error::PackageManagerError;
use tl::Source;

#[allow(dead_code, unused_variables, unreachable_code)]
impl super::PackageManager {
    pub(crate) fn nushell_run(&self, source: impl Into<Source>) -> Result<(), PackageManagerError> {
        todo!("fuck this peice of shit")
    }
}
