use std::{
    ffi::OsString,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use pm_common::gallery_cache::{Orientation, Ratio};

use crate::gallery::gallery_cache::PictureCache;

pub struct ExifFile {
    path: OsString,
    meta: rexiv2::Metadata,
    pub uid: String,
    pub uuid_generated: bool,
    pub date: Option<String>,
    pub location_lat: Option<f64>,
    pub location_long: Option<f64>,
    pub location_alt: Option<f64>,
    pub camera: Option<String>,
    pub orientation: Orientation,
    pub focal_length: Option<f64>,
    pub exposure_time: Option<Ratio>,
    pub iso_speed: Option<i32>,
    pub f_number: Option<f64>,
}

impl ExifFile {
    pub fn new(path: PathBuf) -> Option<Self> {
        let path = path.as_os_str().to_os_string();
        if let Ok(meta) = rexiv2::Metadata::new_from_path(path.clone()) {
            if meta.supports_exif() && meta.supports_xmp() {
                let uid;
                let uuid_generated;
                if let Ok(uid_ok) = meta.get_tag_string("Xmp.PicturesManagerClementGre.uid") {
                    uid = uid_ok;
                    uuid_generated = false;
                } else {
                    uid = gen_new_uid();
                    meta.set_tag_string("Xmp.PicturesManagerClementGre.uid", uid.as_str())
                        .expect("Unable to set UID XMP Field");
                    meta.save_to_file(path.clone()).expect("Unable to save metadata to file");
                    uuid_generated = true;
                }

                let date = meta.get_tag_string("Exif.Image.DateTime").ok();


                let mut location_lat = None;
                let mut location_long = None;
                let mut location_alt = None;
                if let Some(gps_info) = meta.get_gps_info() {
                    location_lat = Some(gps_info.latitude);
                    location_long = Some(gps_info.longitude);
                    location_alt = Some(gps_info.altitude);
                }

                let camera = meta.get_tag_string("Exif.Image.Model").ok();
                let orientation = Orientation::from_revix2(meta.get_orientation());
                let focal_length = meta.get_focal_length();
                let exposure_time = meta.get_exposure_time().map(|et| Ratio::from_num_rational(et));
                let iso_speed = meta.get_iso_speed();
                let f_number = meta.get_fnumber();

                Some(Self {
                    path,
                    meta,
                    uid,
                    uuid_generated,
                    date,
                    location_lat,
                    location_long,
                    location_alt,
                    camera,
                    orientation,
                    focal_length,
                    exposure_time,
                    iso_speed,
                    f_number,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn set_uid(&mut self, uid: String) {
        self.uid = uid.clone();
        self.meta
            .set_tag_string("Xmp.PicturesManagerClementGre.uid", uid.as_str())
            .expect("Unable to set UID");
    }
    pub fn save(&self) {
        self.meta.save_to_file(self.path.clone()).expect("Unable to save metadata");
    }

    pub fn to_picture_cache(&self, path: String) -> PictureCache {
        PictureCache {
            path,
            uuid_generated: self.uuid_generated,
            date: self.date.clone(),
            location_lat: self.location_lat,
            location_long: self.location_long,
            location_alt: self.location_alt,
            camera: self.camera.clone(),
            orientation: self.orientation,
            focal_length: self.focal_length,
            exposure_time: self.exposure_time,
            iso_speed: self.iso_speed,
            f_number: self.f_number,
        }
    }
}

trait FromRexiv2<T> {
    fn from_revix2(r: T) -> Self;
}
impl FromRexiv2<rexiv2::Orientation> for Orientation {
    fn from_revix2(orientation: rexiv2::Orientation) -> Self {
        match orientation {
            rexiv2::Orientation::Normal => Orientation::Normal,
            rexiv2::Orientation::HorizontalFlip => Orientation::HorizontalFlip,
            rexiv2::Orientation::Rotate180 => Orientation::Rotate180,
            rexiv2::Orientation::VerticalFlip => Orientation::VerticalFlip,
            rexiv2::Orientation::Rotate90HorizontalFlip => Orientation::Rotate90HorizontalFlip,
            rexiv2::Orientation::Rotate90 => Orientation::Rotate90,
            rexiv2::Orientation::Rotate90VerticalFlip => Orientation::Rotate90VerticalFlip,
            rexiv2::Orientation::Rotate270 => Orientation::Rotate270,
            _ => Orientation::Unspecified,
        }
    }
}

static UID_SEC_COUNT: AtomicU64 = AtomicU64::new(0);
static UID_LAST_SECS: AtomicU64 = AtomicU64::new(0);

// Generates a new UID based on the current time and a counter
pub fn gen_new_uid() -> String {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Reset counter if we are in a new second
    if since_the_epoch.as_secs() != UID_LAST_SECS.load(Ordering::SeqCst) {
        UID_LAST_SECS.store(since_the_epoch.as_secs(), Ordering::SeqCst);
        UID_SEC_COUNT.store(0, Ordering::SeqCst);
    }

    format!("{:X}-{:X}", since_the_epoch.as_secs(), UID_SEC_COUNT.fetch_add(1, Ordering::SeqCst))
}
