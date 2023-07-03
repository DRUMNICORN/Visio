import subprocess
import logging
import os
import time
import json
import shutil
from error_handling import generate_error_json
from file_handling import generate_files_json, analyze_files, save_files_to_json

# Set the configuration settings
LOG_FILE_NAME = "run.log"
ERRORS_JSON_FILE_NAME = "error.json"
FILES_JSON_FILE_NAME = "files.json"
AUTO_FOLDER = ".auto"

def get_timestamp():
    return int(time.time())

def execute_cargo_run():
    process = subprocess.run(["cargo", "run"], capture_output=True, text=True)
    return process.returncode, process.stdout, process.stderr

def save_log_file(timestamp, stdout, stderr):
    log_file_path = os.path.join(AUTO_FOLDER, LOG_FILE_NAME)
    with open(log_file_path, "w") as log_file:
        log_file.write(stdout)
        log_file.write(stderr)

def create_auto_folder():
    if not os.path.exists(AUTO_FOLDER):
        os.makedirs(AUTO_FOLDER)

def clear_auto_folder():
    # Delete the contents of the .auto folder
    for root, dirs, files in os.walk(AUTO_FOLDER):
        for file in files:
            file_path = os.path.join(root, file)
            os.remove(file_path)
        for dir in dirs:
            dir_path = os.path.join(root, dir)
            shutil.rmtree(dir_path)
def main():
    create_auto_folder()

    # Clear the .auto folder
    clear_auto_folder()

    # Execute Cargo
    timestamp = get_timestamp()
    return_code, stdout, stderr = execute_cargo_run()
    save_log_file(timestamp, stdout, stderr)

    # Gather Errors
    log_file_path = os.path.join(AUTO_FOLDER, LOG_FILE_NAME)
    errors_file_path = os.path.join(AUTO_FOLDER, ERRORS_JSON_FILE_NAME)
    generate_error_json(log_file_path, errors_file_path)

    # Read error files
    with open(errors_file_path, "r") as errors_file:
        error_files = json.load(errors_file)
        error_files = [os.path.join(AUTO_FOLDER, file['file_path']) for file in error_files]

    # Analyze Files
    files = analyze_files(AUTO_FOLDER, error_files)
    files_json_file_path = os.path.join(AUTO_FOLDER, FILES_JSON_FILE_NAME)
    save_files_to_json(files, files_json_file_path)

if __name__ == "__main__":
    main()
