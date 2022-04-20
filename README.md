# td ![logo](https://github.com/Jinmaro/td/blob/main/IMG_0334.PNG)
Command Line Tool to use Your To Do 


## Usage
 * Example
 ```
  $ td
 >>> 1:o, xxxx
 	2:o, yyyy
 $ td -a zzzz
 >>> 1:o, xxxx
 	2:o, yyyy
 	3:o, zzzz
 $ td -f xxxx
 >>> 1:x, xxxx
 	2:o, yyyy
 	3:o, zzzz
 $ td -d yyyy
 >>> 1:x, xxxx
 >>> 2:o, zzzz
 $ td -e zzzz aaaa
 >>> 1:x, xxxx
 	2:o, aaaa
 ```
