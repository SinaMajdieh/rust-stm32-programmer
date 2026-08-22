use std::{
    io,
    path::{Path, PathBuf},
};

/// Configuration for building a project with the ARM GNU toolchain.
///
/// Paths supplied for include directories and the linker script are interpreted
/// relative to the generated project's root directory.
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
    /// `compiler` and `objcopy` identify the external toolchain executables.
    /// `cpu` specifies the target ARM processor, while `defines` and
    /// `include_dirs` configure preprocessing and header lookup during
    /// compilation.
    ///
    /// Include directories and the linker script are interpreted relative to
    /// the generated project's root directory.
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

    /// Returns the configured ARM GCC compiler executable.
    pub fn compiler(&self) -> &Path {
        &self.compiler
    }

    /// Returns the configured ARM `objcopy` executable.
    pub fn objcopy(&self) -> &Path {
        &self.objcopy
    }

    /// Returns the target CPU name passed to the compiler and linker.
    pub fn cpu(&self) -> &str {
        &self.cpu
    }

    /// Returns the C preprocessor definitions passed to the compiler.
    pub fn defines(&self) -> &[String] {
        &self.defines
    }

    /// Returns the project-relative directories searched for header files.
    pub fn include_dirs(&self) -> &[PathBuf] {
        &self.include_dirs
    }

    /// Returns the project-relative path to the linker script.
    pub fn linker_script(&self) -> &Path {
        &self.linker_script
    }

    /// Validates that all files and directories required by the build exist.
    ///
    /// Include directories are required to exist as directories, and the
    /// configured linker script is required to exist as a regular file.
    ///
    /// This validation is performed before compilation begins so a malformed
    /// project layout fails early rather than halfway through the build.
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
