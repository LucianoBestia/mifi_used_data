# mifi_used_data

[comment]: # (lmake_readme cargo.toml data start)

[comment]: # (lmake_readme cargo.toml data end)  

*Things are changing fast. This is the situation on 2020-04-04.*  

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

## crontab - repeat every 15 minutes

I use Debian.  
Run the app every 15 minutes in linux.  
`crontab -e`  
Add the following line for an every-15-minutes interval.  
`*/15 * * * * /home/luciano/rustprojects/mifi_used_data/target/release/mifi_used_data`  
Save the file, and that is it.  
To see the crontab log:
`grep CRON /var/log/cron.log`  

### if crontab log is not configured

If the file does not exist, edit the config file:  
`sudo nano /etc/rsyslog.conf`  
so that the line
`cron.*                          /var/log/cron.log`  
is not commented out.  
Choose the level of logging in this file:  
`sudo nano /etc/default/cron`  
write  
`EXTRA_OPTS="-L 1"`  
Add a new test job that executes every minute,
just to see if cron works:
`crontab -e`  
Add the following line  
`* * * * * /bin/echo "cron works"`  
Save the file.  
Reload logger:  
`sudo /etc/init.d/rsyslog reload`  
Retry grep, now it must work:  
`grep CRON /var/log/cron.log`  

### Some utils  

To set cron to start with the OS:  
`sudo update-rc.d cron defaults`  
Other service commands:  
`sudo service cron status`  
`sudo service cron start`  
`sudo service cron stop`  

### windows WSL and cron

I didn't expect Cron in WSL to work reliably.  
I don't really know when the WSL starts and end.  
I will add an internal timer/scheduler, to be sure that it works nice.  
I tried the library job_scheduler, but the time resolution was so high,
that I got 33% CPU usage all the time.  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## TODO

- learn datetime in Rust
- my laptop cannot be on all day. Maybe my android phone?  
https://robertohuertas.com/2019/06/29/android_foreground_services/
- graph start at hour 00
- what uses 80MB / hour all night long and when nobody is working?
- what is uploading 360 mb in 15 minutes? Only 3 clients: minja A50, LucianoA50 and Mac.
