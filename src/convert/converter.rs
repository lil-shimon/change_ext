use image::GenericImageView;

pub fn convert_jpg_to_png() {

    /// read image
    let image = image::open("src/convert/test.jpg").unwrap();

    /// image size
    let size = image.dimensions();
    println!("image size >>> {:?}", size);

    /// image color type
    let color_type = image.color();
    println!("image color type >>> {:?}", color_type);

    /// save image as png
    image.save("src/convert/test.png").unwrap();
}