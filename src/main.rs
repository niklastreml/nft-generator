use image::{imageops::overlay, GenericImage, Pixel, Primitive};
use rand::Rng;
use std::{
    env, fs,
    path::PathBuf,
    sync::{Arc, Barrier},
    thread,
}; // 0.8.0
use threadpool::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = args[1].parse().unwrap();

    // let pool = ThreadPool::new(8);

    // let barrier = Arc::new(Barrier::new(num as usize + 1));

    for i in 0..num {
        let src = args[2].clone();
        // let barrier = barrier.clone();

        // pool.execute(move || {
        let mut images = Vec::new();
        let paths = get_random_files(src);
        println!("Working on #{:?}", i + 1);

        for p in paths {
            let img = image::open(p).unwrap();
            let img = img.to_rgba16();
            images.push(img);
        }

        combine(&mut images)
            .save(format!("results/out-{}.png", i + 1))
            .unwrap();

        // barrier.wait();
        // });
    }

    // barrier.wait();
}

fn combine<I, P, S>(images: &mut Vec<I>) -> I
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    images.reverse();
    let mut first = images.pop().unwrap();
    images.reverse();

    for i in images {
        overlay(&mut first, i, 0, 0);
    }

    first
}

fn get_random_files(src: String) -> Vec<PathBuf> {
    let paths = fs::read_dir(src).unwrap();
    let mut dirs = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            dirs.push(path);
        }
    }

    let mut all_items = Vec::new();

    for path in dirs {
        // every subdirectory
        let paths = fs::read_dir(path.display().to_string()).unwrap();
        let mut items = Vec::new();

        for path in paths {
            // println!("{}", path.unwrap().path().display());
            let path = path.unwrap().path();

            if path.is_file() {
                items.push(path);
            }
        }

        let rand_idx = rand::thread_rng().gen_range(0..items.len());
        all_items.push(items[rand_idx].clone());
    }

    all_items
}
