use std::env;
use std::path::PathBuf;

use pastelogue::{CatalogueProcessor, ProcessingStatus};

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = PathBuf::from(&args[1]);

    println!("Scanning catalogue ({})", root_path.display());

    let catalogue_processor = CatalogueProcessor::new(&root_path);
    println!(
        "Scanning completed with {} files to process",
        catalogue_processor.len(),
    );

    for processing_info in catalogue_processor {
        if processing_info.status == ProcessingStatus::BadMetadata {
            println!(
                "File {} has malformed or missing metadata",
                processing_info.path.display(),
            );
        }

        println!(
            "[{}/{}] Processed file: {}",
            processing_info.current,
            processing_info.total,
            processing_info.path.display()
        );
    }

    println!("Finished processing catalogue ({})", root_path.display());
}
