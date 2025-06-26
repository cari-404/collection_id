//#![windows_subsystem = "windows"]
use native_windows_gui as nwg;
use native_windows_derive::NwgUi;
use native_windows_gui::NativeUi;
use std::fs;

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control(
		size: (500, 200),
		position: (300, 300),
		title: "Launcher for collectionids",
		icon: Some(&Icon::from_bin(include_bytes!("launcher.ico")).unwrap()),
	)]
    #[nwg_events( OnWindowClose: [App::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Start")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    start_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, col: 1, row: 0, col_span: 2)]
    start_text: nwg::TextInput,

    #[nwg_control(text: "End")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    end_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, col: 1, row: 1, col_span: 2)]
    end_text: nwg::TextInput,

    #[nwg_control(text: "Voucher Code")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2)]
    code_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, col: 1, row: 2, col_span: 2)]
    code_text: nwg::TextInput,

    #[nwg_control(text: "Pilih file")]
    #[nwg_layout_item(layout: grid, col: 3, row: 0)]
    file_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, col: 4, row: 0, col_span: 2)]
    file_combo: nwg::ComboBox<String>,

     #[nwg_control(text: "Parameter")]
    #[nwg_layout_item(layout: grid, col: 3, row: 1)]
    param_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: grid, col: 4, row: 1, col_span: 2)]
    param_text: nwg::TextInput,

    #[nwg_control(text: "Jalankan")]
    #[nwg_layout_item(layout: grid, col: 4, row: 3)]
    #[nwg_events( OnButtonClick: [App::run] )]
    run_button: nwg::Button,

    #[nwg_resource(family: "Segoe UI", size: 8)]
    font_awesome: nwg::Font,
}

impl App {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

	fn run(&self) {
		let start = self.start_text.text();
		let end = self.end_text.text();
		let code = self.code_text.text();
		let file = self.file_combo.selection_string().unwrap_or_default();
        let param = self.param_text.text();

		// Menjalankan program main.exe dengan argumen yang dibuat
		let command = vec![
			"start",
			"main.exe",
			"--start", &start,
			"--end", &end,
			"--v-code", &code,
			"--file", &file,
            "--param", &param,
		];
		println!("{:?}", command.clone());
		// Menjalankan perintah dengan cmd
		let _status = std::process::Command::new("cmd")
			.arg("/c")
			.args(command)
			.spawn()
			.expect("Gagal menjalankan program");
		if file.is_empty() {
			let p = nwg::MessageParams {
			title: "Error",
			content: "Please select a file before running the program",
			buttons: nwg::MessageButtons::Ok,
			icons: nwg::MessageIcons::Info
			};
			assert!(nwg::modal_message(&self.file_combo, &p) == nwg::MessageChoice::Ok);
			return;
		}
	}
	
	fn populate_file_combo(&self) {
        // Specify the folder path where the files are located
        let folder_path = "akun";
		self.file_combo.size();

        // Read the files in the specified folder
        if let Ok(entries) = fs::read_dir(folder_path) {
            println!("Reading folder akun");
            println!("Available file");
			for entry in entries {
                if let Ok(entry) = entry {
                    // Get the file name
                    if let Some(file_name) = entry.file_name().to_str() {
                        // Add the file name to the ComboBox
                        println!("{}", file_name.to_string());
						self.file_combo.push(file_name.to_string());
                    }
                    self.file_combo.set_selection(Some(0));
                }
            }
        } else {
            let p = nwg::MessageParams {
				title: "Folder not found",
				content: "Folder akun tidak ada.\n \nHarap buat folder bernama akun",
				buttons: nwg::MessageButtons::Ok,
				icons: nwg::MessageIcons::Warning
			};
			assert!(nwg::modal_message(&self.file_combo, &p) == nwg::MessageChoice::Ok);
			println!("Failed to read the folder contents");
        }
    }
    fn populate_defaults(&self) {
        self.param_text.set_text("128");
    }
}

fn main() {
    nwg::init().expect("Gagal menginisialisasi native windows gui");
	nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let app = App::build_ui(Default::default()).expect("Gagal membangun antarmuka");
	app.populate_file_combo();
    app.populate_defaults();
    nwg::dispatch_thread_events();
}
