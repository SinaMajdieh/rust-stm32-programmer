use super::*;

#[test]
fn output_formats_have_the_expected_tool_arguments_extensions_and_stages() {
    assert_eq!(OutputFormat::IntelHex.argument(), "ihex");
    assert_eq!(OutputFormat::IntelHex.extension(), "hex");
    assert_eq!(OutputFormat::IntelHex.stage(), BuildStage::ConvertToHex);

    assert_eq!(OutputFormat::Binary.argument(), "binary");
    assert_eq!(OutputFormat::Binary.extension(), "bin");
    assert_eq!(OutputFormat::Binary.stage(), BuildStage::ConvertToBinary);
}
