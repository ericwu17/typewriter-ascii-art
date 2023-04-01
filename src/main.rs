use image::imageops::colorops::grayscale;
use image::{GenericImageView, ImageBuffer, Luma, Pixel};

const TARGET_IMG_HEIGHT_PIXELS: u32 = 108;
const TARGET_IMG_WIDTH_PIXELS: u32 = 68;

// pixel height (on typewriter) is 2.1 mm
// pixel width (on typewriter) is 2.5 mm

// The 'o' actually represents a space on the typewriter
const CHAR_SET: [char; 8] = ['o', '.', ':', ';', 'I', 'V', 'N', 'M'];
const CHAR_BRIGHTNESSES: [i32; 8] = [255, 219, 182, 146, 109, 73, 36, 0];

fn main() {
    let img = image::open("IMG_1167.JPG").unwrap();
    let img = grayscale(&img);

    let (width, height) = img.dimensions();

    // target image will be the original image (img) rescaled, where each pixel in target_img will
    // be the average of the corresponding area in img.
    let mut target_img =
        ImageBuffer::from_fn(TARGET_IMG_WIDTH_PIXELS, TARGET_IMG_HEIGHT_PIXELS, |x, y| {
            image::Luma([get_aggregate_pixel_at(x, y, width, height, &img)])
        });

    // for now we will use a linear scale for each character in the char set and see how it looks.

    let mut output = Vec::<String>::new();
    for row_index in 0..target_img.height() {
        let mut s = String::new();
        for col_index in 0..target_img.width() {
            let pixel = target_img.get_pixel(col_index, row_index);
            let val: u8 = pixel.0[0];
            let (char, pixel_value) = find_closest_value_in_arr(val as i32);
            let pixel_value = pixel_value as u8;
            let error = val as i32 - pixel_value as i32;

            *target_img.get_pixel_mut(col_index, row_index) = image::Luma([pixel_value]);

            // propagate the error to neighboring pixels using the Floyd–Steinberg dithering coefficients
            // https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
            if let Some(p) =
                target_img.get_pixel_mut_checked((col_index + 1) as u32, row_index as u32)
            {
                let mut value = p.0[0];
                value = value.saturating_add_signed((error * 7 / 16) as i8);

                *p = image::Luma([value]);
            }
            if let Some(p) = target_img
                .get_pixel_mut_checked(col_index.saturating_sub(1), (row_index + 1) as u32)
            {
                let mut value = p.0[0];
                value = value.saturating_add_signed((error * 3 / 16) as i8);

                *p = image::Luma([value]);
            }
            if let Some(p) =
                target_img.get_pixel_mut_checked((col_index) as u32, (row_index + 1) as u32)
            {
                let mut value = p.0[0];
                value = value.saturating_add_signed((error * 5 / 16) as i8);

                *p = image::Luma([value]);
            }
            if let Some(p) =
                target_img.get_pixel_mut_checked((col_index) as u32, (row_index + 1) as u32)
            {
                let mut value = p.0[0];
                value = value.saturating_add_signed((error * 1 / 16) as i8);

                *p = image::Luma([value]);
            }

            s += char.to_string().as_str();
        }
        println!("{}", s);
        output.push(s);
    }

    target_img.save("test.png").unwrap();

    // print some run-length-encoded output for easier typing
    for string in output {
        let mut curr_char = string.chars().next().unwrap();
        let mut count: u32 = 1;
        for char in string.chars().skip(1) {
            if char == curr_char {
                count += 1;
            } else {
                if count > 2 {
                    print!("({} {}) ", curr_char, count);
                } else {
                    for _ in 0..count {
                        print!("{}", curr_char);
                    }
                }

                count = 1;
                curr_char = char;
            }
        }
        print!("({} {}) ", curr_char, count);

        println!("");
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

fn find_closest_value_in_arr(value: i32) -> (char, i32) {
    let arr = CHAR_BRIGHTNESSES;
    let char_arr = CHAR_SET;

    let mut closest: i32 = arr[0];
    let mut distance: i32 = (value - arr[0]).abs();
    let mut closest_char = char_arr[0];

    for (num, char) in arr.iter().zip(char_arr.iter()).skip(1) {
        if (value - num).abs() < distance {
            distance = (value - num).abs();
            closest = *num;
            closest_char = *char;
        }
    }
    return (closest_char, closest);
}
