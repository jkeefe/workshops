# Make a Twitter Bot

This module uses a cool Google Spreadsheet to make a Twitter bot!

## What are you going to tweet?

Get your creativity rolling. There are a few possibilities for the bot program we're using, but for now pick one of these two:

* **Every Bot** - This will tweet out every row in a spreadsheet. Great if you have a lot of data you want to tweet for fun. Good examples are:
    - Old French Crime [@OldFrenchCrime](https://twitter.com/oldfrenchcrime)
    - American Injuries [@usinjuries](https://twitter.com/usinjuries)
    
* **Mad-lib Bot** - This bot will assemble a tweet in the format of, "Roses are [blank] violets are [blank]." Two fun examples are:
    - British Gardens [@GardensBritish](https://twitter.com/GardensBritish)
    - Bodega Bot [@bodegabot](https://twitter.com/bodegabot)


## Getting started

- Go to http://bit.ly/sheet-bot
- Make a copy of the spreadsheet, using "File -> Make a Copy"
- Give this spreadsheet a new name
- Close the original tab to avoid confusion
- Let's look at the tabs along the bottom of the screen for a moment
- Setup ... where we'll link up this spreadsheet to Twitter
- Settings ... a bunch of tweaks. For now, just set the "Constructor" to `columns`
- Preview ... here's where we can test the bot without tweeting
- Column ... this is the Mad-lib maker!
- Every ... this is the every line bot.
- Go back to "Setup" to get started

The instructions on this spreadsheet are very good, so we're going to follow along. 

NOTE: We'll be using a script you control but is written by someone else -- and it needs access to your Google spreadsheets and documents. I have no reason to suspect anything malicious about this script, but I don't know everything! If you are especially concerned about this, you may want to:

    - Use your personal account instead of, say, a company account
    - Start a new Google account just for this project

That said, here are some additional notes on each step:

- Step 1 notes: 
    - Already have a phone number liked to a Twitter account? Here's how to a phone number from a previous Twitter account:
        - Log into your original account
        - Click your icon in the menubar at top
        - Click on "Settings and Privacy"
        - Click on "Mobile"
        - Click "Delete my phone"
        - Later, once the bot is running, reverse this process to re-associate it
        
- Step 2 notes:
    - URL to go to is http://apps.twitter.com
    - Once you fill everything out and create your app, find the link "manage keys and access tokens"
    - Follow the instructions to copy over your Consumer Key and Consumer Secret
    - Note ... this isn't in the instructions, but you also need to do:
        - Scroll down the page ... you'll see another section about "Access Tokens"
        - In the "Token Access" box, click the button that reads "Regenerate my Access Token and Token Secret"
        - Make sure that in the two places on this page that now say "Access Level" both have "Read and write" after them.
    
- Step 5 notes (test tweet):
    - You'll need to click that link to authorize this spreadsheet to use your twitter account.
    - When you're done, close that box with the link using the "X"
    - Do the "Send a Test Tweet" again
    
- Step 7 notes (start posting!)
    - Be sure all the settings are right on your settings page
    - Double-check the "Timing" value
    - Go to the Bot menu and click "Start posting tweets"
    - If you need to change the Timing value, you'll have to "Stop posting tweets" and "Start posting tweets" again to make that change take place.
    - Most other changes made will take place with the next tweet


## Inspiration for more complex bots

- Neil Freeman's [Fake is the New Real](http://fakeisthenewreal.org/)
- Darius Kazemi's [Tiny Subversions](http://tinysubversions.com/projects/)
- Also Kazemi's [collection of corpora](https://github.com/dariusk/corpora)


