# USE THIS PROJECT ON YOUR OWN RISK.

## about this project
so lately, i have been feeling discomfort with my current username, i didnt play osu for a while so i didn't mind changing it but now, i really want to change it. unfortunately the name i want is gone, and i really want this name sm. so i came on the idea to create this project which notifies you if the account name you want is available. Welp. here we are, a discord bot, using rust, which fetches from the osu api the available usernames, if your username is free this discord bot will let you know! 

## how to use it.

1. go to this [website](https://osu.ppy.sh/store/products/32)
2. open the dev tools in your browser
3. enter in the input field something, go to the network tab in your dev tools, filter for fetch requests, and look for this `check-username-availability`
4. and then right click on it, copy it as some request, and copy the content of the set-cookie header, now you got ur cooookieee we will need this for later YAY, keep the cookie to yourself plssssssssssssssssssssssssssssss
5. now enter your bot token in the the .env file and use `cargo run` to start it!
6. to actually start sniping the username, you use the username_availablity slash command, or if you prefer prefix, the ,,username_availablity command!

if you use the prebuilt, please be sure to have the .env file in the same dir as the exe file, ofc with all needed properties have fun!
