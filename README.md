# json_rust_parser
JSON parser on rust for education purpose

## Overview

This Rust project, named "my_parser_sofia" is designed to parse JSON files into a structured format. The parser, implemented using the Pest library, provides a `JSONValue` enum to represent various JSON data types, including objects, arrays, strings, numbers, booleans, and null values.

## Parsing Process

The parsing process involves tokenizing the input JSON file based on the specified grammar rules. The parser identifies JSON objects, arrays, strings, numbers, booleans, and null values, creating a structured representation of the JSON data.

## Usage

The parsed JSON values can be utilized in Rust applications for various purposes, such as data manipulation, validation, or integration with other systems. Additionally, the project includes a serialization function (`serialize_jsonvalue`) to convert the parsed JSON values back into a JSON string.

- Become help information
    Example:
    ```shell
    .\target\debug\json_rust_parser.exe --help
    ```

- Run the program using standart files as parameters 
    Example:
    ```shell
    .\target\debug\json_rust_parser.exe --input-file data.json --output-file data_out.json
    ```
