use subprocess::*;

fn main() {
    let commands = [
        "ffmpeg", 
        "-y", 
        "-f", "rawvideo", 
        "-vcodec", "rawvideo", 
        "-s", "1920x1080", 
        "-pix_fmt", "rgb24", 
        "-r", "60", 
        "-i", "-", 
        "-vcodec", "libx264", 
        "output.mp4"];

    let mut process = Popen::create(&commands, PopenConfig {
        stdin: Redirection::Pipe, 
        // stderr: Redirection::Pipe,
        ..Default::default()
    }).unwrap();

    println!("{:?}", process);

    let mut frames = Vec::new();

    for i in 0..1000 {
        let mut test_frame = vec![(i % 255) as u8; 1920 * 1080 * 3];

        frames.append(&mut test_frame);
    }
    

    process.communicate_bytes(Some(&frames)).unwrap();



    // for i in 0..100 {
        
    // };

    // process.terminate().unwrap();



}
