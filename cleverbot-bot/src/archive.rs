

fn get_clevereq_path() -> PathBuf {
    let mut adaptable_path = PathBuf::from(".");
    adaptable_path.push("src");
    adaptable_path.push("static");

    if cfg!(windows) {
        adaptable_path.push("clevreq.exe");
    } else {
        adaptable_path.push("clevreq");
    };

    adaptable_path
}

fn get_static_file_named(name: &str) -> String {
    let mut adaptable_path = PathBuf::from(".");
    adaptable_path.push("src");
    adaptable_path.push("static");
    adaptable_path.push(name);

    let mut log = String::new();
    
    log.push_str("```");

    if !Path::new(&adaptable_path).exists() {
        log.push_str(&format!("1. Executable not found at: {:?}\n", adaptable_path));
    }

    let metadata = fs::metadata(&adaptable_path);

    match metadata {
        Ok(meta) => {
            log.push_str(&format!("Metadata: {:?}\n", meta));

            if meta.is_file() {
                log.push_str(&format!("The specified path IS a file: {:?}\n", adaptable_path));
            } else {
                log.push_str(&format!("The specified path is not a file: {:?}\n", adaptable_path));
            }
        },
        Err(err) => {
            log.push_str(&format!("Failed to read executable metadata: {}\n", err));
        }
    };

    log.push_str("```");

    log
}

fn get_cleverbot_response(input: &str, exe_path: PathBuf) -> Result<String, String> {
    // Define the command to run the Python script
    let cookie = "i";
    let payload = "i";

    info!("Path to executable: {:?}", exe_path);

    let output = Command::new(exe_path)
        .arg("--cookie")
        .arg(cookie)
        .arg("--payload")
        .arg(payload)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                return Ok(stdout.to_string());
            } else {
                return Err(stderr.to_string());
            }
        },
        Err(err) => {
            return Err(format!("Failed to execute process: {}", err));
        }
    }
}
