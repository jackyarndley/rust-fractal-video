use subprocess::*;
use rayon::prelude::*;

use std::fs;
use std::time::Instant;

fn main() {
    // Get the names of all of the frames
    let paths = fs::read_dir("../rust-fractal/output").unwrap();

    let mut png_files = Vec::new();

    for path in paths {
        let name = path.unwrap()
            .path()
            .to_owned()
            .to_str()
            .unwrap()
            .to_string();
        
        if name.contains(".png") {
            png_files.push(name)
        }
    };

    // Possibly need to sort the files just to make sure (they should be correct)
    // println!("{:?}", png_files);
    let zoom_scale = 1.4;
    let maximum_keyframe_number = 1;
    let frames_between_keyframes = 60;

    // Open the first file as the previous keyframe
    let temp1 = image::open(&png_files[0]).unwrap();
    let mut previous_keyframe = temp1.into_rgb();

    // Get width and height, get new width and height
    let width = previous_keyframe.width();
    let height = previous_keyframe.height();

    let scaled_width = (zoom_scale * width as f64) as u32;
    let scaled_height = (zoom_scale * height as f64) as u32;

    // Calculate the new values for the placement box (will be same each time)

    let box_x = (scaled_width - width) / 2;
    let box_y = (scaled_height - height) / 2;

    // Go through each keyframe
    for i in 0..maximum_keyframe_number {
        // get the next keyframe
        let temp2 = image::open(&png_files[i + 1]).unwrap();
        let mut next_keyframe = temp2.into_rgb();

        // scale to correct size
        next_keyframe = image::imageops::resize(&next_keyframe, 
            scaled_width, 
            scaled_height, 
            image::imageops::FilterType::Nearest);

        // paste in the old keyframe
        image::imageops::overlay(&mut next_keyframe, &previous_keyframe, box_x, box_y);

        // get scaling factor thing
        // next_keyframe.save("test.png").unwrap();

        let mut scaling_factors = Vec::new();
        let scaling_coefficient = 10.0f64.powf(zoom_scale.log10() / frames_between_keyframes as f64);
        let mut current_scale = zoom_scale;

        for _ in 0..frames_between_keyframes {
            current_scale /= scaling_coefficient;
            scaling_factors.push(current_scale);
        }

        scaling_factors.reverse();

        let time = Instant::now();

        let buffer = scaling_factors.into_iter()
            .flat_map(|factor| {
                let temp1 = ((1.0 - 1.0 / factor) * (scaled_width as f64 / 2.0)) as u32;
                let temp2 = ((1.0 - 1.0 / factor) * (scaled_height as f64 / 2.0)) as u32;

                let temp3 = scaled_width - 2 * temp1;
                let temp4 = scaled_height - 2 * temp2;

                let temp_image = image::imageops::crop_imm(&next_keyframe, temp1, temp2, temp3, temp4);

                image::imageops::resize(
                    &temp_image, 
                    width, 
                    height, 
                    image::imageops::FilterType::Nearest);

                vec![0u8; 1]
            }).collect::<Vec<u8>>();

        previous_keyframe = image::imageops::resize(&next_keyframe, 
            width, 
            height, 
            image::imageops::FilterType::Lanczos3);

        println!("Rescaling {}: {}ms", i, time.elapsed().as_millis());

        // let time = Instant::now();

        // let commands = [
        //     "ffmpeg", 
        //     "-y", // Force overwrite
        //     "-f", "rawvideo", // Input file format
        //     "-vcodec", "rawvideo", // Input file codec
        //     "-s", "1920x1080", // Input file size
        //     "-pix_fmt", "rgb24", // Input pixel format
        //     "-r", "60", // Input frame rate
        //     "-i", "-", // Input is pipe
        //     "-vcodec", "libx264", // Output video codec
        //     &format!("output_{}.mp4", i)]; // Output video name


        // let mut process = Popen::create(&commands, PopenConfig {
        //     stdin: Redirection::Pipe, 
        //     stderr: Redirection::Pipe,
        //     ..Default::default()
        // }).unwrap();

        // process.communicate_bytes(Some(&buffer)).unwrap();

        // println!("Encoding {}: {}ms", i, time.elapsed().as_millis());


        // flatten and collect

        // get ffmpeg to make video
    }





}
