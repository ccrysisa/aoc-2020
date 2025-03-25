# Advent of Code 2020

## Environment

```console
$ fastfetch 
             ............                 ccrysisa@ccrysisa-PC
         .';;;;;.       .,;,.             ----------                                                        
      .,;;;;;;;.       ';;;;;;;.          OS: Deepin beige 23 x86_64                                        
    .;::::::::'     .,::;;,''''',.        Host: RedmiBook 14 II                                             
   ,'.::::::::    .;;'.          ';       Kernel: Linux 6.12.9-amd64-desktop-rolling                        
  ;'  'cccccc,   ,' :: '..        .:      Uptime: 2 hours, 30 mins                                          
 ,,    :ccccc.  ;: .c, '' :.       ,;     Packages: 2283 (dpkg), 7 (linglong)                               
.l.     cllll' ., .lc  :; .l'       l.    Shell: bash 5.2.21                                                
.c       :lllc  ;cl:  .l' .ll.      :'    Display (BOE08EE): 1920x1080 @ 60 Hz in 14″ [Built-in]            
.l        'looc. .   ,o:  'oo'      c,    DE: DDE                                                           
.o.         .:ool::coc'  .ooo'      o.    WM: KWin (X11)                                                    
 ::            .....   .;dddo      ;c     Theme: deepin [GTK2]                                              
  l:...            .';lddddo.     ,o      Icons: Reversal-blue [GTK2]                                       
   lxxxxxdoolllodxxxxxxxxxc      :l       Cursor: bloom                                                     
    ,dxxxxxxxxxxxxxxxxxxl.     'o,        Terminal: deepin-terminal 6.0.16                                  
      ,dkkkkkkkkkkkkko;.    .;o;          Terminal Font: Noto Sans Mono (11pt)                              
        .;okkkkkdl;.    .,cl:.            CPU: Intel(R) Core(TM) i7-1065G7 (8) @ 3.90 GHz                   
            .,:cccccccc:,.                GPU 1: NVIDIA GeForce MX350 [Discrete]                            
                                          GPU 2: Intel Iris Plus Graphics G7 @ 1.10 GHz [Integrated]
                                          Memory: 5.48 GiB / 15.41 GiB (36%)
```

## Day 1


```
x_1 x_2 x_3 x_4 x_5

Part 1:
find(2020 - x_i)
  
  O(N^2) nested loop
  O(NlogN) sort + bsearch or two pointers
  O(N) hash amort.

Part 2:
find(2020 - x_i - x_j)
  
  O(N^3) nested loop
  O(N^2 logN) sort + bsearch
  O(N^2) hash amort. or sort + two pointers
```

## FAQ

- [How can I build multiple binaries with Cargo?](https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo)
- [How do I split a string in Rust?](https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust)
- [Ergonomically reading a File into a String?](https://www.reddit.com/r/rust/comments/8d2u6t/ergonomically_reading_a_file_into_a_string/)
