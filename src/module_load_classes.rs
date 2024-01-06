use anyhow::Result;
use log::info;

pub fn run(input_file : String, output_file : String) {
    info!("Starting of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);
    if subrun().is_err() {
        panic!("ghjk");
    }

    info!("End of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);
}

fn subrun() -> Result<()> {
    Ok(())
}