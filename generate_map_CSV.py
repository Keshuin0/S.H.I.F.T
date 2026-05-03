import os
import csv

# Settings
target_folder = '.' # The folder to scan
output_file = 'folder_inventory.csv'

def generate_csv():
    with open(output_file, 'w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        # Short headers to save tokens
        writer.writerow(['File', 'Folder', 'Size_KB'])

        for root, dirs, files in os.walk(target_folder):
            # Use relpath to keep folder strings short
            rel_path = os.path.relpath(root, target_folder)
            if rel_path == '.': rel_path = '/'
            
            for name in files:
                try:
                    file_path = os.path.join(root, name)
                    # Rounding to 1 decimal place saves characters
                    size_kb = round(os.path.getsize(file_path) / 1024, 1)
                    writer.writerow([name, rel_path, size_kb])
                except OSError:
                    continue # Skip system-locked files

    print(f"Done! {output_file} created.")

if __name__ == "__main__":
    generate_csv()
