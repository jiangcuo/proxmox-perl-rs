#[perlmod::package(name = "PMG::RS::APT::Repositories")]
mod export {
    use anyhow::Error;

    use crate::common::apt::repositories::export as common;

    /// Get information about configured and standard repositories.
    #[export]
    pub fn repositories() -> Result<common::RepositoriesResult, Error> {
        common::repositories("pmg")
    }

    /// Add the repository identified by the `handle`.
    /// If the repository is already configured, it will be set to enabled.
    ///
    /// The `digest` parameter asserts that the configuration has not been modified.
    #[export]
    pub fn add_repository(handle: &str, digest: Option<&str>) -> Result<(), Error> {
        common::add_repository(handle, "pmg", digest)
    }

    /// Change the properties of the specified repository.
    ///
    /// The `digest` parameter asserts that the configuration has not been modified.
    #[export]
    pub fn change_repository(
        path: &str,
        index: usize,
        options: common::ChangeProperties,
        digest: Option<&str>,
    ) -> Result<(), Error> {
        common::change_repository(path, index, options, digest)
    }
}
