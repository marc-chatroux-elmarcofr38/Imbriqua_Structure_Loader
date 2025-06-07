Managing a single input folder and multiple output folder (session-time archiving)

# How to use

Provinding 'shortcut' managing of input folder and output folder, using [`FileEnv`] structure.

Output folder is organised during instance creation by creating subfolder named by running time formated name (%Y-%m-%d_%Hh%Mm%S/).

## Minimal usecase

Minimal project : make statistic of data samples

```rust
mod module_file_env;

fn main() {
    let file_env = module_file_env::open_env("input_folder/", "main_output_folder/");

    // Create statistic from files
    let mut statistic = StatisticResult::neew();
    statistic.load_from_folder(file_env.get_input_folder());

    // Save statistic in sub_folder (ex : main_output_folder/2024-04-18_01h47m31/)
    statistic.save_in_folder(file_env.get_output_folder());
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
        for entry in folder.read_dir().unrap() {
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
│   ├── module_file_env.rs
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
│   │   └── history_1.log
│   ├── 2024-04-19_01h34m01/
│   │   ├── statistic.csv
│   │   └── history_1.log
│   ├── 2024-04-22_17h19m23/
│   │   ├── statistic.csv
│   │   └── history_1.log
│   └── ...
│
└── ...
```

# Panic and failure

No error or failure are provided by this module
