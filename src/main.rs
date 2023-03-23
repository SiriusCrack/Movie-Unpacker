use std::{
    env,
    io::stdin,
    path::PathBuf,
    fs::{DirEntry, ReadDir, read_dir, rename}, 
};

fn main() {
    let root_dir = get_root_dir();
    for entry in root_dir {
        let entry_is_dir = entry.as_ref().unwrap().metadata().unwrap().file_type()
            .is_dir();
        let entry_path = entry.as_ref().unwrap()
            .path();
        match entry_is_dir {
            true => unpack_movie_dir(entry_path),
            false => continue,
        }
    }
}

fn get_root_dir() -> ReadDir{
    let args: Vec<String> = env::args().collect();
    let path = match args.len() {
        1 => env::current_dir().unwrap(),
        _ => PathBuf::from(&args[1]),
    };
    read_dir(path).unwrap()
}

fn unpack_movie_dir(dir_path: PathBuf) {
    let dir = read_dir(&dir_path).unwrap();
    let result = parse_movie_dir(dir); 
    let subtitle_dir = result.0;
    let movie_file = result.1;
    let movie_title = prompt_movie_title(&movie_file);
    rename_movie(&dir_path, &movie_title, &movie_file);
    if subtitle_dir.is_some() { parse_subtitle_dir(&subtitle_dir.unwrap(), &movie_title); }
}

fn parse_movie_dir(dir: ReadDir) -> (Option<DirEntry>, DirEntry) {
    let mut subtitle_dir = None;
    let mut movie_file = None;
    for entry in dir {
        let entry = entry.unwrap();
        let entry_filename = entry
            .file_name().to_str().unwrap().to_owned();
        let entry_metadata = entry
            .metadata().unwrap();
        if entry_metadata.is_dir() && entry_filename.contains("ub") { subtitle_dir = Some(entry); }
        else { match movie_file {
            None => movie_file = Some(entry),
            Some(ref x) => {
                let prev_file_size = x.metadata().unwrap()
                    .len();
                if prev_file_size < entry_metadata.len() { movie_file = Some(entry); }
            }
        } }
    }
    let movie_file = movie_file.unwrap();
    (
        subtitle_dir, 
        movie_file,
    )
}

fn prompt_movie_title(movie_file: &DirEntry) -> String {
    println!("{:?}", movie_file.file_name());
    let mut input_buffer = String::new();
    stdin().read_line(&mut input_buffer).unwrap();
    input_buffer.trim().to_owned()
}

fn rename_movie(dir_path: &PathBuf, movie_title: &String, movie_file: &DirEntry) {
    let movie_file_extension = movie_file.path()
        .extension().unwrap()
        .to_str().unwrap().to_owned();
    let movie_file_name = format!(
        "{}.{}", 
        movie_title, 
        movie_file_extension
    );
    let from = movie_file.path();
    let to = dir_path
        .parent().unwrap().to_owned()
        .join(movie_file_name);
    rename(&from, &to).unwrap();
}

fn parse_subtitle_dir(subtitle_dir: &DirEntry, movie_title: &String) {
    let subtitle_dir_path = subtitle_dir.path();
    let subtitle_dir = read_dir(&subtitle_dir_path).unwrap();
    let mut eng_subtitle_files = Vec::<DirEntry>::new();
    for entry in subtitle_dir {
        let entry = entry.unwrap();
        let entry_filename = entry
            .file_name().to_str().unwrap().to_owned();
        if entry_filename.contains("nglish") { eng_subtitle_files.push(entry); }
    }
    if eng_subtitle_files.len() == 1 {
        let subtitle_file_extension = eng_subtitle_files[0].path()
            .extension().unwrap().to_str().unwrap().to_owned();
        let subtitle_file_name = format!(
            "{}.en.{}", 
            movie_title, 
            subtitle_file_extension
        );
        let from = eng_subtitle_files[0].path();
        let to = subtitle_dir_path
            .parent().unwrap()
            .parent().unwrap().to_owned()
            .join(subtitle_file_name);
        rename(from, to).unwrap();
    } else {
        println!("Subtitle layout too complex, {:?}", subtitle_dir_path);
        for (i, entry) in eng_subtitle_files.iter().enumerate() {
            let subtitle_file_extension = entry.path()
                .extension().unwrap().to_str().unwrap().to_owned();
            let subtile_file_name = format!(
                "{}{}.{}", 
                movie_title, 
                i.to_string(), 
                subtitle_file_extension
            );
            let from = entry.path();
            let to = subtitle_dir_path
                .parent().unwrap()
                .parent().unwrap().to_owned()
                .join(subtile_file_name);
            rename(from, to).unwrap();
        }
    }
}