use std::{
    ffi::OsString,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use pm_common::gallery_cache::Orientation;

use crate::gallery::gallery_cache::PictureCache;

pub struct ExifFile {
    pub path: OsString,
    meta: rexiv2::Metadata,
    pub uid: String,
    pub uuid_generated: bool,
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

                Some(Self {
                    path,
                    meta,
                    uid,
                    uuid_generated,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_date(&self) -> Option<String> {
        self.meta.get_tag_string("Exif.Photo.DateTimeOriginal").ok()
    }
    pub fn get_location(&self) -> Option<(f64, f64, f64)> {
        if let Some(gps_info) = self.meta.get_gps_info() {
            Some((gps_info.latitude, gps_info.longitude, gps_info.altitude))
        } else {
            None
        }
    }
    pub fn get_camera(&self) -> Option<String> {
        self.meta.get_tag_string("Exif.Image.Model").ok()
    }
    pub fn get_orientation(&self) -> Orientation {
        Orientation::from_rexiv2(self.meta.get_orientation())
    }

    pub fn get_focal_length(&self) -> Option<f64> {
        self.meta.get_focal_length()
    }
    pub fn get_exposure_time(&self) -> Option<(u32, u32)> {
        self.meta.get_exposure_time().map(|et| (*et.numer() as u32, *et.denom() as u32))
    }
    pub fn get_iso_speed(&self) -> Option<i32> {
        self.meta.get_iso_speed()
    }
    pub fn get_f_number(&self) -> Option<f64> {
        self.meta.get_fnumber()
    }
    // Does not takes into account orientation
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.meta.get_pixel_width() as u32, self.meta.get_pixel_height() as u32)
    }

    pub fn to_picture_cache(&self, path: String) -> PictureCache {
        PictureCache {
            path,
            uuid_generated: self.uuid_generated,
            date: self.get_date(),
            location: self.get_location(),
            orientation: self.get_orientation(),
            dimensions: self.get_dimensions(),
            camera: self.get_camera(),
            focal_length: self.get_focal_length(),
            exposure_time: self.get_exposure_time(),
            iso_speed: self.get_iso_speed(),
            f_number: self.get_f_number(),
        }
    }
}

trait FromRexiv2<T> {
    fn from_rexiv2(r: T) -> Self;
}
impl FromRexiv2<rexiv2::Orientation> for Orientation {
    fn from_rexiv2(orientation: rexiv2::Orientation) -> Self {
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
