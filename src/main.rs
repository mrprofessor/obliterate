use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    //pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let path_exists = obliterate::file_io::path_exists(&args.path);
    println!("Does file {} exist? --> {}", args.path.display(), path_exists);

    // Check whether the path is a file or a directory
    let is_a_file = obliterate::file_io::is_a_file(&args.path);

    if !is_a_file {
        println!("Can't obliterate {}", args.path.display());
        println!("{} is a directory", args.path.display());
    } else {
        // Delete the file
        obliterate::file_io::delete_file(&args.path)?;

        println!("Obliterating {}", args.path.display());
        println!("Done!")
    }



    Ok(())
}
