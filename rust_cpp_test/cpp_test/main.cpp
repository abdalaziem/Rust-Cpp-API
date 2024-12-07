#include <iostream>
#include <cstring>
#include <dlfcn.h> 

extern "C" {
    char* search_trouble_codes(const char* file_path, const char* search_query);
    void free_result(char* result);
}

int main() {
    // Path to the JSON file
    const char* file_path = "/home/mohamed/rust_cpp_test/obd-trouble-codes.json";
    
    // Search query
    const char* search_query = "U0401";

    // Call the Rust function
    char* result = search_trouble_codes(file_path, search_query);
    
    if (result != nullptr) {
        std::cout << result << std::endl;

        // Free the result memory
        free_result(result);
    } else {
        std::cerr << "Error: Failed to get results" << std::endl;
    }

    return 0;
}
