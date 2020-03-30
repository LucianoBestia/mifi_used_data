# mifi - used data

*Things are changing fast. This is the situation on 2020-03-27.*  
Read the data from the wifi status page and saves it in a database.  
Then show the used data in a graph.  
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

## main and lib separate in a workspace

Because of testing it is better to have separate lib and main.  
Always. There is no reason to have only main except for super amateur tutorials.  



