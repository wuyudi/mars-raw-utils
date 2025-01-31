use crate::{
    constants, jsonfetch,
    metadata::convert_to_std_metadata,
    msl::latest::{Latest, LatestData},
    msl::metadata::*,
    remotequery::RemoteQuery,
    util::*,
};

use sciimg::path;

use anyhow::{anyhow, Result};

pub fn print_header() {
    println!(
        "{:37} {:15} {:6} {:20} {:27} {:6} {:6} {:7} {:10}",
        "ID",
        "Instrument",
        "Sol",
        "Image Date (UTC)",
        "Image Date (Mars)",
        "Site",
        "Drive",
        "Thumb",
        "Present"
    );
}

fn null_to_str<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        None => String::from(""),
        Some(v) => {
            format!("{}", v)
        }
    }
}

fn print_image(output_path: &str, image: &ImageRecord) {
    let p = format!("{}/{}", output_path, path::basename(&image.url));

    println!(
        "{:37} {:15} {:<6} {:20} {:27} {:6} {:6} {:7} {:10}",
        image.imageid,
        image.instrument,
        image.sol, // This is such a hack...
        &image.date_taken[..16],
        null_to_str(&image.extended.lmst),
        null_to_str(&image.site),
        null_to_str(&image.drive),
        if image.is_thumbnail {
            constants::status::YES
        } else {
            constants::status::NO
        },
        if path::file_exists(&p) {
            constants::status::YES
        } else {
            constants::status::NO
        }
    );
}

async fn process_results(results: &MslApiResults, query: &RemoteQuery) -> usize {
    let mut valid_img_count = 0;
    let images = results.items.iter().filter(|image| {
        !(image.is_thumbnail && !query.thumbnails
            || !query.search.is_empty() && !query.search.iter().any(|i| image.imageid.contains(i)))
    });
    for (idx, image) in images.enumerate() {
        valid_img_count = idx;
        print_image(query.output_path.as_str(), image);
        if !query.list_only {
            _ = fetch_image(&image.url, query.only_new, Some(query.output_path.as_str())).await;
            let image_base_name = path::basename(image.url.as_str());
            _ = save_image_json(
                &image_base_name,
                &convert_to_std_metadata(image),
                query.only_new,
                Some(query.output_path.as_str()),
            );
        }
    }
    valid_img_count
}

pub fn make_instrument_map() -> InstrumentMap {
    InstrumentMap {
        map: [
            (
                "HAZ_FRONT",
                vec!["FHAZ_RIGHT_A", "FHAZ_LEFT_A", "FHAZ_RIGHT_B", "FHAZ_LEFT_B"],
            ),
            (
                "HAZ_REAR",
                vec!["RHAZ_RIGHT_A", "RHAZ_LEFT_A", "RHAZ_RIGHT_B", "RHAZ_LEFT_B"],
            ),
            ("NAV_LEFT", vec!["NAV_LEFT_A", "NAV_LEFT_B"]),
            ("NAV_RIGHT", vec!["NAV_RIGHT_A", "NAV_RIGHT_B"]),
            ("CHEMCAM", vec!["CHEMCAM_RMI"]),
            ("MARDI", vec!["MARDI"]),
            ("MAHLI", vec!["MAHLI"]),
            ("MASTCAM", vec!["MAST_LEFT", "MAST_RIGHT"]),
        ]
        .iter()
        .cloned()
        .collect(),
    }
}

async fn submit_query(query: &RemoteQuery) -> Result<String> {
    let mut params = vec![
        stringvec("condition_1", "msl:mission"),
        stringvec_b("per_page", format!("{}", query.num_per_page)),
        stringvec(
            "order",
            "sol desc,instrument_sort asc,sample_type_sort asc, date_taken desc",
        ),
        stringvec_b("search", query.cameras.join("|")),
        stringvec_b("condition_2", format!("{}:sol:gte", query.minsol)),
        stringvec_b("condition_3", format!("{}:sol:lte", query.maxsol)),
    ];

    if let Some(p) = query.page {
        params.push(stringvec_b("page", format!("{}", p)));
    }

    let uri = constants::url::MSL_RAW_WEBSERVICE_URL;

    if let Ok(mut req) = jsonfetch::JsonFetcher::new(uri) {
        for p in params {
            req.param(p[0].as_str(), p[1].as_str());
        }
        return Ok(req.fetch_str().await?);
    }

    Err(anyhow!("Unable to submit query."))
}

pub async fn fetch_page(query: &RemoteQuery) -> Result<usize> {
    match submit_query(query).await {
        Ok(v) => match serde_json::from_str(&v) {
            Ok(res) => Ok(process_results(&res, query).await),
            // NOTE: The anyhow! macro is glorious for making on-the-fly errors in application code, there's a sister libary called thiserror which is for making explicit libary code error types.
            Err(e) => Err(anyhow!("Serde parsing from_str failed. {}", e)),
        },
        Err(e) => Err(anyhow!("Serde parsing from_str failed. {}", e)),
    }
}

#[derive(Debug, Clone)]
pub struct MslRemoteStats {
    pub more: bool,
    pub total: i32,
    pub page: i32,
    pub per_page: i32,
}

pub async fn fetch_stats(query: &RemoteQuery) -> Result<MslRemoteStats> {
    match submit_query(query).await {
        Ok(v) => {
            let res: MslApiResults = serde_json::from_str(v.as_str()).unwrap();
            Ok(MslRemoteStats {
                more: res.more,
                total: res.total as i32,
                page: res.page as i32,
                per_page: res.per_page as i32,
            })
        }
        Err(e) => Err(e),
    }
}

pub async fn fetch_latest() -> Result<LatestData> {
    let uri = constants::url::MSL_LATEST_WEBSERVICE_URL;

    let req = jsonfetch::JsonFetcher::new(uri)?;
    match req.fetch_str().await {
        Ok(v) => {
            let res: Latest = serde_json::from_str(v.as_str())?;
            if res.success {
                Ok(res.latest_data)
            } else {
                Err(anyhow!("Server error"))
            }
        }
        Err(_e) => Err(anyhow!("JsonFetcher fetch_str() failed")),
    }
}

pub async fn fetch_all(query: &RemoteQuery) -> Result<usize> {
    let stats = match fetch_stats(query).await {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let pages = (stats.total as f32 / query.num_per_page as f32).ceil() as i32;

    let mut count = 0;
    for page in 0..pages {
        let mut q: RemoteQuery = query.clone();
        q.page = Some(page);
        match fetch_page(&q).await {
            Ok(c) => {
                count += c;
            }
            Err(e) => return Err(e),
        };
    }

    Ok(count)
}

pub async fn remote_fetch(query: &RemoteQuery) -> Result<usize> {
    if query.page.is_some() {
        fetch_page(query).await
    } else {
        fetch_all(query).await
    }
}
