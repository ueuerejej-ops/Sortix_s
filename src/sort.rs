use std::{fs, option};
use std::path::PathBuf;


use crate::SortOptions;

#[derive(Debug)]
struct MyFile {
    path: PathBuf,
    name: String,
    ext: String,
}

struct Folders {
    images: PathBuf,
    videos: PathBuf,
    music: PathBuf,
    exe: PathBuf,
    torrent: PathBuf,
    archives: PathBuf,
    documents: PathBuf,
    zip: PathBuf,
    apps: PathBuf,
    other_files: PathBuf,
}

fn files_name(folder: &String)->Vec<String>{
    let mut vec_name = Vec::new();

    for entry in fs::read_dir(folder).unwrap(){
        let entry = entry.unwrap();
        if let Some(name) = entry.path().file_name(){
            vec_name.push(name.to_string_lossy().to_string());
        }
    }
    vec_name
}


fn files_path(folder: &String)->Vec<String>{
    let mut vec_path = Vec::new();

    for entry in fs::read_dir(folder).unwrap(){
        let entry = entry.unwrap();
        vec_path.push(entry.path().to_string_lossy().to_string());
    }
    vec_path
}

fn collect_files(folder: &PathBuf)->Vec<MyFile>{
    let mut files = Vec::new();
    for entry in std::fs::read_dir(folder).unwrap(){
        let enrty = entry.unwrap();
        let path = enrty.path();

       let name = match path.file_name() {
    Some(n) => n.to_string_lossy().to_string(),
    None => continue,
};

        let ext = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();


         files.push(MyFile {

            path,
            name,
            ext,
        });
    }
    files
}

fn collect_files_search(folder: &PathBuf,find:&String)->Vec<MyFile>{
let mut vec_img:Vec<MyFile> = Vec::new();
let vec_files = collect_files(folder);
for file in vec_files{
    if file.ext == find.to_string(){
        vec_img.push(file);
}

}
vec_img
}


fn create_all_files_needed(folder: &str) -> Folders {
    let base = std::path::Path::new(folder);

    let folders = Folders {
        images: base.join("images_s"),
        videos: base.join("videos_s"),
        music: base.join("music_s"),
        exe: base.join("exe_s"),
        torrent: base.join("torrent_s"),
        archives: base.join("archives_s"),
        zip: base.join("zip_s"),
        other_files: base.join("other_files_s"),
        documents: base.join("documents_s"),
        apps: base.join("app_s")
    };

    std::fs::create_dir_all(&folders.images).unwrap();
    std::fs::create_dir_all(&folders.videos).unwrap();
    std::fs::create_dir_all(&folders.music).unwrap();
    std::fs::create_dir_all(&folders.exe).unwrap();
    std::fs::create_dir_all(&folders.torrent).unwrap();
    std::fs::create_dir_all(&folders.archives).unwrap();
    std::fs::create_dir_all(&folders.documents).unwrap();
  std::fs::create_dir_all(&folders.zip).unwrap();
     std::fs::create_dir_all(&folders.other_files).unwrap();
         std::fs::create_dir_all(&folders.apps).unwrap();

    folders
}


 fn undo_sort(base: &PathBuf) {
    let folders = [
        "cpp_s",
        "c_s",
        "rs_s",
        "py_s",
        "js_s",
        "asm_s",
        "go_s",
        "sh_s",
        "make_s",
        "images_s",
        "videos_s",
        "music_s",
        "zip_s",
        "apps_s",
        "documents_s",
        "torrent_s",
        "exe_s",
        "headers_s",
    ];

for folder in folders{
    let dir = base.join(folder);
    if !dir.exists(){
        continue;
    }

    let entries = match fs::read_dir(&dir){
        Ok(e)=>e,
        Err(_)=>continue,
    };

    for entry in entries.flatten(){
        let file_path = entry.path();
        let file_name  = match file_path.file_name(){
            Some(n)=>n,
            None=>continue,
        };
        let new_path  = base.join(file_name);

        fs::rename(&file_path, &new_path);


        if let Err(e) = fs::rename(&file_path, &new_path) { eprintln!("undo error: {}", e); } else { println!("Restored: {:?}", file_name); }
    

     };
     if let Ok(mut check) = fs::read_dir(&dir) { if check.next().is_none() { let _ = fs::remove_dir(&dir); println!("Removed empty folder: {:?}", dir); } }
    }
}


pub fn read_file(path: &PathBuf, options_sort: &SortOptions) {
    if options_sort.undo {
        undo_sort(path);
        return;
    }

    let files = collect_files(path);

    for file in files {

        let target = if options_sort.code_mode {

            let ext = file.ext.to_lowercase();

            match ext.as_str() {
                "cpp" | "hpp" if options_sort.cpp => Some(path.join("cpp_s")),
                "h" if options_sort.cpp => Some(path.join("headers_s")),
                "c" if options_sort.c => Some(path.join("c_s")),
                "rs" if options_sort.rust => Some(path.join("rs_s")),
                "py" if options_sort.python => Some(path.join("py_s")),
                "js" if options_sort.js => Some(path.join("js_s")),
                "asm" | "s" if options_sort.asm => Some(path.join("asm_s")),
                "go" if options_sort.go => Some(path.join("go_s")),
                "sh" if options_sort.sh => Some(path.join("sh_s")),

                _ if options_sort.make && file.name.to_lowercase() == "makefile" => {
                    Some(path.join("make_s"))
                }

                _ => None,
            }

        } else {
            match file.ext.as_str() {
                "jpg" | "png" | "jpeg"  if options_sort.img => {
                    Some(path.join("images_s"))
                }
                "mp4" | "mkv" | "webm" if options_sort.video => {
                    Some(path.join("videos_s"))
                }
                "mp3" if options_sort.music => {
                    Some(path.join("music_s"))
                }
                "exe" if options_sort.exe => {
                    Some(path.join("exe_s"))
                }
                "torrent" if options_sort.torr => {
                    Some(path.join("torrent_s"))
                }
                "zip" if options_sort.zip => {
                    Some(path.join("zip_s"))
                }
                "pdf" | "txt" if options_sort.doc => {
                    Some(path.join("documents_s"))
                }
                "AppImage" | "deb" | "gz" | "xz" if options_sort.down => {
                    Some(path.join("apps_s"))
                }

                _ => None,
            }
        };

        if let Some(dir) = target {
            let _ = std::fs::create_dir_all(&dir);

            let new_path = dir.join(&file.name);

            if let Err(e) = std::fs::rename(&file.path, &new_path) {
                println!("error moving {}: {}", file.name, e);
            } else {
                println!("Moved: {} -> {:?}", file.name, dir);
            }
        }
    }
}