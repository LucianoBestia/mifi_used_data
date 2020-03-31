# mifi_used_data

[comment]: # (lmake_readme version)  

*Things are changing fast. This is the situation on 2020-03-30.*  

## Description

The CLI reads the data from my mifi router status page and saves it in a database.  
Then shows the used data in a graph.  
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

## TODO

- from the table data_used, calculate constant time intervals.How much is used in that time interval.
- The graph in CLI ?every row can be one hour or so, two colors for dl and ul.
- formula to find y for a known x:  Y2 = Y1 + ((X2-X1) * (Y3-Y1)/(X3-X1))
- datetime instead of elapsed minutes
- what will start the cli every x minutes in WSL?
