use clap::Parser;
use converters::{colour_space_to_color_primaries, colour_transfer_to_transfer_char};
use progress_bar::{
    finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color,
    Style,
};
use ssimulacra2::{compute_frame_ssimulacra2, ColorPrimaries, TransferCharacteristic, Xyb};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use yuvxyb::Rgb;

use structs::{Args, FrameResult};

mod converters;
mod enums;
mod structs;

// TODO: Add proper error handling

fn main() {
    let args = Args::parse();

    let threads = if args.threads.is_some() {
        args.threads.unwrap()
    } else {
        num_cpus::get()
    };

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // convert args.colour_space to ColorPrimaries
            let colour_space = colour_space_to_color_primaries(&args.colour_space);

            // convert args.colour_transfer to TransferCharacteristic
            let colour_transfer = colour_transfer_to_transfer_char(&args.colour_transfer);

            // If not a folder process input as a single image
            if args.folders {
                // args get's moved into handle_folder, so we need to clone `out`
                let out_clone = args.out.clone();

                let mut results = handle_folder(args, colour_transfer, colour_space).await;

                // Sort by frame number
                results.sort_by(|a, b| a.frame.cmp(&b.frame));

                // Print Mean, min, max
                println!(
                    "Min: {}",
                    results
                        .iter()
                        .map(|r| r.ssimulacra2)
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                );
                println!(
                    "Max: {}",
                    results
                        .iter()
                        .map(|r| r.ssimulacra2)
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                );
                println!(
                    "Mean: {}",
                    results.iter().map(|r| r.ssimulacra2).sum::<f64>() / results.len() as f64
                );

                // Print CSV
                if let Some(out) = out_clone {
                    let mut csv = String::new();
                    csv.push_str("frame,ssimulacra2\n");
                    for result in results {
                        csv.push_str(&format!("{},{}\n", result.frame, result.ssimulacra2));
                    }
                    // check if `out` is a directory
                    if Path::new(&out).is_dir() {
                        let mut path = Path::new(&out).to_path_buf();
                        path.push("ssimulacra2.csv");
                        fs::write(path, csv).expect("Unable to write file");
                    } else {
                        fs::write(out, csv).expect("Unable to write file");
                    }
                }
            } else {
                let result = process(args.source, args.distorted, colour_transfer, colour_space);
                println!("{result:.8}");
            }
        });
}

/// Processes a single image pair
fn process(
    source_path: String,
    distorted_path: String,
    tc: TransferCharacteristic,
    cp: ColorPrimaries,
) -> f64 {
    // For now just assumes the input is sRGB. Trying to keep this as simple as possible for now.
    let source = image::open(source_path).expect("Failed to open source file");
    let distorted = image::open(distorted_path).expect("Failed to open distorted file");

    let source_data = source
        .to_rgb32f()
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect::<Vec<_>>();

    let source_data = Xyb::try_from(
        Rgb::new(
            source_data,
            source.width() as usize,
            source.height() as usize,
            tc,
            cp,
        )
        .expect("Failed to process source_data into RGB"),
    )
    .expect("Failed to process source_data into XYB");

    let distorted_data = distorted
        .to_rgb32f()
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect::<Vec<_>>();

    let distorted_data = Xyb::try_from(
        Rgb::new(
            distorted_data,
            distorted.width() as usize,
            distorted.height() as usize,
            tc,
            cp,
        )
        .expect("Failed to process distorted_data into RGB"),
    )
    .expect("Failed to process distorted_data into XYB");

    // Compute and return the SSIMulacra2 value
    compute_frame_ssimulacra2(source_data, distorted_data).expect("Failed to calculate ssimulacra2")
}

async fn handle_folder(
    args: Args,
    tc: TransferCharacteristic,
    cp: ColorPrimaries,
) -> Vec<FrameResult> {
    let files = fs::read_dir(args.source.clone()).unwrap();

    let results: Arc<Mutex<Vec<FrameResult>>> = Arc::new(Mutex::new(Vec::new()));

    // let mut count = 0;

    // TODO: This is a bit ugly, but it works. Reopen the folder and iterate over it again because count consumes the iterator.
    let len = fs::read_dir(args.source.clone()).unwrap().count();

    println!("Processing {len} files");

    let mut handles = vec![];

    init_progress_bar(len);
    set_progress_bar_action("Processing", Color::Blue, Style::Bold);

    // TODO: Figure out how to multithread this?
    for (count, path) in files.enumerate() {
        // count += 1;

        let arg_source = args.source.clone();
        let arg_distorted = args.distorted.clone();

        let results_clone = Arc::clone(&results);

        handles.push(tokio::spawn(async move {
            let src_path = Path::new(&arg_source);
            let dst_path = Path::new(&arg_distorted);

            let file_name = path.unwrap().file_name();

            let ssimulacra2_result = process(
                src_path
                    .join(file_name.clone())
                    .to_str()
                    .unwrap()
                    .to_owned(),
                dst_path.join(file_name).to_str().unwrap().to_owned(),
                tc,
                cp,
            );

            results_clone.lock().unwrap().push(FrameResult {
                frame: count.try_into().unwrap(),
                ssimulacra2: ssimulacra2_result,
            });

            // println!("Frame {}/{} complete!", count, len);
            inc_progress_bar();
        }));
    }

    futures::future::join_all(handles).await;

    finalize_progress_bar();

    let x = results.lock().unwrap().to_vec();
    x
}

#[cfg(test)]
mod tests {
    use crate::enums::{ColourSpace, ColourTransferCharacteristic};

    use super::*;

    #[test]
    fn process_test() {
        let res = process(
            "./test_images/source.png".to_string(),
            "./test_images/mozjpeg_100.png".to_string(),
            TransferCharacteristic::SRGB,
            ColorPrimaries::BT709,
        );

        // 91.91524120240736 was the old known value, no idea why it is a clean 100.0 now
        // 100.0 is the known result for source.png and mozjpeg_100.png
        assert_eq!(100.0, res);
    }

    #[test]
    fn convert_colour_space() {
        let res = colour_space_to_color_primaries(&ColourSpace::BT470M);

        assert_eq!(ColorPrimaries::BT470M, res);
    }

    #[test]
    fn convert_colour_transfer() {
        let res = colour_transfer_to_transfer_char(&ColourTransferCharacteristic::BT1886);

        assert_eq!(TransferCharacteristic::BT1886, res);
    }
}
