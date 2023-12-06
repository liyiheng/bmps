# BMPS lib

```sh
cargo run --example normalize -- -h
Usage: normalize [OPTIONS]
```
```text
Options:
  -w, --width [<WIDTH>]       Width [default: 1920]
  -H, --height [<HEIGHT>]     Height [default: 1080]
  -b, --blur [<BLUR>]         Blur radius [default: 50]
  -r, --round [<ROUND>]       Round radius [default: 45]
  -s, --shadow [<SHADOW>]     Shadow width [default: 40]
  -p, --padding [<PADDING>]   Range [0, 0.5) [default: 0.1]
  -i, --input [<INPUT_PATH>]  File or directory path [default: .]
  -o, --out [<OUTPUT>]        Output path [default: .]
  -h, --help                  Print help
```
Source file:

 <img src="./hello.jpg" width = "320" height = "180" alt="" align=center />


Landscape output:

 <img src="./output_p.jpg" width = "320" height = "180" alt="" align=center />

Portrait output:

 <img src="./output_l.jpg" width = "180" height = "320" alt="" align=center />
