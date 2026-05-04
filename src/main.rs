use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

use crate::sort::read_file;

mod sort;
#[derive(Clone)]
pub struct SortOptions {
      img: bool,
    video: bool,
    music: bool,
    exe: bool,
    torr: bool,
    zip: bool,
    doc: bool,
    code_mode: bool,
    down: bool,


     cpp: bool,
     rust: bool,
     python: bool,
     js: bool,
       asm: bool,
 go: bool,
c: bool,
 sh: bool,
 make: bool,
 undo: bool,
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "_S app",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    current_dir: Option<PathBuf>,
    exe: bool,
    img: bool,
    doc: bool,
    video: bool,
    music: bool,
    torr: bool,
    zip: bool,
    down: bool,
    code_mode: bool,
    cpp: bool,
    undo: bool,
rust: bool,
python: bool,
js: bool,
c: bool,
asm: bool,
go: bool,
sh: bool,
make: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
          ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {


ui.checkbox(&mut self.img, "img");
ui.checkbox(&mut self.video, "video");
ui.checkbox(&mut self.music, "music");
ui.checkbox(&mut self.exe, "EXE");
ui.checkbox(&mut self.torr, "Torrent");
ui.checkbox(&mut self.zip, "ZIP");
ui.checkbox(&mut self.doc, "documents");
ui.checkbox(&mut self.down, "dowmloads");
ui.checkbox(&mut self.code_mode, "code mode");
ui.checkbox(&mut self.undo, "undo");
if self.undo{
    self.img = false;
        self.video = false;
        self.music = false;
        self.exe = false;
        self.torr = false;
        self.zip = false;
        self.doc = false;
        self.down = false; 
        self.code_mode = false;
}
if self.code_mode{
    ui.separator();
    ui.label("Languages:");

      ui.checkbox(&mut self.cpp, "C++ (.cpp, .h)");
    ui.checkbox(&mut self.rust, "Rust (.rs)");
    ui.checkbox(&mut self.python, "Python (.py)");
    ui.checkbox(&mut self.js, "JavaScript (.js)");
  ui.checkbox(&mut self.cpp, "C(.c, .h)");
    ui.checkbox(&mut self.asm, "ASM (.asm, .s)");
    ui.checkbox(&mut self.go, "Go (.go)");
    ui.checkbox(&mut self.sh, "Shell (.sh)");
    ui.checkbox(&mut self.make, "Makefile");

 self.img = false;
        self.video = false;
        self.music = false;
        self.exe = false;
        self.torr = false;
        self.zip = false;
        self.doc = false;
        self.down = false;
}

if ui.button("📁 Select Folder").clicked() {
    let ctx_clone = ctx.clone();
 let sort_options = SortOptions {
    img: self.img,
    video: self.video,
    music: self.music,
    exe: self.exe,
    torr: self.torr,
    zip: self.zip,
    doc: self.doc,
    down: self.down,
    code_mode: self.code_mode,
    cpp: self.cpp,
    rust: self.rust,
    python: self.python,
    js: self.js,
    c: self.c,
    asm: self.asm,
    go: self.go,
    sh:self.sh,
    make:self.make,
    undo: self.undo
};
    std::thread::spawn(move || {
        if let Some(path) = FileDialog::new().pick_folder() {
            println!("Folder Choose: {:?}", path);
        read_file(&path,&sort_options);

        }
        ctx_clone.request_repaint();
    });
}
        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_dir: None,
            img: true,
            video: true,
            music: true,
            exe: true,
            torr: true,
            zip: true,
            doc: true,
            down: true,
            code_mode: false,
            cpp: true,
            python: true,
            rust:true,
            js:true,
            make: false,
            sh: false,
            c: true,
            asm: true,
            go: true,
            undo: false,
        }
    }
}