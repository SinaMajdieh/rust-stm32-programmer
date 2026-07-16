use std::{
    fs::File, 
    io::{self, BufRead, BufReader}, 
    path::Path,
    thread,
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct ProgrammingProgress {
    pub percentage: i32,
    pub text_chunk: String
}

pub fn program_file(
    file_path: impl AsRef<Path>,
    mut report_progress: impl FnMut(ProgrammingProgress),
) -> io::Result<()> {
    let file = File::open(file_path)?;
    let total_bytes = file.metadata()?.len();

    let mut reader = BufReader::new(file);
    let mut bytes_read = 0_u64;
    let mut line = String::new();

    loop {
        line.clear();

        let read_count = reader.read_line(&mut line)?;

        if read_count == 0 {
            break;
        }

        bytes_read += read_count as u64;

        let percentage = if total_bytes == 0 {
            100
        } else {
            ((bytes_read * 100) / total_bytes) as i32
        };

        report_progress(ProgrammingProgress {
            percentage,
            text_chunk: line.clone(),
        });

        thread::sleep(Duration::from_millis(100));
    }

    report_progress(ProgrammingProgress {
        percentage: 100,
        text_chunk: String::new(),
    });
    
    Ok(())
}