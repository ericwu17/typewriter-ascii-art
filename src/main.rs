use image::imageops::colorops::grayscale;
use image::{GenericImageView, ImageBuffer, Luma, Pixel};

const TARGET_IMG_HEIGHT_PIXELS: u32 = 100;
const TARGET_IMG_WIDTH_PIXELS: u32 = 100;

fn main() {
    let img = image::open("IMG_1167.JPG").unwrap();
    let img = grayscale(&img);

    let (width, height) = img.dimensions();

    let target_img =
        ImageBuffer::from_fn(TARGET_IMG_WIDTH_PIXELS, TARGET_IMG_HEIGHT_PIXELS, |x, y| {
            image::Luma([get_aggregate_pixel_at(x, y, width, height, &img)])
        });
    target_img.save("test.png").unwrap();
}

fn get_aggregate_pixel_at<I: GenericImageView<Pixel = Luma<u8>>>(
    x: u32,
    y: u32,
    source_width: u32,
    source_height: u32,
    img: &I,
) -> u8 {
    // takes a position in the TARGET image and then returns what the value of that position
    // should be. Calculated by averaging the patch in the source img corresponding to the location
    // in the target image (remember, we assume that the target image is of lower resolution than the source img)

    // restraints on inputs: 0 <= x < TARGET_IMG_WIDTH_PIXELS
    //                       0 <= y < TARGET_IMG_HEIGHT_PIXELS

    let x_begin: u32 = (x * source_width) / TARGET_IMG_WIDTH_PIXELS;
    let y_begin: u32 = (y * source_height) / TARGET_IMG_HEIGHT_PIXELS;
    let x_end: u32 = ((x + 1) * source_width) / TARGET_IMG_WIDTH_PIXELS;
    let y_end: u32 = ((y + 1) * source_height) / TARGET_IMG_HEIGHT_PIXELS;

    let mut acc: u32 = 0;
    for x in x_begin..x_end {
        for y in y_begin..y_end {
            let p = img.get_pixel(x, y).to_luma();
            acc += p.0[0] as u32;
        }
    }

    let area = (x_end - x_begin) * (y_end - y_begin);

    return (acc / area) as u8;
}
