# What could be improved?
## Add optional CLI `--verbose`
```rs
/// Whether to add links to navigation between titles
#[arg(long, default_value_t = false)]
verbose: bool,
```
If defined, the content of being generated title will be printed in console line by line

## Add optional CLI `--detect-existing-title`
```rs
/// Whether to detect existing title
#[arg(long, default_value_t = false)]
detect_existing_title: bool,
```
If defined, the program will try to detect and drop out from result file content already existing title

## Add optional CLI `--with-links`
```rs
/// Whether to add links to navigation between titles
#[arg(long, default_value_t = false)]
with_links: bool,
```
If defined, generated title will included the links to all headers