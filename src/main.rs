use clap::{arg, Command};
use std::process::Command as ShellCommand;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn deploy_contract(
    salt: usize,
    keystore_password: &str,
    is_found: Arc<AtomicBool>,
    class_hash: &str,
    constructor_args: &str,
) {
    let cmd = format!(
        "starkli deploy --simulate \
    --salt {salt} \
    --keystore-password {keystore_password} {class_hash} {constructor_args}",
        salt = salt,
        keystore_password = keystore_password,
        class_hash = class_hash,
        constructor_args = constructor_args
    );

    let output = ShellCommand::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    let stderr = str::from_utf8(&output.stderr).unwrap_or("");
    if stderr.contains("The contract will be deployed at address") {
        let address = stderr.split_whitespace().last().unwrap_or("");
        println!("Salt: {}, address: {}", salt, address);

        if address.starts_with("0x04515") {
            println!("Found matching address: {} with salt: {}", address, salt);
            is_found.store(true, Ordering::Relaxed);
            return;
        }
    }
}

fn main() {
    let matches = Command::new("My App")
        .version("1.0")
        .about("Deploys contracts")
        .arg(arg!(-p --"keystore-password" <KEYSTORE_PASSWORD> "The password for the keystore")
             .required(true))
        .arg(arg!(-c --"class-hash" <CLASS_HASH> "The class hash of your contract to deploy")
             .required(true))
        .arg(arg!(-t --"threads" [THREADS] "The number of threads to use")
             .default_value("8")
             .value_parser(clap::value_parser!(usize)))
        .arg(arg!(-a --"constructor-args" <CONSTRUCTOR_ARGS> "The constructor arguments for your contract to deploy"))
        .get_matches();

    let keystore_password = matches
        .get_one::<String>("keystore-password")
        .expect("Password is required")
        .clone();

    let class_hash = matches
        .get_one::<String>("class-hash")
        .expect("Class hash is required")
        .clone();

    let constructor_args: String = matches
        .get_one::<String>("constructor-args")
        .expect("Constructor args are required")
        .clone();

    let num_threads: usize = *matches.get_one("threads").unwrap();
    println!("Using {} threads", num_threads);

    let is_found = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    for i in 0..num_threads {
        let is_found_clone = Arc::clone(&is_found);
        let keystore_password = keystore_password.clone();
        let class_hash = class_hash.clone();
        let constructor_args = constructor_args.clone();

        handles.push(thread::spawn(move || {
            let mut salt = i;
            while !is_found_clone.load(Ordering::Relaxed) {
                deploy_contract(
                    salt,
                    &keystore_password,
                    Arc::clone(&is_found_clone),
                    &class_hash,
                    &constructor_args,
                );
                salt += num_threads;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
