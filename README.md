# png-texter
A CLI tool to hide encrypted messages in a .png file written in rust
   
## Usage 👾

```
A simple png chunk message encoder/decoder

Usage: png-texter [OPTIONS] [COMMAND]

Commands:
  encode  encode the png with a secret message
  decode  decode the png to get the secret message
  remove  remove the chunk from the png
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --chunk-type <CHUNK_TYPE>  Sets the chunk type to use
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples 🔬

![image](https://user-images.githubusercontent.com/65186979/216777155-6683d182-97c2-45d5-8635-0ea5d19a815f.png)


### Encode

      ~/png-texter -c "rust" encode -o "png_tests/encoded.png"  png_tests/example.png "Hello User!"

### Decode

      ~/png-texter -c "rust" decode png_tests/encoded.png
      
### Remove
      
      ~/png-texter remove png_tests/encoded.png


#### If you find a bug or have an idea for a new feature, please open an issue or submit a pull request.
