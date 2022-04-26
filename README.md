# ✔️ `td` ⌘
Command Line Tool to manage your To Do 


![logo](https://github.com/Jinmaro/td/blob/main/IMG_0334.PNG)

## ❓ What `td` ⌘ does
* You can manage your To Do with `td`
	- check your To Do
	- edit your To Do
	- add your To Do
	- delete your To Do

## ⌨️ Usage
```
td [OPTIONS]
OPTIONS
    -a, --add <NEW_TO_DO>
    -f, --finish <FINISHED_TO_DO>
    -d, --delete <DELETED_TO_DO>
    -e, --edit <EDITED_TO_DO> <NEW_TO_DO>
    -h, --help 
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
