use photon_rs::multiple::watermark;
use photon_rs::transform::{crop, resize};
use photon_rs::PhotonImage;

pub fn crop_image(image: PhotonImage) -> PhotonImage {
    let mut image = image;
    let height = image.get_height();
    let width = image.get_width();

    if width < height {
        // find the difference between the two sides
        let diff = (height - width) / 2;
        crop(&mut image, 0, diff, width, height - diff)
    } else {
        let diff = (width - height) / 2;
        crop(&mut image, diff, 0, width - diff, height)
    }
}

pub fn watermark_image(image: PhotonImage, water_mark: PhotonImage) -> PhotonImage {
    let height = image.get_height();
    let width = image.get_width();

    let wm_height = water_mark.get_height();
    let wm_width = water_mark.get_width();

    let quarter_h = (height + wm_height) / 7;
    let quarter_w = (width + wm_width) / 7;

    // place watermark on images at different positions of the image
    let mut image = image;
    watermark(&mut image, &water_mark, 3 * quarter_w, quarter_h);
    watermark(&mut image, &water_mark, 3 * quarter_w, 3 * quarter_h);
    watermark(&mut image, &water_mark, 3 * quarter_w, 5 * quarter_h);
    watermark(&mut image, &water_mark, quarter_w, quarter_h);
    watermark(&mut image, &water_mark, quarter_w, 3 * quarter_h);
    watermark(&mut image, &water_mark, quarter_w, 5 * quarter_h);
    watermark(&mut image, &water_mark, 5 * quarter_w, quarter_h);
    watermark(&mut image, &water_mark, 5 * quarter_w, 3 * quarter_h);
    watermark(&mut image, &water_mark, 5 * quarter_w, 5 * quarter_h);

    image
}

pub fn process_image(water_mark: PhotonImage, img: PhotonImage) -> PhotonImage {
    // resize watermark
    let resized_watermark = resize(
        &water_mark,
        50,
        50,
        photon_rs::transform::SamplingFilter::Nearest,
    );

    // crop image to get it square
    let cropped = crop_image(img);
    let resized = resize(
        &cropped,
        400,
        400,
        photon_rs::transform::SamplingFilter::Nearest,
    );
    watermark_image(resized, resized_watermark)
}
