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

 * Example
```shell
    	td
		1:o, xxxx
		2:o, yyyy
	td -a zzzz
		1:o, xxxx
		2:o, yyyy
		3:o, zzzz
	td -f xxxx
		1:x, xxxx
		2:o, yyyy
		3:o, zzzz
	td -d yyyy
		1:x, xxxx
		2:o, zzzz
	td -e zzzz aaaa
		1:x, xxxx
		2:o, aaaa
```

## 🖥️ Sample Output
```
$ td
1:o, Homework
2:o, Running
```
