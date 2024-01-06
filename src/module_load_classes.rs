use anyhow::Result;
use log::{info, error};

pub fn run(input_file : String, output_file : String) {
    info!("Starting of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);

    let sub_run_result = sub_run();

    if sub_run_result.is_err() {
        error!("{}", sub_run_result.err().unwrap());
        error!("Panic : Error during loading of a input file");
        panic!("Error during loading of a input file");
    }

    info!("End of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);
}

fn sub_run() -> Result<()> {
    
    Ok(())
}