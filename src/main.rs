//Require our libraries
//We need to be able to read the args that are passed
use {std::env::{args,current_dir}};
//We need to get the size of the console.
use term_size::{dimensions};
//We also need the image crate
use::image::{DynamicImage,GenericImageView};

static ASCII:[char;10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn main() {
    let image_name = args().enumerate().last().unwrap().1;
    let mut current_dir = current_dir().unwrap().into_os_string().into_string().unwrap()+"/src/"+&image_name;
   

    //Does the image exist?
    image::open(&current_dir).expect("The file did not exist. Please make sure to place the image in the /src directory.");

    //Lets open the image using the image crate
    let mut image = image::open(&current_dir).unwrap();

    //Console size - tuple
    let (width,height) = dimensions().unwrap();

    //Scale the image to our consoles size
    scale_image(&mut image,width,height);

    let mut image_to_ascii:String = String::new();
    //Get image current dimensions after scaling
    let (image_width,image_height) = image.dimensions();

    //Build our image
    for y in 0..image_height {
        for x in 0..image_width {
            let pixel = image.get_pixel(x, y);
            let luminance = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as usize;
            let chars_to_push = &ASCII[luminance * (&ASCII.len() - 1) / 255];
            image_to_ascii.push(*chars_to_push);
        }
        image_to_ascii.push('\n');
    }

    //print out the image
    println!("{}",image_to_ascii);

}


//Define the scale image function to take a mutable reference to our image. Later in the function, we will derefence the image during the assignment of the value
//Also takes the consoles width and consoles height
fn scale_image(image: &mut DynamicImage, console_width: usize, console_height: usize) -> () {
    //Get the images sizes
    let (image_width,image_height) = image.dimensions();
    let image_ratio = image_height as f32 / image_width as f32;
    
    let new_image_width:f32 = if image_width as f32 > console_width as f32 * image_ratio{ console_width as f32}else{image_width as f32};
    let new_image_height:f32 = if image_height as f32 > console_height as f32 * image_ratio{ console_height as f32}else{image_height as f32};

    *image = image.resize(new_image_width as u32 * 5, new_image_height as u32, image::imageops::FilterType::Nearest);

    ()
}