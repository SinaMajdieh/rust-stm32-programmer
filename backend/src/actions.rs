use crate::hex;
use crate::hex::HexError;
use crate::report::Report;
use std::path::Path;

/// Validates an Intel HEX firmware file and reports progress and status.
///
/// The validator reports progress through `report` while checking the file.
/// A successful validation reports 100% progress and a success message.
/// Validation failures are reported and returned unchanged.
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
