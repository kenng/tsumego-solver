use std::fs;
use std::io;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tsumego_solver::generation::generate_puzzle;
use tsumego_solver::go::GoBoard;
use uuid::Uuid;

pub fn run(output_directory: &Path, thread_count: u8) -> io::Result<()> {
    fs::create_dir_all(output_directory)?;

    let (tx, rx) = channel::<GoBoard>();

    for _ in 0..thread_count {
        let tx = tx.clone();
        thread::spawn(move || loop {
            let puzzle = generate_puzzle(Duration::from_secs(1));
            tx.send(puzzle).unwrap();
        });
    }

    loop {
        let puzzle = rx.recv().unwrap();
        let file = output_directory.join(format!("{}.sgf", Uuid::new_v4()));
        fs::write(file.as_path(), puzzle.to_sgf())?;

        println!("Generated {}", file.display());
    }
}
