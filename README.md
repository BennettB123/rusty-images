# rusty-images

rusty-images is a command-line tool written in rust to view images in a terminal. The output is displayed with the following Unicode shading characters: " ░▒▓█".

## How to

rusty-images supports PNG, JPEG, BMP, PNM and several other image formats. 
The tool requires the first command-line parameter to be a file path to an image. The second parameter is the output's width in characters. The third is its height. Width and height are both optional with a default value of 50.

### Example

<img src="docs/images/rust_logo.jpg" width="200px" height="200px"/>

Given the above image with a file path of `./docs/images/rust_logo.png`, Running the following command will produce the following output.

``` sh
rusty-images ./docs/images/rust_logo.png 40 20
```

result:
```
███████████████▓░██░░██░▓███████████████
█████████▓██  ░          ░  ██▓█████████
████████▓          ██          ▓████████
██████      ░▒▓███▒  ▒███▓▒░      ██████
███▓░▒    ▒██████████████████▒    ▒░▓███
████                      ░▒███▒    ████
██                           ▒██▒     ██
█▓▒ ░█▒ ░▓░      ▓▓▓▓▓▒░      █▓ ▒█░ ▒▓█
█░   ░░▒██▒      ▓▓▓▓▓▓░     ░██▒░░   ░█
▓▒   ▒████▒                 ▓█████▒   ▒▓
▓▒   ▒████▒      ░░░░       ░████▒░   ▒▓
█░   ░████▒      █████▓      ▒██▒     ░█
█▓▒                 ▓██░             ▒▓█
██                  ▓███              ██
████     ▒▒▒▒▓▓▓▓▓▓▓█████▓▓▒▒▒▒     ████
███▓░▒    ░▒ ▒████████████▒ ▒░    ▒░▓███
██████    ▒▓  ▓██████████▓  ▓▒    ██████
████████▓         ░░░░         ▓████████
█████████▓██  ░          ░  ██▓█████████
███████████████▓░██░░██░▓███████████████
```
