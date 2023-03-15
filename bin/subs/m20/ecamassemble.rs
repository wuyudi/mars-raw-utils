use mars_raw_utils::prelude::*;

use crate::subs::runnable::RunnableSubcommand;

use mars_raw_utils::m20::ncamtile;

use std::process;

#[derive(clap::Args)]
#[clap(author, version, about = "Reassemble M20 ECAM subframes", long_about = None)]
pub struct M20EcamAssemble {
    #[clap(
        long,
        short,
        parse(from_os_str),
        help = "Input raw images",
        multiple_values(true)
    )]
    input_files: Vec<std::path::PathBuf>,

    #[clap(long, short, parse(from_os_str), help = "Output image")]
    output: std::path::PathBuf,
}

#[async_trait::async_trait]
impl RunnableSubcommand for M20EcamAssemble {
    async fn run(&self) {
        let in_files: Vec<String> = self
            .input_files
            .iter()
            .map(|s| String::from(s.as_os_str().to_str().unwrap()))
            .collect();
        //let output = self.output.as_os_str().to_str().unwrap();

        let mut tiles: Vec<MarsImage> = vec![];
        for in_file in in_files.iter() {
            if !path::file_exists(in_file) {
                eprintln!("File not found: {}", in_file);
                process::exit(1);
            }
            let mut image = MarsImage::open(String::from(in_file), Instrument::M20NavcamRight);
            image.destretch_image();

            tiles.push(image);
        }

        ncamtile::match_levels(&mut tiles);

        for tile in tiles {
            if let Some(file_path) = &tile.file_path {
                let out_file = util::append_file_name(file_path, "matched");
                vprintln!("Writing to disk...");

                tile.save(&out_file);
            }
        }

        /*
        let mut tiles: Vec<Tile> = vec![];

        for in_file in in_files.iter() {
            if !path::file_exists(in_file) {
                eprintln!("File not found: {}", in_file);
                process::exit(1);
            }
            let tile = Tile::new(in_file);
            tiles.push(tile);
        }

        // TODO: This is bad form.
        vprintln!("Creating composite structure");
        let mut composite = Composite::new(&tiles);

        vprintln!("Adding {} tiles to composite", tiles.len());
        composite.paste_tiles(&tiles);

        vprintln!("Saving composite to {}", output);
        composite.finalize_and_save(output);
        */
    }
}
