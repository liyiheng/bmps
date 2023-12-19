# BMPS lib

```text
❯ cargo run --example normalize --  -h
   Compiling bmps v0.1.0 (/home/liyiheng/just_for_fun/photography/bmps/native/bmps)
    Finished dev [unoptimized + debuginfo] target(s) in 2.47s
     Running `/home/liyiheng/just_for_fun/photography/bmps/target/debug/examples/normalize -h`
Usage: normalize [OPTIONS]

Options:
  -w, --width [<WIDTH>]                    Width [default: 1920]
  -H, --height [<HEIGHT>]                  Height [default: 1080]
  -b, --blur [<BLUR>]                      Blur radius [default: 50]
  -r, --round [<ROUND>]                    Round radius [default: 45]
  -s, --shadow [<SHADOW>]                  Shadow width [default: 40]
      --shadow-offset-x <SHADOW_OFFSET_X>  [default: 30]
      --shadow-offset-y <SHADOW_OFFSET_Y>  [default: 30]
  -p, --padding [<PADDING>]                Range [0, 0.5) [default: 0.1]
  -i, --input [<INPUT_PATH>]               File or directory path [default: .]
  -o, --out [<OUTPUT>]                     Output path [default: .]
  -W, --white-bg                           White background
      --aspect-ratio                       If width and height stand for aspect ratio
  -h, --help                               Print help
```
Source file:

 <img src="./hello.jpg" width = "321" height = "214" alt="" align=center />


Landscape output:

 <img src="./output_p.jpg" width = "320" height = "180" alt="" align=center />

Portrait output:

 <img src="./output_l.jpg" width = "180" height = "320" alt="" align=center />
