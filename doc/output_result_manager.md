Managing session-time archiving for output folder, including an input folder and a result folder

# How to use

Provinding 'shortcut' managing of input folder, output folder history and result folder, using [`ResultEnv`] structure.

Output folder is organised during instance creation by creating subfolder named by running time formated name (__%Y-%m-%d_%Hh%Mm%S/__).

### Struct
 - [`ResultEnv`] :
    - [`ResultEnv::input_folder`] ([`PathBuf`]) : not used in feature, but se as shortcut
    - [`ResultEnv::output_folder`] ([`PathBuf`]) : new output instance as each ResultEnv instance
    - [`ResultEnv::result_folder`] ([`PathBuf`]) : used to export the last output if needed

### Tools
 - [`ResultEnv::get_input_folder`] : get the input folder
 - [`ResultEnv::get_output_folder`] : get the current output folder
 - [`ResultEnv::get_result_folder`] : get the result folder
 - [`ResultEnv::delete_if_empty`] : delete current output folder (for cleaning), if empty
 - [`ResultEnv::export_result`] : copy current output folder to result folder

# Minimal usecase

Minimal project : make statistic of data samples

```rust
mod output_result_manager;

fn main() {
    let file_env = output_result_manager::open_env("input_folder/", "main_output_folder/", "result_folder");

    // Create statistic from files (example)
    let mut statistic = StatisticResult::new();
    statistic.load_from_folder(file_env.get_input_folder());

    // Save statistic in sub_folder (ex : main_output_folder/2024-04-18_01h47m31/)
    if StatisticResult.is_success() {
        statistic.save_in_folder(file_env.get_output_folder());
        file_env.export_result();
    } else {
        file_env.delete_if_empty();
    }
    
}

/// Statistic content
struct StatisticResult {
    // ...
}

impl StatisticResult {
    fn new() -> Self {
        // ...
    }

    /// Treatment of all files of a input folder
    fn load_from_folder(&mut self, folder : PathBuf) {
        for entry in folder.read_dir().unwrap() {
            // Load each file
            self.load_from_file(entry.path());
        }
    }

    /// Treatment of one file
    fn load_from_file(&mut self, file : PathBuf) {
        // Make statistic from the file content
        // ...
    }

    /// Treatment of all file of a input folder
    fn save_in_folder(&self, folder : PathBuf) {
        // Write in a statistic.csv file, in the output folder
        // ...
    }
}
```

```text
.
├── src/
│   ├── main.rs
│   ├── output_result_manager.rs
│   └── ...
│
├── input_folder/
│   ├── sample_A.csv
│   ├── sample_B.csv
│   └── ...
│
├── main_output_folder/
│   ├── 2024-04-18_01h47m31/
│   │   ├── statistic.csv
│   │   └── log_2024-04-18_01h47m31.log
│   ├── 2024-04-19_01h34m01/
│   │   ├── statistic.csv
│   │   └── log_2024-04-19_01h34m01.log
│   ├── 2024-04-22_17h19m23/
│   │   ├── statistic.csv
│   │   └── log_2024-04-22_17h19m23.log
│   └── ...
│
├── result_folder/
│   ├── statistic.csv
│   └── log_2024-04-22_17h19m23.log
│
└── ...
```

# Panic and failure

No error or failure are provided by this module
