use programmer_core::program_file;

fn main() -> std::io::Result<()> {
    program_file("example.txt", |progress| {
        println!(
            "{}% {:?}",
            progress.percentage,
            progress.text_chunk,
        );
    })
}