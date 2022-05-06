# ✔️ `td` ⌘
Command Line Tool to manage your To Do

[![build](https://github.com/Jinmaro/td/actions/workflows/build.yaml/badge.svg)](https://github.com/Jinmaro/td/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/Jinmaro/td/badge.svg)](https://coveralls.io/github/Jinmaro/td)

![logo](https://github.com/Jinmaro/td/blob/main/IMG_0334.PNG)


## ❓ What `td` ⌘ does
* You can manage your To Do with `td`
	- check your To Do
	- edit your To Do
	- add your To Do
	- delete your To Do

## ⌨️ Usage
```
Command Line Tool to manage your To Do.

USAGE:
    td [OPTIONS] <TODO>

ARGS:
    <TODO>    

OPTIONS:
    -a, --add <NEW_TO_DO>            add NEW_TO_DO to your To Do list.
    -d, --delete <DELETED_TO_DO>     delete DELETED_TO_DO from your To Do list.
    -e, --edit <EDITED_TO_DO>        change EDITED_TO_DO of your To Do list to NEW_TO_DO.
    -f, --finish <FINISHED_TO_DO>    put a check mark to FINISHED_TO_DO of your To Do list.
    -h, --help                       Print help information
    -V, --version                    Print version information
```

## 🖥️ Sample Output
```
$ td
o: Homework
o: Running

$ td -a Buy_2_tomatos
o: Homework
o: Running
o: Buy_2_tomatos

$ td -f Running
o: Homework
x: Running
o: Buy_2_tomatos

$ td -d Homework
x: Running
o: Buy_2_tomatos

$ td -e Buy_2_tomatos Buy_3_tomatos
x: Running
o: Buy_3_tomatos
```
