use tower_lsp::jsonrpc::Result;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::{InitializeParams, InitializeResult, MessageType};
use std::process::{Command, Stdio};
use serde_json::json;
use std::io::{Read, Write};
use std::io::{BufRead, BufReader, copy};

use serde_json::Value;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

#[derive(Debug)]
struct Backend {
    client: Client
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: tower_lsp::lsp_types::InitializedParams) {
        self.client.log_message(MessageType::INFO, "Initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}



fn start_language_server(language: &str) -> std::process::Child {
    match language {
        "python" => Command::new("pylsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Python language server"),
        "rust" => Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start Rust language server"),
        _ => panic!("Language not supported"),
    }
}


fn initialize_language_server(child: &mut std::process::Child, root_uri: &str) {
    let stdin = child.stdin.as_mut().unwrap();
    let initialize_params = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "semanticTokens": {
                        "dynamicRegistration": false,
                        "tokenTypes": ["namespace", "class", "enum", "function", "variable"],
                        "tokenModifiers": ["declaration", "definition", "readonly"],
                    }
                }
            }
        }
    });

    writeln!(stdin, "Content-Length: {}\r\n\r\n{}", initialize_params.to_string().len(), initialize_params.to_string()).unwrap();
}

fn request_semantic_tokens(child: &mut std::process::Child, file_uri: &str) {
    let stdin = child.stdin.as_mut().unwrap();
    let semantic_tokens_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/semanticTokens/full",
        "params": {
            "textDocument": {
                "uri": file_uri
            }
        }
    });

    writeln!(stdin, "Content-Length: {}\r\n\r\n{}", semantic_tokens_request.to_string().len(), semantic_tokens_request.to_string()).unwrap();
}



fn receive_semantic_tokens(child: &mut std::process::Child) -> serde_json::Value {
    let stdout = child.stdout.as_mut().unwrap();
    let mut reader = BufReader::new(stdout);
    let mut response = String::new();

    reader.read_line(&mut response).unwrap();  // Read the content length header
    let content_length: usize = response.split_whitespace().nth(1).unwrap().parse().unwrap();

    let mut response_body = vec![0; content_length];
    reader.read_exact(&mut response_body).unwrap();

    serde_json::from_slice(&response_body).unwrap()
}

fn decode_semantic_tokens(tokens: &[u32]) -> Vec<(usize, usize, usize, String, Vec<String>)> {
    let mut decoded_tokens = Vec::new();
    let mut line = 0;
    let mut column = 0;

    let token_types = vec!["namespace", "class", "enum", "function", "variable"];
    let token_modifiers = vec!["declaration", "definition", "readonly"];

    for chunk in tokens.chunks(5) {
        line += chunk[0] as usize;
        if chunk[0] != 0 {
            column = chunk[1] as usize;
        } else {
            column += chunk[1] as usize;
        }
        let length = chunk[2] as usize;
        let token_type = token_types[chunk[3] as usize].to_string();
        let modifiers: Vec<String> = (0..token_modifiers.len())
            .filter(|&i| (chunk[4] >> i) & 1 != 0)
            .map(|i| token_modifiers[i].to_string())
            .collect();

        decoded_tokens.push((line, column, length, token_type, modifiers));
    }

    decoded_tokens
}




// Struct to hold the relevant metadata for an LSP extension
#[derive(Debug)]
struct LspExtension {
    name: String,
    publisher: String,
    version: String,
    download_url: String,
}

// Function to search for LSPs based on a user query
async fn search_lsp(query: &str) -> Result<Vec<LspExtension>> {
    let url = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=1.93";
    let client = reqwest::Client::new();

    // Marketplace search query for the given string (LSP search term)
    let body = format!(
        r#"{{
            "filters": [{{
                "criteria": [{{
                    "filterType": 10,
                    "value": "{}"
                }}]
            }}],
            "assetTypes": [],
            "flags": 131
        }}"#,
        query
    );

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await.unwrap();

    println!("{:?}", response);

    let json: Value = response.json().await.unwrap();

    println!("{:?}", json);

    // Parse the result and extract LSP metadata
    let mut lsp_extensions = Vec::new();
    if let Some(extensions) = json["results"][0]["extensions"].as_array() {
        for ext in extensions {
            let name = ext["extensionName"].as_str().unwrap().to_string();
            let publisher = ext["publisher"]["publisherName"].as_str().unwrap().to_string();
            let version = ext["versions"][0]["version"].as_str().unwrap().to_string();
            let asset_url = ext["versions"][0]["files"]
                .as_array()
                .unwrap()
                .iter()
                .find(|file| file["assetType"] == "Microsoft.VisualStudio.Services.VSIXPackage")
                .unwrap()["source"]
                .as_str()
                .unwrap()
                .to_string();

            lsp_extensions.push(LspExtension {
                name,
                publisher,
                version,
                download_url: asset_url,
            });
        }
    }

    Ok(lsp_extensions)
}

// Function to download a specific LSP extension
async fn download_lsp_vsix(lsp: &LspExtension) -> Result<()> {
    let response = reqwest::get(&lsp.download_url).await.unwrap();
    let mut file = File::create(format!("{}.vsix", &lsp.name)).unwrap();
    let content = response.bytes().await.unwrap();

    copy(&mut content.as_ref(), &mut file).unwrap();

    Ok(())
}

// A helper function to sanitize file paths
fn sanitize_path(file_name: &str, destination: &Path) -> io::Result<PathBuf> {
    // Convert the file name into a Path
    let path = Path::new(file_name);

    // Ensure the path is relative and doesn't contain ".."
    let safe_path = path
        .components()
        .filter(|component| match component {
            std::path::Component::Normal(_) => true,   // Only allow normal components
            _ => false,  // Disallow things like ".." or absolute paths
        })
        .collect::<PathBuf>();

    // Prepend the destination directory
    let full_path = destination.join(safe_path);

    Ok(full_path)
}

// Function to extract the `.vsix` file safely
fn extract_vsix(file_name: &str, destination: &Path) -> Result<()> {
    let file = File::open(file_name).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        // Sanitize the file name
        let outpath = sanitize_path(file.name(), destination).unwrap();

        // Check if it's a directory or file
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}



#[cfg(test, target_os=windows)]
mod lsp_tests {
    use std::fs;
    use std::path::Path;
    use crate::language_server_protocol::{decode_semantic_tokens, download_lsp_vsix, extract_vsix, initialize_language_server, receive_semantic_tokens, request_semantic_tokens, search_lsp, start_language_server};

    #[test]
    pub fn test() {
        if !fs::exists("C:/Users/Westb/Desktop/Python-Projects/youtube-thingy/").unwrap() {
            return;
        }

        let rt = tokio::runtime::Runtime::new().unwrap();

        let search_query = "";
        let lsp_list = rt.block_on(search_lsp(search_query)).unwrap();

        // Print the found LSPs and download the first one
        if !lsp_list.is_empty() {
            println!("Found LSP extensions: {:?}", lsp_list);

            let lsp = &lsp_list[0];  // Use the first LSP in the result list
            println!("Downloading: {:?}", lsp);
            rt.block_on(download_lsp_vsix(lsp)).unwrap();

            let destination = Path::new("./extensions/lsp/");

            extract_vsix(&format!("{}.vsix", &lsp.name), &destination).unwrap();
            println!("LSP extracted!");
        } else {
            println!("No LSP found for query: {}", search_query);
        }


        let mut child = start_language_server("python");
        initialize_language_server(&mut child, "file:///C:/Users/Westb/Desktop/Python-Projects/youtube-thingy/");
        request_semantic_tokens(&mut child, "file:///C:/Users/Westb/Desktop/Python-Projects/youtube-thingy/main.py");
        let response = receive_semantic_tokens(&mut child);
        let tokens = decode_semantic_tokens(response["result"]["data"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect::<Vec<_>>().as_slice());

        // Render tokens in the IDE
        for (line, col, len, token_type, modifiers) in tokens {
            println!("Token at ({}, {}), length: {}, type: {}, modifiers: {:?}", line, col, len, token_type, modifiers);
        }

    }

}


