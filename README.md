# wikitable-map

A CLI tool for turning a CSV file of a map into wikitable markup. This can be
used to generate [game wiki tables of the style found here](https://dofuswiki.fandom.com/wiki/Walnut_(tree)).

## Installation

Clone the repo, then:
`cargo install --path .`

## Usage

see `wikitable-map --help` for usage instructions

## Example

*input.csv*
```CSV
,6,7,8,9,10,11
28,x,x,w,,,
29,3s,,1s,,,
30,5s,9s,5s,4s,,
31,,,,w,w,w
```

*CLI invocation*
cat input.csv | wikitable-map --sprite "Walnut_Wood.png" --sprite-alt "Walnut" --table-attrs 'class="wikitable" style="text-align: center;"' > table.txt

*table.txt*
```text
{| class="wikitable" style="text-align: center;"
! style="width: 40px; height: 40px" |
! style="width: 40px;" |6
! style="width: 40px;" |7
! style="width: 40px;" |8
! style="width: 40px;" |9
! style="width: 40px;" |10
! style="width: 40px;" |11
|-
! style="height: 40px;" |28
| style="background:#e6caa2;" |
| style="background:#e6caa2;" |
| style="background:#b2ccd3;" |
|
|
|
|-
! style="height: 40px;" |29
| 3[[File:Walnut_Wood.png|15px|Walnut]]
|
| 1[[File:Walnut_Wood.png|15px|Walnut]]
|
|
|
|-
! style="height: 40px;" |30
| 5[[File:Walnut_Wood.png|15px|Walnut]]
| 9[[File:Walnut_Wood.png|15px|Walnut]]
| 5[[File:Walnut_Wood.png|15px|Walnut]]
| 4[[File:Walnut_Wood.png|15px|Walnut]]
|
|
|-
! style="height: 40px;" |31
|
|
|
| style="background:#b2ccd3;" |
| style="background:#b2ccd3;" |
| style="background:#b2ccd3;" |
|}
```
