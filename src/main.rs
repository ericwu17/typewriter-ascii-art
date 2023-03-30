use image::imageops::colorops::grayscale;
use image::{GenericImageView, ImageBuffer, Luma, Pixel};

const TARGET_IMG_HEIGHT_PIXELS: u32 = 108;
const TARGET_IMG_WIDTH_PIXELS: u32 = 68;

// pixel height (on typewriter) is 2.1 mm
// pixel width (on typewriter) is 2.5 mm

const CHAR_SET: [char; 8] = [' ', '.', ':', 'V', 'I', 'Z', 'N', 'M'];

fn main() {
    let img = image::open("IMG_1167.JPG").unwrap();
    let img = grayscale(&img);

    let (width, height) = img.dimensions();

    let target_img =
        ImageBuffer::from_fn(TARGET_IMG_WIDTH_PIXELS, TARGET_IMG_HEIGHT_PIXELS, |x, y| {
            image::Luma([get_aggregate_pixel_at(x, y, width, height, &img)])
        });
    target_img.save("test.png").unwrap();

    // for now we will use a linear scale for each character in the char set and see how it looks.

    let mut output = Vec::<String>::new();

    for row in target_img.rows() {
        let mut s = String::new();
        for pixel in row {
            let val: u8 = 255 - pixel.0[0];
            let index = val / 32;
            let char = CHAR_SET[index as usize];

            print!("{}", char);
            s += char.to_string().as_str();
        }
        output.push(s);
        println!("");
    }

    for string in output {
        let mut curr_char = string.chars().next().unwrap();
        let mut count: u32 = 1;
        for char in string.chars().skip(1) {
            if char == curr_char {
                count += 1;
            } else {
                print!("({}, {}) ", curr_char, count);
                count = 1;
                curr_char = char;
            }
        }
        print!("({}, {}) ", curr_char, count);
        println!("");
    }
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
