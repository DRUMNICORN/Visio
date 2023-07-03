import os
import re
import coloredlogs
import time
import logging
import json

class Error:
    def __init__(self, message, timestamp):
        self.message = message
        self.timestamp = timestamp
        self.snippets = []

    def add_snippet(self, file_path, line_number, column_number, code_snippet,hints=None, notes=None):
        snippet_lines = [line for line in code_snippet.split("\n") if line.strip() and line.strip() != "|"]
        snippet = {
            "file_path": file_path,
            "line_number": line_number,
            "column_number": column_number,
            "code_snippet": snippet_lines,
            "hints": hints or [],
            "notes": notes or []
        }
        self.snippets.append(snippet)

    def serialize(self):
        serialized_error = {
            "message": self.message,
            "timestamp": self.timestamp,
            "snippets": self.snippets
        }
        return serialized_error


def analyze_log_file(log_file_path, timestamp, logger):
    errors = []
    error_pattern = re.compile(r"error(\[.*\])?:")
    snippet_pattern = re.compile(r"\s+--> (.*):(\d+):(\d+)")
    code_snippet_pattern = re.compile(r"\s+([^\n]+)")
    notes_pattern = re.compile(r"\s+(= )?note: (.+)")
    hints_pattern = re.compile(r"\s+= help: (.+)")
    cleanup_pattern = re.compile(r"\s+\|+|-+|\^+|_+")
    with open(log_file_path, "r") as log_file:
        current_error = None
        current_snippet_lines = []
        for line in log_file:
            if error_pattern.search(line):
                logger.error(line.strip())
                if current_error:
                    errors.append(current_error)
                current_error = Error(line.strip(), timestamp)
            else:
                snippet_match = snippet_pattern.search(line)
                if snippet_match and current_error:
                    file_path = snippet_match.group(1)
                    line_number = int(snippet_match.group(2))
                    column_number = int(snippet_match.group(3))
                    current_error.add_snippet(file_path, line_number, column_number, "".join(current_snippet_lines))
                    current_snippet_lines = []
                elif current_error:
                    code_snippet_match = code_snippet_pattern.search(line)
                    if code_snippet_match:
                        notes_match = notes_pattern.search(line)
                        hints_match = hints_pattern.search(line)
                        if notes_match:
                            current_error.snippets[-1]["notes"].append(notes_match.group(2))
                        elif hints_match:
                            current_error.snippets[-1]["hints"].append(hints_match.group(1))
                        else:
                            code_snippet = code_snippet_match.group(1)
                            cleaned_line = cleanup_pattern.sub("", code_snippet)
                            current_snippet_lines.append(cleaned_line + "\n")
        if current_error:
            errors.append(current_error)
    return errors


def save_errors_to_json(errors, json_file_path):
    error_data = []
    for error in errors:
        error_data.append(error.serialize())
    with open(json_file_path, "w") as json_file:
        json.dump(error_data, json_file, indent=4)


def generate_error_json(log_file_path, json_file_path):
    timestamp = int(time.time())

    logger = logging.getLogger("error_logger")
    coloredlogs.install(level="ERROR", logger=logger)

    errors = analyze_log_file(log_file_path, timestamp, logger)
    save_errors_to_json(errors, json_file_path)
