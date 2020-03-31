# mifi_used_data

[comment]: # (lmake_readme version)  

*Things are changing fast. This is the situation on 2020-03-30.*  

## Description

The CLI reads the data from my mifi router status page and saves it in a database.  
Then shows the used data in a graph.  

![snap01](https://github.com/LucianoBestia/mifi_used_data/blob/master/img/mifi_screen_01.JPG)

The url for the status:  
<http://192.168.225.1/cgi-bin/en-jio/mStatus.html>  
The part of the html, that is interesting:  

```html
<strong id="pSentPackets">UL:</strong><br>
<label id="lsentPackets">258.55 MB</label><br><br>
<strong id="pRecPackets">DL:</strong><br>
<label id="lRecPackets">1.17 GB</label>
```

## bundled database sqlite

I will use `rusqlite` to write to a private local database file.  

## main and lib must be separate in a workspace

Because of testing it is always better to have separate `lib` and `main`.  
Always.  
There is no reason to have only `main` except for super amateur tutorials.  

## calculate and draw graph

Calculates a sql table with same time interval, so the graph is meaningful.  
Draws the graph on the screen with simple printl! macros.  

## TODO

- what will start the cli every x minutes in WSL?
