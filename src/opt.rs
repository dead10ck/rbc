use std::str::FromStr;

use counter::Counter;

#[derive(StructOpt, Debug)]
#[structopt(about = "Counts things in strings.")]
pub struct Opt {
    /// Counts the grapheme clusters
    #[structopt(short = "c", long = "grapheme-clusters")]
    pub grapheme_clusters: bool,

    /// Counts the number of bytes
    #[structopt(short = "b", long = "bytes")]
    pub bytes: bool,

    /// Counts the number of lines
    #[structopt(short = "l", long = "lines")]
    pub lines: bool,

    /// Counts the number of words
    #[structopt(short = "w", long = "words")]
    pub words: bool,

    /// Don't print the counter header
    #[structopt(short = "n", long = "no-header")]
    pub no_header: bool,

    /// Counts the number of words
    #[structopt(short = "m", long = "mode", default_value = "file",
                help = "The format checker to use.",
                possible_values_raw = "&[\"file\", \"f\", \"line\", \"l\"]")]
    pub mode: CountMode,

    /// Sets the input file(s) to use. "-" gets treated as stdin.
    #[structopt(default_value = "-")]
    pub files: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, StructOpt)]
pub enum CountMode {
    /// Performs counts for every file.
    File,

    /// Performs counts for every line.
    Line,
}

impl FromStr for CountMode {
    type Err = String;

    fn from_str(s: &str) -> Result<CountMode, String> {
        match s {
            "file" | "f" => Ok(CountMode::File),
            "line" | "l" => Ok(CountMode::Line),
            _ => Err(format!("Unknown count mode: {}", s)),
        }
    }
}

impl Opt {
    /// Gets the [`Counter`]s from the CLI options.
    pub fn get_counters(&self) -> Vec<Counter> {
        let mut counters = Vec::new();

        if self.grapheme_clusters {
            counters.push(Counter::GraphemeCluster);
        }

        if self.bytes {
            counters.push(Counter::NumByte);
        }

        if self.lines {
            counters.push(Counter::Line);
        }

        if self.words {
            counters.push(Counter::Words);
        }

        // pick some defaults if the user doesn't specify any counters
        if counters.is_empty() {
            counters.push(Counter::Line);
            counters.push(Counter::Words);
            counters.push(Counter::NumByte);
        }

        counters
    }
}
