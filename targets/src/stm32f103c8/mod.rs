//! Project generation support for the STM32F103C8 target.
mod project;
mod template;

pub use project::Project;
pub use template::Template;

#[cfg(test)]
mod tests {
    use super::template::*;
    use include_dir::Dir;
    use std::{fs, io, path::Path};
    use tempfile::tempdir;

    #[test]
    fn generate_project_from_embedded_template() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("generated").join("blink");

        let project = Template::new().generate(output.clone())?;

        assert_eq!(project.root(), output);
        assert!(project.root().is_dir());

        assert_template_was_extracted(&TEMPLATE, project.root());

        Ok(())
    }

    #[test]
    fn does_not_overwrite_an_existing_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("overwrite");

        let target = Template::new();
        target.generate(output.clone())?;

        let error = match target.generate(output) {
            Ok(_) => panic!("Generation should not overwrite and existing project"),
            Err(err) => err,
        };

        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    fn assert_template_was_extracted(template: &Dir<'_>, output: &Path) {
        for file in template.files() {
            let generated_file = output.join(file.path());

            assert!(
                generated_file.is_file(),
                "Expected generated file: {}",
                generated_file.display()
            );
        }

        for dir in template.dirs() {
            let generated_directory = output.join(dir.path());

            assert!(
                generated_directory.is_dir(),
                "expected generated directory: {}",
                generated_directory.display()
            );

            assert_template_was_extracted(dir, output);
        }
    }
    #[test]
    fn adds_source_to_generated_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("project");

        let mut project = Template::new().generate(output)?;
        project.add_source("main.c", "int main(void) { return 0; }")?;

        let main = project.root().join("src/main.c");

        assert!(main.is_file());
        assert_eq!(fs::read_to_string(&main)?, "int main(void) { return 0; }");
        assert!(project.sources().contains(&main));

        Ok(())
    }
}
