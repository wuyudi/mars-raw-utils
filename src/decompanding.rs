
use crate::{imagebuffer::ImageBuffer, constants, enums, error, ok};

pub const ILT : [u32; 256] = [0, 2, 3, 3, 4, 5, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16,
                                18, 19, 20, 22, 24, 25, 27, 29, 31, 33, 35, 37, 39, 41,
                                43, 46, 48, 50, 53, 55, 58, 61, 63, 66, 69, 72, 75, 78,
                                81, 84, 87, 90, 94, 97, 100, 104, 107, 111, 115, 118, 122,
                                126, 130, 134, 138, 142, 146, 150, 154, 159, 163, 168, 172,
                                177, 181, 186, 191, 196, 201, 206, 211, 216, 221, 226, 231,
                                236, 241, 247, 252, 258, 263, 269, 274, 280, 286, 292, 298,
                                304, 310, 316, 322, 328, 334, 341, 347, 354, 360, 367, 373,
                                380, 387, 394, 401, 408, 415, 422, 429, 436, 443, 450, 458,
                                465, 472, 480, 487, 495, 503, 510, 518, 526, 534, 542, 550,
                                558, 566, 575, 583, 591, 600, 608, 617, 626, 634, 643, 652,
                                661, 670, 679, 688, 697, 706, 715, 724, 733, 743, 752, 761,
                                771, 781, 790, 800, 810, 819, 829, 839, 849, 859, 869, 880,
                                890, 900, 911, 921, 932, 942, 953, 964, 974, 985, 996, 1007,
                                1018, 1029, 1040, 1051, 1062, 1074, 1085, 1096, 1108, 1119,
                                1131, 1142, 1154, 1166, 1177, 1189, 1201, 1213, 1225, 1237,
                                1249, 1262, 1274, 1286, 1299, 1311, 1324, 1336, 1349, 1362,
                                1374, 1387, 1400, 1413, 1426, 1439, 1452, 1465, 1479, 1492,
                                1505, 1519, 1532, 1545, 1559, 1573, 1586, 1600, 1614, 1628,
                                1642, 1656, 1670, 1684, 1698, 1712, 1727, 1741, 1755, 1770,
                                1784, 1799, 1814, 1828, 1843, 1858, 1873, 1888, 1903, 1918,
                                1933, 1948, 1963, 1979, 1994, 2009, 2025, 2033];


// https://pds-imaging.jpl.nasa.gov/data/nsyt/insight_cameras/calibration/ilut/
pub const NSYT_ILT : [u32; 256] = [
                                0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 8, 9, 11, 12,
                                14, 15, 17, 19, 21, 23, 25, 27, 29, 32, 34, 37, 40, 43, 46,
                                49, 52, 55, 59, 62, 66, 70, 73, 77, 82, 86, 90, 95, 99, 104,
                                109, 114, 119, 124, 129, 135, 140, 146, 152, 158, 164, 170,
                                176, 182, 189, 196, 202, 209, 216, 224, 231, 238, 246, 254,
                                261, 269, 277, 286, 294, 302, 311, 320, 328, 337, 347, 356,
                                365, 375, 384, 394, 404, 414, 424, 435, 445, 456, 467, 477,
                                488, 500, 511, 522, 534, 545, 557, 569, 581, 594, 606, 619,
                                631, 644, 657, 670, 683, 697, 710, 724, 738, 752, 766, 780,
                                794, 809, 823, 838, 853, 868, 884, 899, 914, 930, 946, 962,
                                978, 994, 1011, 1027, 1044, 1061, 1078, 1095, 1112, 1130, 
                                1147, 1165, 1183, 1201, 1219, 1237, 1256, 1274, 1293, 1312,
                                1331, 1350, 1370, 1389, 1409, 1429, 1449, 1469, 1489, 1509,
                                1530, 1551, 1572, 1593, 1614, 1635, 1657, 1678, 1700, 1722,
                                1744, 1766, 1789, 1811, 1834, 1857, 1880, 1903, 1926, 1950,
                                1974, 1997, 2021, 2045, 2070, 2094, 2119, 2144, 2168, 2193,
                                2219, 2244, 2270, 2295, 2321, 2347, 2373, 2400, 2426, 2453,
                                2479, 2506, 2534, 2561, 2588, 2616, 2644, 2671, 2700, 2728,
                                2756, 2785, 2813, 2842, 2871, 2900, 2930, 2959, 2989, 3019,
                                3049, 3079, 3109, 3140, 3170, 3201, 3232, 3263, 3295, 3326,
                                3358, 3390, 3421, 3454, 3486, 3518, 3551, 3584, 3617, 3650,
                                3683, 3716, 3750, 3784, 3818, 3852, 3886, 3920, 3955, 3990,
                                4025, 4060, 4095];


fn get_ilt_for_instrument(instrument:enums::Instrument) -> [u32; 256] {

    match instrument {
        enums::Instrument::NsytICC => NSYT_ILT,
        enums::Instrument::NsytIDC => NSYT_ILT,
        _ => ILT
    }
}

pub fn get_max_for_instrument(instrument:enums::Instrument) -> u32 {
    let ilt = get_ilt_for_instrument(instrument);
    ilt[255]
}

pub fn decompand_buffer(buffer:&mut ImageBuffer, instrument:enums::Instrument) -> error::Result<&str> {

    let ilt = get_ilt_for_instrument(instrument);

    for x in 0..buffer.width {
        for y in 0..buffer.height {
            let raw_value = buffer.get(x, y).unwrap() as usize;
            if raw_value > 255 {
                return Err(constants::status::INVALID_RAW_VALUE);
            }
            let ilt_value = ilt[raw_value];
            buffer.put(x, y, ilt_value as f32).unwrap();
        }
    }
    ok!()
}

// This method backs out the ILT table and is inefficient. Use sparingly
fn get_lut_value_from_ilt_value(ilt_value:u32, instrument:enums::Instrument) -> u32 {
    let ilt = get_ilt_for_instrument(instrument);

    if ilt_value == 0 {
        return 0;
    }

    for i in 1..ilt.len() {
        if ilt_value == ilt[i] || (ilt_value < ilt[i] && ilt_value > ilt[i - 1]) {
            return i as u32;
        } 
    }

    0
}

// This method backs out the ILT table and is inefficient. Use sparingly
pub fn compand_buffer(buffer:&mut ImageBuffer, instrument:enums::Instrument) -> error::Result<&str> {

    for x in 0..buffer.width {
        for y in 0..buffer.height {
            let ilt_value = buffer.get(x, y).unwrap();
            let lut_value = get_lut_value_from_ilt_value(ilt_value as u32, instrument);
            buffer.put(x, y, lut_value as f32).unwrap();
        }
    }

    ok!()
}


