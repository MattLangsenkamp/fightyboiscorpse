extern crate image;

mod imagemangler {
    pub fn dice_image(filename: &String, spots:i32) -> Vec<String> {
        /*
        let img = image::open(filename).unwrap();
        let dims = img.dimensions()
        let full_pixel_col = dims.1 as i32;
        let full_pixel_row = dims.0 as i32;
        let dims = generate_dimensions(spots, pixel_row, pixel_col);
        for r in 1..(dims.1+1) {
             for c in 1..(dims.2+1) {
                let pixel_col = dim.0 + ((full_pixel_col - c*dim.0) < dim.0*2) * (full_pixel_col - c*dim.0));
                let pixel_row = dim.0 + ((full_pixel_row - c*dim.0) < dim.0*2) * (full_pixel_row - c*dim.0));
                create_image_for_spot(let pixel_row,
                }
         }
        */
    }

    fn generate_dimensions(spots: f64, pixel_row: f64, pixel_col:f64) -> (f64, f64, f64) {
        let c = (pixel_col*spots/pixel_row).sqrt();
        let r = (pixel_row*spots/pixel_col).sqrt();
        let l = pixel_col/c;
        return (r, c, l)
    }

    fn create_image_for_spot(row:i32, col:i32, pixel_row:i32, pixel_col:i32, img:DynamicImage) {

    }

}