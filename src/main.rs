/*
This Is a first version of get_vouchers_by_collections
This version using api reqwest
Whats new In 1.1.7 :
fix for windows 7 console
Whats new In 1.1.6 :
fix included ansicode on logs
Whats new In 1.1.5 :
Add function interactive_print
*/

use reqwest;
use reqwest::ClientBuilder;
use reqwest::header::HeaderMap;
use std::io;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use anyhow::Result;
use reqwest::Version;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;
use std::process;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::{Local, Utc};
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

async fn process_arguments2(start: &str, end: &str, v_code: &str, selected_file: &str) -> Result<()> {
    // Read the content of "akun.conf"
    let file_path = format!("./akun/{}", selected_file);
    let mut cookie_content = String::new();
    File::open(&file_path)?.read_to_string(&mut cookie_content)?;

    // Process HTTP with common function
    some_function(start, end, v_code, &cookie_content, &selected_file).await?;

    Ok(())
}

async fn manual_input() -> Result<()> {
    let mut start = String::new();
    let mut end = String::new();
    let mut v_code = String::new();
    
    // Display the list of available cookie files
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

    // Select the file number for the cookie
    let selected_file = loop {
        println!("Pilih nomor file cookie yang ingin digunakan:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        // Convert input to index number
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= file_options.len() {
                break file_options[index - 1].clone();
            }
        }
    };

    // Read the content of the selected cookie file
    let file_path = format!("./akun/{}", selected_file);
    let mut cookie_content = String::new();
    File::open(&file_path)?.read_to_string(&mut cookie_content)?;
	
	println!("Contoh input: Awal: 12905192072, Akhir: 12905192100");
    println!("Masukkan nilai start:");
    io::stdin().read_line(&mut start).expect("Gagal membaca baris");

    println!("Masukkan nilai akhir:");
    io::stdin().read_line(&mut end).expect("Gagal membaca baris");
	
	println!("Contoh input DC10010RB1109");
	println!("Masukkan voucher_code:");
    io::stdin().read_line(&mut v_code).expect("Gagal membaca baris");

    // Process HTTP with common function
    match some_function(&start, &end, &v_code, &cookie_content, &selected_file).await {
    Ok(_) => {},
    Err(err) => eprintln!("Error: {}", err),
	}
    Ok(())
}

fn print_and_log(pb: &ProgressBar, mut log_file: &File, mes1: &str, color: &str, mes2: &str, logmes: &str) {
	let reset_color = "\x1b[0m";
	#[cfg(windows)]
	if OsVersion::current() <= OsVersion::new(6, 3, 0, 9800) {
		interactive_print(pb, &format!("{}{}", mes1, mes2));
	}else{
		interactive_print(pb, &format!("{}{}{}{}", mes1, color, mes2, reset_color));
	}
    // Menyimpan data ke dalam berkas log
    writeln!(log_file, "{}{}", logmes, mes2).expect("Gagal menulis ke file log");
}

fn interactive_print(pb: &ProgressBar, message: &str) {
	let is_interactive = atty::is(atty::Stream::Stdout);
	#[cfg(windows)]
	if cfg!(windows) && OsVersion::current() <= OsVersion::new(6, 2, 0, 9800){
		println!("{}", format!("{}", message));
	}else if is_interactive {
		pb.println(format!("{}", message));
	}else{
		println!("{}", format!("{}", message));
	}
}

async fn some_function(start: &str, end: &str, v_code: &str, cookie_content: &str, selected_file: &str) -> Result<()> {
	let green = "\x1b[32m";
    let yellow = "\x1b[33m";
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
	
	let start: i64 = start.trim().parse().expect("Input tidak valid");
	let end: i64 = end.trim().parse().expect("Input tidak valid");

    // Jumlah nilai per batch
    let values_per_batch = 5;

    // Hitung jumlah batch
    let batch_count = ((end - start) / 128 / values_per_batch) + 1;

    // Iterasi dan menuliskan angka dengan jarak 128
    let mut batch_number = 1;
    let mut current = start;
	#[cfg(windows)]
    let pb = if cfg!(windows) {
        if OsVersion::current() <= OsVersion::new(6, 3, 0, 9800) {
            ProgressBar::hidden()
        } else {
            let progress_bar = ProgressBar::new(batch_count.try_into().unwrap());
            progress_bar.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {percent}% {msg}")?
                .progress_chars("█░"));
            progress_bar
        }
	} else {
		let progress_bar = ProgressBar::new(batch_count.try_into().unwrap());
		progress_bar.set_style(ProgressStyle::default_bar()
			.template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {percent}% {msg}")?
			.progress_chars("█░"));
		progress_bar
    };
	#[cfg(not(windows))]{
        let pb = ProgressBar::new(batch_count.try_into().unwrap());
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {percent}% {msg}")?
            .progress_chars("█░"));
    };

    for _ in 0..batch_count {
        interactive_print(&pb, &format!("[{}] Batch {} of {}", Local::now().format("%H:%M:%S.%3f"), batch_number, batch_count));

        for _ in 0..values_per_batch {
            // Bentuk struct VoucherCollectionRequest
            let voucher_request = VoucherCollectionRequest {
                collection_id: current.to_string(),
                component_type: 2,
                component_id: 1694165901230,
                limit: 100,
                microsite_id: 58982,
                offset: 0,
                number_of_vouchers_per_row: 1,
            };
			
			let mut headers = reqwest::header::HeaderMap::new();
			headers.insert("sec-ch-ua", reqwest::header::HeaderValue::from_static("\"Chromium\";v=\"119\", \"Not)A;Brand\";v=\"24\", \"Google Chrome\";v=\"119\""));
			headers.insert("x-sap-access-f", reqwest::header::HeaderValue::from_static("3.2.119.2.0|13|3.3.0-2_5.1.0_0_343|3f8e71489e604fe39d386d8a6810764f4b299d79ad9a4d|10900|1100"));
			headers.insert("x-sz-sdk-version", reqwest::header::HeaderValue::from_static("3.3.0-2&1.6.8"));
			headers.insert("x-shopee-language", reqwest::header::HeaderValue::from_static("id"));
			headers.insert("x-requested-with", reqwest::header::HeaderValue::from_static("XMLHttpRequest"));
			headers.insert("x-sap-access-t", reqwest::header::HeaderValue::from_static("1694342213"));
			headers.insert("af-ac-enc-dat", reqwest::header::HeaderValue::from_static(""));
			headers.insert("x-sap-access-s", reqwest::header::HeaderValue::from_static("LwGv74_7pqcgSlOERyAuF3XJ4Xw9IZ6gWvo_ZdVuFJA="));
			headers.insert("x-csrftoken", reqwest::header::HeaderValue::from_static("6du999g4UlXCglP4gjLi1wp6RzWoa4BW"));
			headers.insert("sec-ch-ua-platform", reqwest::header::HeaderValue::from_static("\"Windows\""));
			headers.insert("x-sap-sec", reqwest::header::HeaderValue::from_static(""));
			headers.insert("sec-ch-ua-mobile", reqwest::header::HeaderValue::from_static("?0"));
			headers.insert("user-agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
			headers.insert("x-api-source", reqwest::header::HeaderValue::from_static("pc"));
			headers.insert("content-type", reqwest::header::HeaderValue::from_static("application/json"));
			headers.insert("accept", reqwest::header::HeaderValue::from_static("application/json"));
			headers.insert("af-ac-enc-sz-token", reqwest::header::HeaderValue::from_str(&sz_token_content.trim())?);
			headers.insert("origin", reqwest::header::HeaderValue::from_static("https://shopee.co.id"));
			headers.insert("sec-fetch-site", reqwest::header::HeaderValue::from_static("same-origin"));
			headers.insert("sec-fetch-mode", reqwest::header::HeaderValue::from_static("cors"));
			headers.insert("sec-fetch-dest", reqwest::header::HeaderValue::from_static("empty"));
			headers.insert("referer", reqwest::header::HeaderValue::from_static("https://shopee.co.id/m/9-9"));
			headers.insert("accept-encoding", reqwest::header::HeaderValue::from_static("gzip, deflate"));
			headers.insert("accept-language", reqwest::header::HeaderValue::from_static("en-US,en;q=0.9,id;q=0.8"));
			headers.insert(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(&cookie_content)?);

            // Bentuk struct JsonRequest
            let json_request = JsonRequest {
                voucher_collection_request_list: vec![voucher_request],
            };

            // Convert struct to JSON
            let json_body = serde_json::to_string(&json_request)?;

			let client = ClientBuilder::new()
				.gzip(true)
				.use_rustls_tls() // Use Rustls for HTTPS
				.build()?;

            // Buat permintaan HTTP POST
            let response = client
                .post("https://shopee.co.id/api/v1/microsite/get_vouchers_by_collections")
                .header("Content-Type", "application/json")
                .headers(headers.clone())
                .body(json_body)
				.version(Version::HTTP_2) 
                .send()
                .await?;
			// Check for HTTP status code indicating an error
			//let http_version = response.version(); 		// disable output features
			//println!("HTTP Version: {:?}", http_version); // disable output features
			let log_filename = format!("{}-{}_{}.log", start , end , formatted_datetime);
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
				// Access specific values using serde_json::Value methods
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
											// Check if v_code matches the found voucher_code
											if v_code.trim() == voucher_code_value {
												let promotion_id_str = promotion_id.to_string();
												// Set the flag to true when a voucher code is found
												print_and_log(&pb, &mut log_file, &format!("Voucher ditemukan:"), "", "", &format!("Voucher ditemukan:"));
												print_and_log(&pb, &mut log_file, &format!("promotion_id: "), green, &promotion_id_str, &format!("promotion_id: "));
												print_and_log(&pb, &mut log_file, &format!("voucher_code: "), green, voucher_code, &format!("voucher_code: "));
												print_and_log(&pb, &mut log_file, &format!("signature: "), green, signature, &format!("signature: "));
												print_and_log(&pb, &mut log_file, &format!("collection_id: "), green, collection_id, &format!("collection_id: "));
												// Exit the program if a matching voucher code is found
												interactive_print(&pb, &format!("Voucher code found. Program selesai."));
												return Ok(());
											} else {
												print_and_log(&pb, &mut log_file, &format!("voucher_code yang ditemukan: "), yellow, voucher_code, &format!("voucher_code yang ditemukan: "));
												print_and_log(&pb, &mut log_file, &format!("collection_id: "), green, collection_id, &format!("collection_id: "));
											}
										}
									}
								}
							}
						}else{
							print_and_log(&pb, &mut log_file, &format!("API Checker 1"), "", "", &format!("API Checker 1"));
							let cid_1 = current.to_string();
							api_1(&pb, &cid_1, &headers.clone(), v_code, &log_file).await?;
						}
					}
				}else {
					print_and_log(&pb, &mut log_file, &format!("Tidak ada data ditemukan untuk collection_id: "), "", &current.to_string(), &format!("collection_id: "));
				}
			}else {
				interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", current.to_string()));
				interactive_print(&pb, &format!("Status: {}", status));
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

async fn api_1(pb: &ProgressBar, cid_1: &str, headers: &HeaderMap, v_code: &str, mut log_file: &File) -> Result<()> {
	let green = "\x1b[32m";
    let yellow = "\x1b[33m";
	let cloned_headers = headers.clone();
	let voucher_request = VoucherCollectionRequest {
		collection_id: cid_1.to_string(),
		component_type: 1,
		component_id: 1694165901230,
		limit: 100,
		microsite_id: 58982,
		offset: 0,
		number_of_vouchers_per_row: 1,
	};
	// Bentuk struct JsonRequest
	let json_request = JsonRequest {
		voucher_collection_request_list: vec![voucher_request],
	};

	// Convert struct to JSON
	let json_body = serde_json::to_string(&json_request)?;

	let client = ClientBuilder::new()
		.gzip(true)
		.use_rustls_tls() // Use Rustls for HTTPS
		.build()?;

	// Buat permintaan HTTP POST
	let response = client
		.post("https://shopee.co.id/api/v1/microsite/get_vouchers_by_collections")
		.header("Content-Type", "application/json")
		.headers(cloned_headers)
		.body(json_body)
		.version(Version::HTTP_2) 
		.send()
		.await?;
	// Check for HTTP status code indicating an error
	//let http_version = response.version(); 		// disable output features
	//println!("HTTP Version: {:?}", http_version); // disable output features
	let status = response.status();
	let text = response.text().await?;	
	if status == reqwest::StatusCode::OK {
		let hasil: Value = serde_json::from_str(&text)?;
		// Access specific values using serde_json::Value methods
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
									// Check if v_code matches the found voucher_code
									if v_code.trim() == voucher_code_value {
										let promotion_id_str = promotion_id.to_string();
										// Set the flag to true when a voucher code is found
										print_and_log(&pb, &mut log_file, &format!("Voucher ditemukan:"), "", "", &format!("Voucher ditemukan:"));
										print_and_log(&pb, &mut log_file, &format!("promotion_id: "), green, &promotion_id_str, &format!("promotion_id: "));
										print_and_log(&pb, &mut log_file, &format!("voucher_code: "), green, voucher_code, &format!("voucher_code: "));
										print_and_log(&pb, &mut log_file, &format!("signature: "), green, signature, &format!("signature: "));
										print_and_log(&pb, &mut log_file, &format!("collection_id: "), green, collection_id, &format!("collection_id: "));

										// Exit the program if a matching voucher code is found
										interactive_print(&pb, &format!("Voucher code found. Program selesai."));
										process::exit(1);
									} else {
										print_and_log(&pb, &mut log_file, &format!("voucher_code yang ditemukan: "), yellow, voucher_code, &format!("voucher_code yang ditemukan: "));
										print_and_log(&pb, &mut log_file, &format!("collection_id: "), green, collection_id, &format!("collection_id: "));
									}
								}
							}
						}
					}
				}else{
					print_and_log(&pb, &mut log_file, &format!("Bug API 2"), "", "", &format!("Bug API 2"));
					print_and_log(&pb, &mut log_file, &format!("Tidak ada Info vouchers ditemukan untuk collection_id: "), "", cid_1, &format!("collection_id: "));
				}
			}
		}else {
			print_and_log(&pb, &mut log_file, &format!("Tidak ada data ditemukan untuk collection_id: "), "", cid_1, &format!("collection_id: "));
		}
	}else {
		interactive_print(&pb, &format!("POST request gagal untuk collection_id:: {}", cid_1));
		interactive_print(&pb, &format!("Status: {}", status));
	}
	Ok(())	
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if there are command line arguments
    let args: Vec<String> = std::env::args().collect();
	
	println!("-------------------------------------------");
	println!("get_vouchers_by_collections [Version 1.1.7]");
	println!("");
	println!("Dapatkan Info terbaru di https://google.com");
	println!("");
	println!("-------------------------------------------");

	// Mengecek jenis metode berdasarkan jumlah argumen
    match args.len() {
        2 if args[1] == "cookie" => {
            // Metode Cookie: Memilih cookie dan menyimpannya di file akun.conf
            println!("Metode Cookie: Memilih cookie dan menyimpannya di file akun.conf");

            // Memanggil fungsi untuk memilih cookie dan menyimpannya
            choose_and_save_cookie()?;
        }
        4 => {
            // Metode Cepat: Menjalankan main.exe dengan tiga argumen
            println!("Metode Cepat: Menjalankan main.exe dengan tiga argumen.");
            
			// Case: Command-line arguments provided (e.g., "main.exe 12905192072 12905200000 DC10010RB110")
			let start = &args[1];
			let end = &args[2];
			let v_code = &args[3];
			process_arguments(start, end, v_code).await?;
        }
        5 => {
            // Metode Cepat: Menjalankan main.exe dengan empat argumen
            println!("Metode Cepat: Menjalankan main.exe dengan empat argumen.");
            
			// Case: Command-line arguments provided (e.g., "main.exe 12905192072 12905200000 DC10010RB110 asu.txt")
			let start = &args[1];
			let end = &args[2];
			let v_code = &args[3];
			let selected_file = &args[4];
			process_arguments2(start, end, v_code, selected_file).await?;
		}
        _ => {
            // Case: No command-line arguments (old manual input method)
            manual_input().await?;
        }
    }
    Ok(())
}

fn choose_and_save_cookie() -> Result<(), Box<dyn std::error::Error>> {
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
        println!("Pilih nomor file cookie yang ingin digunakan:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        // Konversi input ke nomor indeks
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= file_options.len() {
                break file_options[index - 1].clone();
            }
        }
    };
    // Simpan nama file cookie yang dipilih ke dalam akun.conf
    let mut akun_conf_file = File::create("akun.conf")?;
    write!(akun_conf_file, "{}", selected_file)?;

    Ok(())
}