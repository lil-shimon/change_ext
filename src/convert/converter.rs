use image::GenericImageView;

pub fn convert_jpg_to_png() {
    /// image url
    let test_img_url = "src/convert/test.jpg";

    /// read image
    let image = image::open(test_img_url).unwrap();

    /// image size
    let size = image.dimensions();
    println!("{:?}", size)
}