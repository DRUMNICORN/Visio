import os
import re
import coloredlogs
import time
import logging
import json
from pathlib import Path

class FileData:
    def __init__(self, file_path, content, links=None, functions=None, structs=None, traits=None):
        self.file_path = file_path
        self.content = content
        self.links = links or []
        self.functions = functions or []
        self.structs = structs or []
        self.traits = traits or []

    def add_link(self, relation):
        if relation not in self.links:
            self.links.append(relation)
            self.links.sort()

    def add_function(self, function):
        if function not in self.functions:
            self.functions.append(function)
            self.functions.sort()

    def add_struct(self, struct):
        if struct not in self.structs:
            self.structs.append(struct)
            self.structs.sort()

    def add_trait(self, trait):
        if trait not in self.traits:
            self.traits.append(trait)
            self.traits.sort()

    def serialize(self):
        serialized_file_data = {
            "file_path": self.file_path,
            "content": self.content,
            "links": self.links,
            "functions": self.functions,
            "structs": self.structs,
            "traits": self.traits
        }
        return serialized_file_data


def analyze_folder(folder):
    files = []

    # Analyze files in the specified folder using Path
    for file_path in Path(folder).rglob("*"):
        if file_path.is_file():
            with open(file_path, "r") as file:
                content = file.read()

            content = remove_comments_and_empty_lines(content)
            if content:
                file_data = FileData(str(file_path), content.split("\n"))
                file_data.functions = list(set(extract_functions(content)))
                file_data.structs = list(set(extract_structs(content)))
                file_data.traits = list(set(extract_traits(content)))
                file_data.links = list(set(extract_links(content)))
                file_data.functions.sort()
                file_data.structs.sort()
                file_data.traits.sort()
                file_data.links.sort()
                files.append(file_data)
    
    return files


def extract_functions(content):
    # Example implementation to extract functions
    function_pattern = r"fn\s+(\w+)\s*[(]"
    functions = re.findall(function_pattern, content)
    return functions

def extract_structs(content):
    struct_pattern = r"(?:pub\s+)?struct\s+(\w+)\s*[{]?[^}]*[}]?"
    structs = re.findall(struct_pattern, content, re.MULTILINE | re.DOTALL)
    return structs

def extract_traits(content):
    trait_pattern = r"(?:pub\s+)?trait\s+(\w+)\s*[{]"
    traits = re.findall(trait_pattern, content)
    return traits

def extract_links(content):
    link_pattern = r"use\s+([\w:]+);"
    links = re.findall(link_pattern, content)
    return links
    
def extract_functions_from_tree(tree):
    functions = []

    cursor = tree.walk()

    def traverse_tree(cursor):
        nonlocal functions
        tree_node = cursor.node

        if tree_node.type == 'function_declaration' or tree_node.type == 'function_definition':
            function_name_node = tree_node.child_by_field_name('name')
            if function_name_node:
                functions.append(function_name_node.text)

        if cursor.goto_first_child():
            traverse_tree(cursor)
            cursor.goto_parent()

        while cursor.goto_next_sibling():
            traverse_tree(cursor)

    traverse_tree(cursor)
    return functions

def analyze_files(folder, error_files):
    files = []

    for file_path in error_files:
        if os.path.isfile(file_path):
            with open(file_path, "r") as file:
                content = file.read()

            content = remove_comments_and_empty_lines(content)
            if content:
                file_data = FileData(str(file_path), content.split("\n"))
                file_data.functions = list(set(extract_functions(content)))
                file_data.structs = list(set(extract_structs(content)))
                file_data.traits = list(set(extract_traits(content)))
                file_data.links = list(set(extract_links(content)))
                file_data.functions.sort()
                file_data.structs.sort()
                file_data.traits.sort()
                file_data.links.sort()
                files.append(file_data)

    return files


def remove_comments_and_empty_lines(content):
    lines = content.split("\n")
    processed_lines = []

    for line in lines:
        # Remove trailing whitespace
        line = line.rstrip()

        # Remove single-line comments
        if line.startswith("//"):
            continue

        # Remove comments at the end of lines
        if "//" in line:
            line = line[:line.index("//")].rstrip()

        # Skip empty lines
        if not line:
            continue

        processed_lines.append(line)

    processed_content = "\n".join(processed_lines)
    return processed_content


def save_files_to_json(files, json_file_path):
    file_data = []
    for file in files:
        file_data.append(file.serialize())
    with open(json_file_path, "w") as json_file:
        json.dump(file_data, json_file, indent=4)

import os
import re
import coloredlogs
import time
import logging
import json
from pathlib import Path

class FileData:
    def __init__(self, file_path, content, links=None, functions=None, structs=None, traits=None):
        self.file_path = file_path
        self.content = content
        self.links = links or []
        self.functions = functions or []
        self.structs = structs or []
        self.traits = traits or []

    def add_link(self, relation):
        if relation not in self.links:
            self.links.append(relation)
            self.links.sort()

    def add_function(self, function):
        if function not in self.functions:
            self.functions.append(function)
            self.functions.sort()

    def add_struct(self, struct):
        if struct not in self.structs:
            self.structs.append(struct)
            self.structs.sort()

    def add_trait(self, trait):
        if trait not in self.traits:
            self.traits.append(trait)
            self.traits.sort()

    def serialize(self):
        serialized_file_data = {
            "file_path": self.file_path,
            "content": self.content,
            "links": self.links,
            "functions": self.functions,
            "structs": self.structs,
            "traits": self.traits
        }
        return serialized_file_data


def analyze_folder(folder):
    files = []

    # Analyze files in the specified folder using Path
    for file_path in Path(folder).rglob("*"):
        if file_path.is_file():
            with open(file_path, "r") as file:
                content = file.read()

            content = remove_comments_and_empty_lines(content)
            if content:
                file_data = FileData(str(file_path), content.split("\n"))
                file_data.functions = list(set(extract_functions(content)))
                file_data.structs = list(set(extract_structs(content)))
                file_data.traits = list(set(extract_traits(content)))
                file_data.links = list(set(extract_links(content)))
                file_data.functions.sort()
                file_data.structs.sort()
                file_data.traits.sort()
                file_data.links.sort()
                files.append(file_data)
    
    return files

def analyze_files(folder):
    files = []
    local_files = analyze_folder(folder)
    return local_files

def generate_files_json(files_path, json_file_path):
    timestamp = int(time.time())

    logger = logging.getLogger("file_logger")
    coloredlogs.install(level="ERROR", logger=logger)

    files = analyze_files(files_path)
    save_files_to_json(files, json_file_path)


def extract_functions(content):
    # Example implementation to extract functions
    function_pattern = r"fn\s+(\w+)\s*[(]"
    functions = re.findall(function_pattern, content)
    return functions

def extract_structs(content):
    struct_pattern = r"(?:pub\s+)?struct\s+(\w+)\s*[{]?[^}]*[}]?"
    structs = re.findall(struct_pattern, content, re.MULTILINE | re.DOTALL)
    return structs

def extract_traits(content):
    trait_pattern = r"(?:pub\s+)?trait\s+(\w+)\s*[{]"
    traits = re.findall(trait_pattern, content)
    return traits

def extract_links(content):
    link_pattern = r"use\s+([\w:]+);"
    links = re.findall(link_pattern, content)
    return links
    
def extract_functions_from_tree(tree):
    functions = []

    cursor = tree.walk()

    def traverse_tree(cursor):
        nonlocal functions
        tree_node = cursor.node

        if tree_node.type == 'function_declaration' or tree_node.type == 'function_definition':
            function_name_node = tree_node.child_by_field_name('name')
            if function_name_node:
                functions.append(function_name_node.text)

        if cursor.goto_first_child():
            traverse_tree(cursor)
            cursor.goto_parent()

        while cursor.goto_next_sibling():
            traverse_tree(cursor)

    traverse_tree(cursor)
    return functions

def remove_comments_and_empty_lines(content):
    lines = content.split("\n")
    processed_lines = []

    for line in lines:
        # Remove trailing whitespace
        line = line.rstrip()

        # Remove single-line comments
        if line.startswith("//"):
            continue

        # Remove comments at the end of lines
        if "//" in line:
            line = line[:line.index("//")].rstrip()

        # Skip empty lines
        if not line:
            continue

        processed_lines.append(line)

    processed_content = "\n".join(processed_lines)
    return processed_content


def save_files_to_json(files, json_file_path):
    file_data = []
    for file in files:
        file_data.append(file.serialize())
    with open(json_file_path, "w") as json_file:
        json.dump(file_data, json_file, indent=4)
