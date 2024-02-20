use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::env;
use std::path::PathBuf;

fn main() {
   println!("Updated");
   let current_dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(err) => {
         println!("Failed to get current directory: {}", err);
         return;
      }
   };

   let pre_commit_path = current_dir.join("pre-commit");

    // Change the permissions of the pre-commit file
    let perm_change = Command::new("chmod")
        .arg("+x")
        .arg(&pre_commit_path)
        .output();

    match perm_change {
        Ok(output) => {
            if output.status.success() {
                println!("Permissions changed successfully");
            } else {
                println!(
                    "Failed to change permissions of pre-commit file: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return;
            }
        }
        Err(err) => {
            println!("Failed to execute chmod: {}", err);
            return;
        }
    }


   let mut pre_commit_file = match OpenOptions::new().append(true).open(pre_commit_path) {
      Ok(file) => file,
      Err(err) => {
         println!("Failed to open pre-commit file: {}", err);
         exit(1);
      }
   };

   // Append the multi-line string to the pre-commit file
   let multi_line_string = "# Navigate to the root directory of the Git repository\n
                           cd \"$(git rev-parse --show-toplevel)\"\n
                           
                           # Run pipreqs\n
                           pipreqs ./ --force\n";
   match writeln!(pre_commit_file, "{}", multi_line_string) {
      Ok(_) => println!("Multi-line string appended to pre-commit file"),
      Err(err) => println!("Failed to append multi-line string to pre-commit file: {}", err),
   }

   // Check if pipreqs is installed
   let pipreqs_check = Command::new("pipreqs").arg("--version").output();

   match pipreqs_check {
      Ok(output) => {
         if !output.status.success() {
            // pipreqs is not installed, install it using pip
            let pip_install = Command::new("pip")
               .arg("install")
               .arg("pipreqs")
               .output();

            match pip_install {
               Ok(_) => println!("pipreqs installed successfully"),
               Err(err) => println!("Failed to install pipreqs: {}", err),
            }
         } else {
            println!("pipreqs is already installed");
         }
      }
      Err(err) => println!("Failed to check pipreqs installation: {}", err),
   }
}

