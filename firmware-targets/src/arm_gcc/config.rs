use std::{
    io,
    path::{Path, PathBuf},
};

/// Configuration for building a project with the ARM GNU toolchain.
#[derive(Debug, Clone)]
pub struct ArmGccConfig {
    compiler: PathBuf,
    objcopy: PathBuf,
    cpu: String,
    defines: Vec<String>,
    include_dirs: Vec<PathBuf>,
    linker_script: PathBuf,
}

impl ArmGccConfig {
    /// Creates an ARM GCC build configuration.
    ///
    /// Include directories and the linker script are relative to the
    /// generated project's root directory.
    pub fn new(
        compiler: impl Into<PathBuf>,
        objcopy: impl Into<PathBuf>,
        cpu: impl Into<String>,
        defines: impl IntoIterator<Item = impl Into<String>>,
        include_dirs: impl IntoIterator<Item = impl Into<PathBuf>>,
        linker_script: impl Into<PathBuf>,
    ) -> Self {
        Self {
            compiler: compiler.into(),
            objcopy: objcopy.into(),
            cpu: cpu.into(),
            defines: defines.into_iter().map(Into::into).collect(),
            include_dirs: include_dirs.into_iter().map(Into::into).collect(),
            linker_script: linker_script.into(),
        }
    }

    /// Returns the ARM GCC executable.
    pub fn compiler(&self) -> &Path {
        &self.compiler
    }

    /// Returns the ARM objcopy executable.
    pub fn objcopy(&self) -> &Path {
        &self.objcopy
    }

    /// Returns the target CPU name.
    pub fn cpu(&self) -> &str {
        &self.cpu
    }

    /// Returns the C preprocessor definitions.
    pub fn defines(&self) -> &[String] {
        &self.defines
    }

    /// Returns the project-relative include directories.
    pub fn include_dirs(&self) -> &[PathBuf] {
        &self.include_dirs
    }

    /// Returns the project-relative linker script path.
    pub fn linker_script(&self) -> &Path {
        &self.linker_script
    }

    pub(crate) fn validate_project_layout(&self, project_root: &Path) -> io::Result<()> {
        for include_dir in &self.include_dirs {
            let path = project_root.join(include_dir);

            if !path.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("include directory does not exist: {}", path.display()),
                ));
            }
        }

        let linker_script = project_root.join(&self.linker_script);

        if !linker_script.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("linker script does not exist: {}", linker_script.display()),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
