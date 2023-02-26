 # gen_input.py

def main():
    """Generate the input.txt file"""
    with open('input.txt', 'w', encoding='UTF-8') as input_file:
        for line in range(10_000_000):
            input_file.write(f"{line} {line}\n")
        input_file.close()

if __name__ == "__main__":
    main()
