//This Is a first version of get_vouchers_by_collections
//This version using api reqwest
//Whats new In 1.0.3 :
//add ansi colour
//add log file same as python

use reqwest;
use reqwest::ClientBuilder;
use std::io;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use anyhow::Result;
use reqwest::Version;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::prelude::*;
use ansi_term::Colour;
use chrono::{DateTime, Utc, NaiveDateTime};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Input nilai start dan akhir
    let mut start = String::new();
    let mut end = String::new();
	let mut v_code = String::new();
	let formatted_datetime = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
	
	println!("get_vouchers_by_collections [Version 1.0.3]");
	println!("");
	println!("Dapatkan Info terbaru di https://google.com");
	println!("");
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
		let input = env::var("COOKIE_IDX")
			.expect("Environment variable COOKIE_IDX tidak ditemukan");

		// Konversi input ke nomor indeks
		if let Ok(index) = input.trim().parse::<usize>() {
			if index > 0 && index <= file_options.len() {
				break file_options[index - 1].clone();
			}
		}
	};

	// Baca isi file cookie
	let file_path = format!("./akun/{}", selected_file);
	let mut cookie_content = String::new();
	File::open(&file_path)?.read_to_string(&mut cookie_content)?;
	
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
			
	println!("Contoh input: Awal: 12905192072, Akhir: 12905192100");
    println!("Masukkan nilai start:");
    let start = env::var("START_COLLECTION_ID")
		.expect("Environment variable START_COLLECTION_ID tidak ditemukan");

    println!("Masukkan nilai akhir:");
    let end = env::var("END_COLLECTION_ID")
		.expect("Environment variable END_COLLECTION_ID tidak ditemukan");
	
	println!("Contoh input DC10010RB1109");
	println!("Masukkan voucher_code:");
	let v_code = env::var("VOUCHER_CODE")
		.expect("Environment variable VOUCHER_CODE tidak ditemukan");

    // Parse nilai start dan akhir ke dalam tipe data integer
    let start: i64 = start.trim().parse().expect("Input tidak valid");
    let end: i64 = end.trim().parse().expect("Input tidak valid");

    // Jumlah nilai per batch
    let values_per_batch = 5;

    // Hitung jumlah batch
    let batch_count = ((end - start) / 128 / values_per_batch) + 1;

    // Iterasi dan menuliskan angka dengan jarak 128
    let mut batch_number = 1;
    let mut current = start;

    for _ in 0..batch_count {
        println!("Batch {} of {}", batch_number, batch_count);

        for _ in 0..values_per_batch {
            // Bentuk struct VoucherCollectionRequest
            let voucher_request = VoucherCollectionRequest {
                collection_id: current.to_string(),
                component_type: 2,
                component_id: 1694165901230,
                limit: 1,
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
			headers.insert("af-ac-enc-dat", reqwest::header::HeaderValue::from_static("AAczLjMuMC0yAAABjD2/+9wAAA+nAyAAAAAAAAAAAv0CU/YKNLZ3x+BnWz70VZfkPeXOl/82EyfDx/bVRcPaaRvYm5f/NhMnw8f21UXD2mkb2JseRmK2cCMhvxPnkQMgYOR3MdO0gnVkjIgJW23lZmQ6sCWooZQKR7gDVwwCOYI1dY53Klz0SQJAiMqSWXTebQiMBSCy14MigrBSynoPM/sJ55f/NhMnw8f21UXD2mkb2Jtn/ITcW+wLKZcVOXYrFuPG8oyOVk+CDzd2jz4zfQgqkxN5QCFuh0m2a7iQsFUA38jt78nvQtHCyYkKdmCs2BOm0RySdhyhKzToeXrsCr/7VgmrRqWZ6CUplt6oFbQx+d+S131dA11Iyo+it+R0CjI1jVVTM0ZXbzLLjOMDkk0u56JlVsxG5BkGVyzOLMyu0PDy2k1+VdkT4pgXk5lHU3OPYXlO2iSYnkJub+qCrFiPR7t/x/v9SPKXNqWKS7oOCZ9xcVTz7HDD3cgFNYM1SCF4KX0sQZdHIFpWrefIgEZDRt7bpc6e6UvvoL4f9+4rwqv23iZus44hunvXxHju3CIGekVQqyQYHKRy3gkwLDX1fSh6bAojldn5JgJweNxrodaDu4ubLr4bcUOLBA2c2bmdlgH+2CNyd5v6F9lcOXQq5szoUhKAg6vJXYPkXE5ho+pc0XG8frvsLhQK78fs7www8WJ6JV771K7M3S4Ty2Ncm8vFui5C+Cokhc47s9IEFIsGDUtpEpNSSI0oOt3tagOTHkenEhnNI10zaevKlBjvvsvFui5C+Cokhc47s9IEFIsGDUtpEpNSSI0oOt3tagOTUWJy02xIVh23BkGRTuyWQfap+xOCy0qf6FceYCNc1JRGrbfJWuRYlFp+J0tcUk6vRQg7nCQX6c8aLMHSU1udIlj4f3pW05can3G4luWGcOFGrbfJWuRYlFp+J0tcUk6vMzJkOKM7a6hSXhvQxqV9Gr75OCXRj3ppyFQjfu1/3QAwDau2Xiuafq3FAfj0VLRrH3wBwU+KzLIFlwnnaaSPkUHfwoKFdwD4S3t6s0u0cnMeoZffwAq9Vd5qYWkImDtWIq9wd+MfTC4cdHCfaUSI7Q=="));
			headers.insert("x-sap-access-s", reqwest::header::HeaderValue::from_static("LwGv74_7pqcgSlOERyAuF3XJ4Xw9IZ6gWvo_ZdVuFJA="));
			headers.insert("x-csrftoken", reqwest::header::HeaderValue::from_static("6du999g4UlXCglP4gjLi1wp6RzWoa4BW"));
			headers.insert("sec-ch-ua-platform", reqwest::header::HeaderValue::from_static("\"Windows\""));
			headers.insert("x-sap-sec", reqwest::header::HeaderValue::from_static("kDXjm6XKbFr0xFr0ZJr+xFN0ZJr0xFN0xFr7xFr0uFp0xDjKxFOaxmr0ZFr0x1fAr0b+xFr0eF50x1j2xFPIDZAA+FkzOI22AYpdVDxPaBB5QSI5j9U3mrFs9tDTUeoMYq3vGOSVZet5IaRgpFKPx1Er7/d8Rb9kxcl6yBWBZbMNSTZCd4adh1HH0bhfVHZkidUCgGqjr45Jzx1JVnmPNZcJ+ehT/nAFH7xt16ef1OF6zzDYBZEe3Fy1DlrEB+ReGk8T2J0PUm7NOtCLUmbqvGLcTPgDjBqf1S0ffGWTwTmvx8tkwThmeS9h7DMdv57LbEvhRBhkGbGmYnYmjdfgykMGjvg3dDNzCtHPPyQWiFk324WM0JIt4ORp3VNqh8Z4bgsgluTEjauT+IvbjDdJDhTl2K5vKC0NCCgtrtRTqnfwBge0+ZB0iHfYHpFCOxTLo0aJNa1eLjBvhT4oGPgKEHGfe0RDkCi+KT1DdspJEccEn3KGBi7q8iUzz0HSwH/x0RVrtHaHnZBdC2oxhs++qJSqKu0p6sFUQe6IeUW+WiHlfINlkFslqdFFr7zmxgfXhRzHGR7cIp2yUWdNbhddLp95LPq13zttiDUQb5KzMB3fcxQgv4Xc32c1Sx7gCvMLovrcGsiKxFGmsDfS5TfzEanxiN5zUgJ1XSj+xFr0fdRVFr0xg4N0xFr00qxFrcX0xFr4xFr0GFr0xXtU9cqzp1ZLM0zpHTYuuskNWjpzaFr0xT+CfdNUF40wxFr0xtfFr0b+xFr0wFr0xDN0xFO9BImbC+SDBfZuZziUGD52wjmeOJX0xFkGFj4wFTTbDmr0xFr+xFj0aFr7xFX0xFr+xFr0wFr0xDN0xFOmsOpUxzmjjqTe2jVcMkU3QpdGEmX0xFrtgdRnfdRufJr0xFO="));
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
                .headers(headers)
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
												println!("{}", Colour::Green.paint("Voucher ditemukan:"));
												println!("promotion_id: {}", Colour::Green.paint(promotion_id_str));
												println!("voucher_code: {}", Colour::Green.paint(voucher_code));
												println!("signature: {}", Colour::Green.paint(signature));
												println!("collection_id: {}", Colour::Green.paint(collection_id));
												// Simpan data voucher yang ditemukan ke dalam berkas log dengan nama berkas yang baru dibuat
												writeln!(&mut log_file, "Voucher ditemukan:").expect("Gagal menulis ke file log");
												writeln!(&mut log_file, "promotion_id: {}", promotion_id).expect("Gagal menulis ke file log");
												writeln!(&mut log_file, "voucher_code: {}", voucher_code).expect("Gagal menulis ke file log");
												writeln!(&mut log_file, "signature: {}", signature).expect("Gagal menulis ke file log");
												writeln!(&mut log_file, "collection_id: {}", collection_id).expect("Gagal menulis ke file log");

												// Exit the program if a matching voucher code is found
												println!("Voucher code found. Program selesai.");
												return Ok(());
											} else {
												println!("voucher_code yang ditemukan: {}", Colour::Yellow.paint(voucher_code));
												println!("collection_id: {}", Colour::Green.paint(collection_id));
												writeln!(&mut log_file, "voucher_code yang ditemukan: {}", voucher_code).expect("Gagal menulis ke file log");
												writeln!(&mut log_file, "collection_id: {}", collection_id).expect("Gagal menulis ke file log");
											}
										}
									}
								}
							}
						}
					}
				}else {
					println!("Tidak ada data ditemukan untuk collection_id: {}", current.to_string());
					writeln!(&mut log_file, "collection_id: {}", current.to_string()).expect("Gagal menulis ke file log");
				}
			}else {
				println!("POST request gagal untuk collection_id:: {}", current.to_string());
				println!("Status: {}", status);
			}
            // Tingkatkan nilai current untuk batch berikutnya
            current += 128;
        }
        println!();
        batch_number += 1;
    }
    Ok(())
}
