import rust_core

def main():
    # Call the Rust `greet` function
    greeting = rust_core.greet("Python")
    print(greeting)  # Output: Hello, Python!

    # Call the Rust `add_numbers` function
    result = rust_core.add_numbers(3, 5)
    print(f"3 + 5 = {result}")  # Output: 3 + 5 = 8

    result = rust_core.http()
    print(result)

if __name__ == "__main__":
    main()
