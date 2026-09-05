use crate::hex;
use crate::hex::HexError;
use crate::report::Report;
use std::path::Path;

pub fn validate_firmware(
    path: impl AsRef<Path>,
    report: &mut impl FnMut(Report),
) -> Result<(), HexError> {
    report(Report::Progress(0));
    report(Report::Log("Validating Intel HEX file...".into()));

    match hex::validate_file(path) {
        Ok(()) => {
            report(Report::Progress(100));
            report(Report::Log("Intel HEX file is valid.".into()));
            Ok(())
        }
        Err(error) => {
            report(Report::Log(format!("Validation failed: {}", error)));
            Err(error)
        }
    }
}
