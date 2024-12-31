/*
This Is a first version of get_vouchers_by_collections
This version using api reqwest
Whats new In 1.3.0 :
- add base64 encode for evcode
Whats new In 1.2.9 :
Bugs Fix?
Whats new In 1.2.8 :
Bugs Fix?
*/

use rquest as reqwest;
use reqwest::tls::Impersonate;
use reqwest::{ClientBuilder, header::HeaderMap, Error, Response, Version};
use reqwest::header::HeaderValue;
use serde::Serialize;
use serde_json::{self, Value};
use anyhow::Result;
use std::fs::{self, OpenOptions, File};
use std::path::Path;
use std::io::{self, Write, prelude::*};
use std::process;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::{Local, Utc};
use structopt::StructOpt;
use base64::encode;
use urlencoding::encode as url_encode;
#[cfg(windows)]
use windows_version::*;

#[derive(Serialize)]
struct JsonRequest {
	voucher_collection_request_list: Vec<VoucherCollectionRequest>,
}

#[derive(Serialize)]
struct VoucherCollectionRequest {
	collection_id: String,
	component_type: i64,
	component_id: i64,
	limit: i64,
	microsite_id: i64,
	offset: i64,
	number_of_vouchers_per_row: i64,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "get_vouchers_by_collections", about = "get_vouchers_by_collections")]
struct Opt {
	#[structopt(short, long, help = "Start scan collection_id")]
	start: Option<String>,
	#[structopt(short, long, help = "End scan collection_id")]
	end: Option<String>,
	#[structopt(short, long, help = "Target Code")]
	v_code: Option<String>,
	#[structopt(short, long, help = "select file cookie")]
	file: Option<String>,
	#[structopt(short, long, help = "Set default cookie file")]
	cookie: bool,
}

fn extract_csrftoken(cookie_string: &str) -> String {
	let mut csrftoken = String::new();
	if let Some(token_index) = cookie_string.find("csrftoken=") {
		let token_start = token_index + "csrftoken=".len();
		if let Some(token_end) = cookie_string[token_start..].find(';') {
			csrftoken = cookie_string[token_start..token_start + token_end].to_string();
		}
	}
	csrftoken
}

async fn process_arguments(start: &str, end: &str, v_code: &str) -> Result<()> {
	// Read the content of "akun.conf"
	let selected_file = std::fs::read_to_string("./akun.conf")?;
	let file_path = format!("./akun/{}", selected_file.trim());
	let mut cookie_content = String::new();
	std::fs::File::open(&file_path)?.read_to_string(&mut cookie_content)?;

	// Process HTTP with common function
	some_function(start, end, v_code, &cookie_content, &selected_file).await?;

	Ok(())
}

async fn manual_input(opt: &Opt) -> Result<()> {
	let selected_file = opt.file.clone().unwrap_or_else(|| choose_cookie().expect("Folder akun dan file cookie tidak ada\n"));
	// Read the content of the selected cookie file
	let file_path = format!("./akun/{}", selected_file);
	let mut cookie_content = String::new();
	File::open(&file_path)?.read_to_string(&mut cookie_content)?;
	
	println!("Contoh input: Awal: 12905192072, Akhir: 12905192100");
	let start = opt.start.clone().unwrap_or_else(|| get_user_input("Masukkan nilai start: "));
	let end = opt.end.clone().unwrap_or_else(|| get_user_input("Masukkan nilai akhir: "));
	let v_code = opt.v_code.clone().unwrap_or_else(|| get_user_input("Masukkan voucher_code: "));

	// Process HTTP with common function
	match some_function(&start, &end, &v_code, &cookie_content, &selected_file).await {
	Ok(_) => {},
	Err(err) => eprintln!("Error: {}", err),
	}
	Ok(())
}

fn print_and_log(pb: &ProgressBar, mut log_file: &File, mes1: &str, color: &str, mes2: &str, logmes: &str) {
	let reset_color = "\x1b[0m";
	if cfg!(windows) {
		#[cfg(windows)]
		if OsVersion::current() <= OsVersion::new(6, 3, 0, 9800) {
			interactive_print(pb, &format!("{}{}", mes1, mes2));
		}else{
			interactive_print(pb, &format!("{}{}{}{}", mes1, color, mes2, reset_color));
		}
	}else{
		interactive_print(pb, &format!("{}{}{}{}", mes1, color, mes2, reset_color));
	}
	writeln!(log_file, "{}{}", logmes, mes2).expect("Gagal menulis ke file log");
}

fn interactive_print(pb: &ProgressBar, message: &str) {
	let is_interactive = atty::is(atty::Stream::Stdout);
	if cfg!(windows) {
		#[cfg(windows)]
		if OsVersion::current() <= OsVersion::new(6, 3, 0, 9800){
			println!("{}", format!("{}", message));
		}else{
			pb.println(format!("{}", message));
		}
	}else if is_interactive {
		pb.println(format!("{}", message));
	}else{
		println!("{}", format!("{}", message));
	}
}

async fn some_function(start: &str, end: &str, v_code: &str, cookie_content: &str, selected_file: &str) -> Result<()> {
	let formatted_datetime = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
	// Mengonversi nama akun menjadi format folder yang sesuai
	let header_folder = format!("./header/{}/af-ac-enc-sz-token.txt", selected_file);
	
	// Membuat folder header jika belum ada
	fs::create_dir_all(&format!("./header/{}", selected_file))?;

	// Membuat file header jika belum ada
	if !File::open(&header_folder).is_ok() {
		let mut header_file = File::create(&header_folder)?;
		// Isi file header dengan konten default atau kosong sesuai kebutuhan
		header_file.write_all(b"ganti kode ini dengan sz-token valid")?;
	}

	// Baca isi file untuk header af-ac-enc-sz-token
	let mut sz_token_content = String::new();
	File::open(&header_folder)?.read_to_string(&mut sz_token_content)?;
	println!("sz-token:{}", sz_token_content);
	let cookie_content_owned = cookie_content.to_string();

	// Pass the cloned String to extract_csrftoken
	let csrftoken = extract_csrftoken(&cookie_content_owned);
	println!("csrftoken: {}", csrftoken);
	let csrftoken_string = csrftoken.to_string();
	let mulai = fix_start (&start);
	let end: i64 = end.trim().parse().expect("Input tidak valid");

	// Jumlah nilai per batch
	let values_per_batch = 5;

	// Hitung jumlah batch
	let batch_count = ((end - mulai) / 128 / values_per_batch) + 1;

	// Iterasi dan menuliskan angka dengan jarak 128
	let mut batch_number = 1;
	let mut current = mulai;
	
	let pb = bar (&batch_count);

	for _ in 0..batch_count {
		interactive_print(&pb, &format!("[{}] Batch {} of {}", Local::now().format("%H:%M:%S.%3f"), batch_number, batch_count));

		for _ in 0..values_per_batch {
			// Bentuk struct VoucherCollectionRequest
			let voucher_request = VoucherCollectionRequest {
				collection_id: current.to_string(),
				component_type: 2,
				component_id: 1712077200,
				limit: 100,
				microsite_id: 63749,
				offset: 0,
				number_of_vouchers_per_row: 2,
			};
			
			let mut headers = reqwest::header::HeaderMap::new();
			headers.insert("User-Agent", HeaderValue::from_static("Android app Shopee appver=29335 app_type=1"));
			headers.insert("Connection", HeaderValue::from_static("keep-alive"));
			headers.insert("Accept", HeaderValue::from_static("application/json"));
			headers.insert("Accept-Encoding", HeaderValue::from_static("gzip"));
			headers.insert("Content-Type", HeaderValue::from_static("application/json"));
			headers.insert("x-api-source", HeaderValue::from_static("rn"));
			headers.insert("if-none-match-", HeaderValue::from_static("55b03-1e991df3597baecb4f87bfbe85b99329"));
			headers.insert("af-ac-enc-dat", HeaderValue::from_static(""));
			headers.insert("af-ac-enc-sz-token", HeaderValue::from_static(""));
			headers.insert("shopee_http_dns_mode", HeaderValue::from_static("1"));
			headers.insert("af-ac-enc-id", HeaderValue::from_static(""));
			headers.insert("x-sap-access-t", HeaderValue::from_static(""));
			headers.insert("x-sap-access-s", HeaderValue::from_static(""));
			headers.insert("x-sap-access-f", HeaderValue::from_static(""));
			headers.insert("x-shopee-client-timezone", HeaderValue::from_static("Asia/Jakarta"));
			headers.insert("referer", HeaderValue::from_static("https://mall.shopee.co.id/"));
			headers.insert("x-csrftoken", reqwest::header::HeaderValue::from_str(&csrftoken_string)?);
			headers.insert(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_content)?);

			// Bentuk struct JsonRequest
			let json_request = JsonRequest {
				voucher_collection_request_list: vec![voucher_request],
			};

			// Convert struct to JSON
			let json_body = serde_json::to_string(&json_request)?;
			
			loop {
				let response = make_http_request(&headers, json_body.clone()).await?;
				// Check for HTTP status code indicating an error
				//let http_version = response.version(); 		// disable output features
				//println!("HTTP Version: {:?}", http_version); // disable output features
				let logs_folder = "logs"; // Nama folder induk
				let sub_folder = "cid";   // Nama folder di dalam "logs"
				let file_name = format!("{}-{}_{}.log", mulai, end, formatted_datetime);

				// Pastikan folder "logs" ada
				if !Path::new(logs_folder).exists() {
					fs::create_dir(logs_folder).expect("Gagal membuat folder logs");
				}

				// Pastikan folder "cid" ada di dalam folder "logs"
				let sub_path = format!("{}/{}", logs_folder, sub_folder);
				if !Path::new(&sub_path).exists() {
					fs::create_dir(&sub_path).expect("Gagal membuat sub folder");
				}

				// Buat jalur lengkap untuk file
				let log_filename = format!("{}/{}", sub_path, file_name);
				// Pembukaan file dilakukan di luar loop
				let mut log_file = OpenOptions::new()
					.create(true)
					.append(true)
					.open(&log_filename)
					.expect("Gagal membuka file log");
				let status = response.status();
				let text = response.text().await?;	
				if status == reqwest::StatusCode::OK {
					let hasil: Value = serde_json::from_str(&text)?;
					let error_res = hasil.get("error").and_then(|er| er.as_i64()).unwrap_or(0);
					let error_res_str = error_res.to_string();
					let rcode = process_response(&pb, v_code, &log_file, &text).await;
					if rcode == "bug" {
						print_and_log(&pb, &mut log_file, &format!("API Checker 1"), "", "", &format!("API Checker 1"));
						let cid_1 = current.to_string();
						api_1(&pb, &cid_1, &headers.clone(), v_code, &log_file).await?;
					}else if rcode == "none" {
						print_and_log(&pb, &mut log_file, &format!("Tidak ada data ditemukan untuk collection_id: "), "", &current.to_string(), &format!("collection_id: "));
						print_and_log(&pb, &mut log_file, &format!("error : "), "", &error_res_str, &format!("error : "));
						print_and_log(&pb, &mut log_file, &format!("Body  : "), "", &text, &format!("Body  : "));
					}
					break;
				}else if status == reqwest::StatusCode::IM_A_TEAPOT {
					interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", current.to_string()));
					interactive_print(&pb, &format!("Gagal, status code: 418 - I'm a teapot. Mencoba kembali..."));
					interactive_print(&pb, &format!("{}", text));
					continue;
				}else {
					interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", current.to_string()));
					interactive_print(&pb, &format!("Status: {}", status));
					break;
				}
			}
			// Tingkatkan nilai current untuk batch berikutnya
			current += 128;
		}
		interactive_print(&pb, &format!(""));
		pb.inc(1);
		batch_number += 1;
	}
	pb.finish();
	Ok(())	
}
async fn process_response(pb: &ProgressBar, v_code: &str, mut log_file: &File, text: &str) -> String {
	let green = "\x1b[32m";
	let yellow = "\x1b[33m";
	let mut rcode = String::new();
	let hasil: Value = serde_json::from_str(&text).expect("REASON");
	if let Some(data_array) = hasil.get("data").and_then(|data| data.as_array()) {
		for data_value in data_array {
			if let Some(vouchers_array) = data_value.get("vouchers").and_then(|vouchers| vouchers.as_array()) {
				for voucher_value in vouchers_array {
					if let Some(voucher_obj) = voucher_value.get("voucher").and_then(|voucher| voucher.as_object()) {
						if let Some(voucher_identifier_obj) = voucher_obj.get("voucher_identifier").and_then(|vi| vi.as_object()) {
							if let Some(v_code_api) = voucher_identifier_obj.get("voucher_code").and_then(|vc| vc.as_str()) {
								// Use a different variable name to avoid shadowing the outer v_code
								let voucher_code_value = v_code_api.to_string();
								let promotion_id = voucher_identifier_obj.get("promotion_id").and_then(|pi| pi.as_i64()).unwrap_or(0);
								let voucher_code = voucher_identifier_obj.get("voucher_code").and_then(|vc| vc.as_str()).unwrap_or("");
								let signature = voucher_identifier_obj.get("signature").and_then(|s| s.as_str()).unwrap_or("");
								let collection_id = data_value.get("collection_id").and_then(|ci| ci.as_str()).unwrap_or("");
								let promotion_id_str = promotion_id.to_string();
								let url = format!("https://shopee.co.id/voucher/details?&evcode={}&from_source=paijo&promotionId={}&signature={}&source=0", url_encode(&encode(voucher_code)), promotion_id_str, signature);
								// Check if v_code matches the found voucher_code
								if v_code.trim() == voucher_code_value {
									// Set the flag to true when a voucher code is found
									print_and_log(&pb, &mut log_file, &format!("Voucher ditemukan :"), "", "", &format!("Voucher ditemukan:"));
									print_and_log(&pb, &mut log_file, &format!("promotion_id      : "), green, &promotion_id_str, &format!("promotion_id: "));
									print_and_log(&pb, &mut log_file, &format!("voucher_code      : "), green, voucher_code, &format!("voucher_code: "));
									print_and_log(&pb, &mut log_file, &format!("signature         : "), green, signature, &format!("signature: "));
									print_and_log(&pb, &mut log_file, &format!("collection_id     : "), green, collection_id, &format!("collection_id: "));
									print_and_log(&pb, &mut log_file, &format!("Link              : "), "", &url, &format!("Link: "));
									// Exit the program if a matching voucher code is found
									interactive_print(&pb, &format!("Voucher code found. Program selesai."));
									process::exit(1);
								} else {
									print_and_log(&pb, &mut log_file, &format!("voucher_code yang ditemukan : "), yellow, voucher_code, &format!("voucher_code yang ditemukan: "));
									print_and_log(&pb, &mut log_file, &format!("collection_id               : "), green, collection_id, &format!("collection_id: "));
									print_and_log(&pb, &mut log_file, &format!("Link                        : "), "", &url, &format!("Link: "));
								}
							}
						}
					}
				}
			}else{
				rcode = "bug".to_string(); 
				return rcode;
			}
		}
	/*} else if !error_res_str.is_empty() {
		interactive_print(&pb, &format!("error: {}", error_res_str));*/
	}else {
		rcode = "none".to_string(); 
		return rcode;
	}
	rcode.to_string()
}
async fn make_http_request(headers: &HeaderMap, json_body: String) -> Result<Response, Error> {
	// Buat klien HTTP
	let client = ClientBuilder::new()
		.danger_accept_invalid_certs(true)
        .impersonate_skip_headers(Impersonate::Chrome131)
        .enable_ech_grease(true)
        .permute_extensions(true)
		.gzip(true)
		//.use_boring_tls(boring_tls_connector) // Use Rustls for HTTPS
		.build()?;

	// Buat permintaan HTTP POST
	let response = client
		.post("https://mall.shopee.co.id/api/v1/microsite/get_vouchers_by_collections")
		.headers(headers.clone())
		.body(json_body)
		.version(Version::HTTP_2) 
		.send()
		.await?;

	Ok(response)
}

async fn api_1(pb: &ProgressBar, cid_1: &str, headers: &HeaderMap, v_code: &str, mut log_file: &File) -> Result<()> {
	let cloned_headers = headers.clone();
	let voucher_request = VoucherCollectionRequest {
		collection_id: cid_1.to_string(),
		component_type: 1,
		component_id: 1708068524282,
		limit: 100,
		microsite_id: 62902,
		offset: 0,
		number_of_vouchers_per_row: 1,
	};
	// Bentuk struct JsonRequest
	let json_request = JsonRequest {
		voucher_collection_request_list: vec![voucher_request],
	};

	// Convert struct to JSON
	let json_body = serde_json::to_string(&json_request)?;
	
	loop {
		let response = make_http_request(&cloned_headers, json_body.clone()).await?;
		let status = response.status();
		let text = response.text().await?;
		if status == reqwest::StatusCode::OK {
			let hasil: Value = serde_json::from_str(&text)?;
			let error_res = hasil.get("error").and_then(|er| er.as_i64()).unwrap_or(0);
			let error_res_str = error_res.to_string();
			let rcode = process_response(&pb, v_code, &log_file, &text).await;
			if rcode == "bug" {
				print_and_log(&pb, &mut log_file, &format!("Bug API 2"), "", "", &format!("Bug API 2"));
				print_and_log(&pb, &mut log_file, &format!("Tidak ada Info vouchers ditemukan untuk collection_id: "), "", cid_1, &format!("collection_id: "));
			}else if rcode == "none" {
				print_and_log(&pb, &mut log_file, &format!("Tidak ada data ditemukan untuk collection_id: "), "", cid_1, &format!("collection_id: "));
				print_and_log(&pb, &mut log_file, &format!("error : "), "", &error_res_str, &format!("error : "));
				print_and_log(&pb, &mut log_file, &format!("Body  : "), "", &text, &format!("Body  : "));
			}
			break;
		}else if status == reqwest::StatusCode::IM_A_TEAPOT {
			interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", cid_1));
			interactive_print(&pb, &format!("Gagal, status code: 418 - I'm a teapot. Mencoba kembali..."));
			interactive_print(&pb, &format!("{}", text));
			continue;
		}else {
			interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", cid_1));
			interactive_print(&pb, &format!("Status: {}", status));
			break;
		}
	}
	Ok(())	
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opt = Opt::from_args();
	let version_info = env!("CARGO_PKG_VERSION");
	println!("-------------------------------------------");
	println!("get_vouchers_by_collections [Version {}]", version_info);
	println!("");
	println!("Dapatkan Info terbaru di https://google.com");
	println!("");
	println!("-------------------------------------------");

	if opt.cookie {
		println!("Metode Cookie: Memilih cookie dan menyimpannya di file akun.conf");
		choose_and_save_cookie()?;
	}else if !opt.start.is_none() && !opt.end.is_none() && !opt.v_code.is_none() && opt.file.is_none() {
		println!("Metode Cepat: Menjalankan main.exe dengan tiga argumen.");
		process_arguments(
			opt.start.as_ref().map(String::as_str).unwrap_or_default(),
			opt.end.as_ref().map(String::as_str).unwrap_or_default(),
			opt.v_code.as_ref().map(String::as_str).unwrap_or_default(),
		).await?;
	} else {
		manual_input(&opt).await?;
	}
	Ok(())
}
fn choose_cookie() -> Result<String> {
	// Menampilkan daftar file cookie yang tersedia
	println!("Daftar file cookie yang tersedia:");
	let files = std::fs::read_dir("./akun")?;
	let mut file_options = Vec::new();
	for (index, file) in files.enumerate() {
		if let Ok(file) = file {
			let file_name = file.file_name();
			println!("{}. {}", index + 1, file_name.to_string_lossy());
			file_options.push(file_name.to_string_lossy().to_string());
		}
	}
	// Pilih nomor file cookie yang ingin digunakan
	let selected_file = loop {
		let input = get_user_input("Pilih nomor file cookie yang ingin digunakan: ");

		// Konversi input ke nomor indeks
		if let Ok(index) = input.trim().parse::<usize>() {
			if index > 0 && index <= file_options.len() {
				break file_options[index - 1].clone();
			}
		}
	};
	Ok(selected_file)
}
fn choose_and_save_cookie() -> Result<()> {
	let selected_file = choose_cookie();
	// Simpan nama file cookie yang dipilih ke dalam akun.conf
	let mut akun_conf_file = File::create("akun.conf")?;
	write!(akun_conf_file, "{:?}", selected_file)?;

	Ok(())
}
fn get_user_input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}
fn fix_start (start: &str) -> i64 {
	let start: i64 = start.trim().parse().expect("Input tidak valid");
	println!("Check Start");
	let x = (start - 8) as f64 / 128.0;
	if x.fract() == 0.0 {
		println!("Benar");
		start
	} else {
		println!("Hitung pendekatannya");
		let rounded_up = x.ceil() as i64;
		println!("Pembulatan ke atas: {}", rounded_up);
		let mulai = (rounded_up * 128) - 120;
		mulai
	}
}

#[cfg(windows)]
fn bar(batch_count: &i64) -> ProgressBar {
	if OsVersion::current() <= OsVersion::new(6, 3, 0, 9800) {
		ProgressBar::hidden()
	} else {
		let progress_bar = ProgressBar::new(*batch_count as u64);
		progress_bar.set_style(ProgressStyle::default_bar()
			.template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {percent}% {msg}")
			.expect("Failed to set progress bar style")
			.progress_chars("█░"));
		progress_bar
	}
}
#[cfg(not(windows))]
fn bar(batch_count: &i64) -> ProgressBar {
	let progress_bar = ProgressBar::new(*batch_count as u64);
	progress_bar.set_style(ProgressStyle::default_bar()
		.template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {percent}% {msg}")
		.expect("Failed to set progress bar style")
		.progress_chars("█░"));
	progress_bar
}