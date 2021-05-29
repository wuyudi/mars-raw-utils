use crate::{
    vprintln, 
    rgbimage, 
    enums, 
    path,
    constants,
    util
};

pub fn process_file(input_file:&str,  only_new:bool) {
    let out_file = util::append_file_name(input_file, constants::OUTPUT_FILENAME_APPEND);
    if path::file_exists(&out_file) && only_new {
        vprintln!("Output file exists, skipping. ({})", out_file);
        return;
    }
    
    let mut raw = rgbimage::RgbImage::open(String::from(input_file), enums::Instrument::M20HeliRte).unwrap();
    

    let data_max = 255.0;

    //if ! no_ilt {
    //    vprintln!("Decompanding...");
    //    raw.decompand().unwrap();
    //    data_max = decompanding::get_max_for_instrument(enums::Instrument::M20Watson) as f32;
    //}

    vprintln!("Flatfielding...");
    raw.flatfield().unwrap();

    vprintln!("Normalizing...");
    raw.normalize_to_16bit_with_max(data_max).unwrap();

    vprintln!("Writing to disk...");
    raw.save(&out_file).unwrap();
}