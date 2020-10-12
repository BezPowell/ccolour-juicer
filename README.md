# ccolour-juicer
![Rust](https://github.com/BezPowell/ccolour-juicer/workflows/Rust/badge.svg)
---
Simple cli utility to print out a list of unique colours in a given css file. Outputs each unique colour string organised by number of times it appears. Also indicates if, once computed, each colour is a duplicate of another (e.g. `white` would be listed as a duplicate of `#fff`).

## Usage example
`ccolour-juicer path/to/file.css`

## Example output
Running utility on `bootstrap.min.css` (current as of 2020-10-12) gives the following output
```
Colour                 | Count      | Duplicate 
#fff                   | 120        |           
#6c757d                | 39         |           
#007bff                | 34         |           
red                    | 34         |           
#212529                | 32         |           
#dee2e6                | 28         |           
#28a745                | 28         |           
#dc3545                | 28         |           
white                  | 21         | #fff      
#343a40                | 19         |           
#f8f9fa                | 18         |           
#ffc107                | 17         |           
#17a2b8                | 17         |           
#e9ecef                | 15         |           
#495057                | 14         |           
rgba(0,123,255,.25)    | 9          |           
#000                   | 9          |           
rgba(0,123,255,.5)     | 7          |           
#adb5bd                | 6          |           
#155724                | 5          |           
#1d2124                | 5          |           
#004085                | 5          |           
#117a8b                | 5          |           
#1b1e21                | 5          |           
#bd2130                | 5          |           
#856404                | 5          |           
#b3d7ff                | 5          |           
#dae0e5                | 5          |           
#d39e00                | 5          |           
rgba(0,0,0,.9)         | 5          |           
#383d41                | 5          |           
rgba(0,0,0,.075)       | 5          |           
#1e7e34                | 5          |           
#721c24                | 5          |           
#0062cc                | 5          |           
#0c5460                | 5          |           
#818182                | 5          |           
#545b62                | 5          |           
#0056b3                | 4          |           
#abdde5                | 4          |           
#b1dfbb                | 4          |           
rgba(40,167,69,.25)    | 4          |           
rgba(220,53,69,.25)    | 4          |           
#c8cbcf                | 4          |           
rgba(255,255,255,.5)   | 4          |           
rgba(0,0,0,.1)         | 4          |           
#b9bbbe                | 4          |           
rgba(0,0,0,.5)         | 4          |           
#ffe8a1                | 4          |           
#f1b0b7                | 4          |           
#80bdff                | 4          |           
rgba(0,0,0,.125)       | 4          |           
#ced4da                | 4          |           
#9fcdff                | 4          |           
rgba(0,0,0,.25)        | 4          |           
#ececf6                | 4          |           
#d6d8db                | 3          |           
rgba(40,167,69,.5)     | 3          |           
#f5c6cb                | 3          |           
rgba(108,117,125,.5)   | 3          |           
#bee5eb                | 3          |           
#b8daff                | 3          |           
rgba(23,162,184,.5)    | 3          |           
#c3e6cb                | 3          |           
#c6c8ca                | 3          |           
rgba(248,249,250,.5)   | 3          |           
#fdfdfe                | 3          |           
rgba(255,255,255,.15)  | 3          |           
rgba(255,193,7,.5)     | 3          |           
#ffeeba                | 3          |           
rgba(52,58,64,.5)      | 3          |           
rgba(220,53,69,.5)     | 3          |           
rgba(58,176,195,.5)    | 2          |           
#e0a800                | 2          |           
rgba(255,255,255,.85)  | 2          |           
#e4606d                | 2          |           
#e2e6ea                | 2          |           
rgba(216,217,219,.5)   | 2          |           
gray                   | 2          |           
#e83e8c                | 2          |           
rgba(0,0,0,.15)        | 2          |           
#c82333                | 2          |           
rgba(0,0,0,.03)        | 2          |           
rgba(130,138,145,.5)   | 2          |           
#5a6268                | 2          |           
rgba(0,0,0,.05)        | 2          |           
#454d55                | 2          |           
rgba(225,83,97,.5)     | 2          |           
rgba(38,143,255,.5)    | 2          |           
#34ce57                | 2          |           
#218838                | 2          |           
#f7f7f7                | 2          |           
rgba(0,0,0,.2)         | 2          |           
rgba(72,180,97,.5)     | 2          |           
rgba(222,170,12,.5)    | 2          |           
#138496                | 2          |           
rgba(82,88,93,.5)      | 2          |           
#0069d9                | 2          |           
#23272b                | 2          |           
#cce5ff                | 1          |           
#0b2e13                | 1          |           
#95999c                | 1          |           
#491217                | 1          |           
#fff3cd                | 1          |           
#19692c                | 1          |           
#1c7430                | 1          |           
#002752                | 1          |           
#6f42c1                | 1          |           
rgba(0,0,0,.175)       | 1          |           
#0f6674                | 1          |           
green                  | 1          |           
#b3b7bb                | 1          |           
#533f03                | 1          |           
#fd7e14                | 1          |           
#d1ecf1                | 1          |           
#86cfda                | 1          |           
rgba(255,255,255,.075) | 1          |           
purple                 | 1          |           
#171a1d                | 1          |           
#fefefe                | 1          |           
yellow                 | 1          |           
#ba8b00                | 1          |           
#8fd19e                | 1          |           
rgba(255,255,255,.05)  | 1          |           
#d3d9df                | 1          |           
#494f54                | 1          |           
rgba(40,167,69,.9)     | 1          |           
#686868                | 1          |           
#a71d2a                | 1          |           
#fbfcfc                | 1          |           
#20c997                | 1          |           
#fcf8e3                | 1          |           
pink                   | 1          |           
#d4edda                | 1          |           
#b21f2d                | 1          |           
rgba(0,0,0,.7)         | 1          |           
#cbd3da                | 1          |           
#7abaff                | 1          |           
#16181b                | 1          |           
blue                   | 1          |           
#005cbf                | 1          |           
rgba(255,255,255,.75)  | 1          |           
#121416                | 1          |           
rgba(255,255,255,.25)  | 1          |           
#4e555b                | 1          |           
#202326                | 1          |           
black                  | 1          | #000      
rgba(255,255,255,.1)   | 1          |           
#062c33                | 1          |           
#c69500                | 1          |           
#10707f                | 1          |           
#ebebeb                | 1          |           
#e2e3e5                | 1          |           
orange                 | 1          |           
rgba(220,53,69,.9)     | 1          |           
teal                   | 1          |           
cyan                   | 1          |           
#6610f2                | 1          |           
#040505                | 1          |           
#ffdf7e                | 1          |           
#f8d7da                | 1          |           
rgba(0,0,0,0)          | 1          |           
indigo                 | 1          |           
rgba(0,0,0,.3)         | 1          |           
#d6d8d9                | 1          |           
#ed969e                | 1          |
```

## Compilation
Requires `rust` and `cargo` to be installed. Just clone repository and run `cargo build --release` in root directory.

## Known limitations
1. Only supports CSS 3 syntax due to `css-color-parser2` not having support for CSS Color Module Level 4.
2. Currently panics on any invalid colour strings (e.g. `rgb(255,255,255,1)`) - proper error handling will be implemented at a later date.