# Make a Twitter Bot

This module uses a cool Google Spreadsheet to make a Twitter bot!

Short link to this module: [jkef.me/make-twitter-bot](http://jkef.me/make-twitter-bot)

## What are you going to tweet?

Get your creativity rolling. There are a few possibilities for the bot program we're using, but for now pick one of these two:

* **Every Bot** - This will tweet out every row in a spreadsheet. Great if you have a lot of data you want to tweet for fun. Good examples are:
    - Old French Crime [@OldFrenchCrime](https://twitter.com/oldfrenchcrime)
    - American Injuries [@usinjuries](https://twitter.com/usinjuries)
    
* **Mad-lib Bot** - This bot will assemble a tweet in the format of, "Roses are [blank] violets are [blank]." Two fun examples are:
    - British Gardens [@GardensBritish](https://twitter.com/GardensBritish)
    - Bodega Bot [@bodegabot](https://twitter.com/bodegabot)



## Getting started

- Log into your Google account

Important Note about your Google Account: We'll be using a script you control but is written by someone else -- and it needs access to your Google spreadsheets and documents. I have no reason to suspect anything malicious about this script, but I don't know everything! If you are especially concerned about granting these permissions later, you may want to:

    - Use your personal account instead of, say, a company account
    - Start a new Google account just for this project

- Go to http://bit.ly/sheet-bot
- Make a copy of the spreadsheet, using "File -> Make a Copy"
- Give this spreadsheet a new name
- Close the original tab to avoid confusion
- Let's look at the tabs along the bottom of the screen for a moment:
    - Column ... this is the Mad-lib maker!
    - Every ... this is the every line bot.
    - Setup ... where we'll link up this spreadsheet to Twitter
    - Settings ... a bunch of tweaks. For now, just set the "Constructor" to `columns`
    - Preview ... here's where we can test the bot without tweeting
- Go back to "Setup" to get started

## If you already have a Twitter account ...

For several reasons, I don't recommend attaching a bot to your personal Twitter account. If the bot goes haywire for some reason, you could get locked out.

However, to make a new account, you need a phone number where you can get a text today. If the phone number you are going to use is already attached to your personal account, you need to disconnect that number from your original account and use it for the new one. (Don't worry, you can add it back to your original account once the bot is working.)

Here's how: 

- Log into your existing account
- Click on your small icon in the upper-right corner of the screen
- From the dropdown, select "Settings and Privacy"
- From the menu on the left, click on "Mobile"
- If there's a phone number associated with the account, click "Delete my phone"
- Click your small icon in the upper-right corner again
- Log out of your existing account
    
## Making your bot

The instructions on this spreadsheet are very good, so we're going to follow along, and add additional notes here:

- Step 1: Create a Twitter account for your bot 
    - Go to http://twitter.com
    - Click "Sign up"
    - When you log in, use the phone number of the phone you have with you.
    - You also need a real email you can check today, and probably one you haven't used before. Here's how!
        * Helpful hint: You can use a `+` sign in your email address to make a "new" address that goes to your existing email account. Both of these addresses will go to JaneDoe@gmail.com:
            - `JaneDoe@gmail.com`
            - `JaneDoe+twitbotJ@gmail.com`
    - Be sure to confirm your account via email! (And the confirmation is probably in your Spam folder.)
    
- Step 2: Create a Twitter account for your bot

You'll need to create an "app" within your new Twitter account. The bot connects with the app to make the tweets happen.

    - URL to go to is http://apps.twitter.com
    - Create a new app    
    - Name: <Whatever you want but it must be unique and cannot be blank>
    - Description: <Whatever you want but it cannot be blank>  
    - Website URL: <Your actual website or a placeholder like http://www.example.com/>
    - Callback: From the "Setup" tab on the spreadsheet, copy the link in the yellow box that begins `https://script.google.com/macros/d/...` and ends `...usercallback`. 
    - Go back to your Twitter page and paste this into the "Callback" box.
    
- Step 3: Complete and copy your App's settings.

    - Once you fill everything out and create your app, find the link "manage keys and access tokens"
    - Follow the instructions on the spreadsheet to copy over your Consumer Key and Consumer Secret
    - **Be especially careful not to have additional spaces or returns before or after the keys when you paste them into the spreadsheet**
    - Note ... this isn't in the instructions, but you also need to do:
        - Click on "Keys and Access Tokens" tab
        - Scroll down the page ... you'll see another section about "Access Tokens"
        - In the "Token Access" box, click the button that reads "Regenerate my Access Token and Token Secret"
        - Reload this page
        - Make sure that in the two places on this page that now say "Access Level" both have "Read and write" after them. If they don't, you may need to do "Regenerate my Access Token and Token Secret" until "read-only" becomes "read and write."

- Step 4: Generate a preview.
    - In the "Bot" menu (find it in the menu bar), pick "Generate preview"
    - Pick your Google account -- which allows this code access to your Google Drive
    - You'll get a Scary Warning saying it's untrusted.
        - This is true, yet we continue (see note above)
        - Click "Advanced"
        - Scroll down
        - Click go to bot
        - You may need to type "Continue"
        - Click the "Allow" button
    
- Step 5: Send a test tweet.
    - From the "Bot" menu, click "Send a Test Tweet" (You will actually do this twice in this step)
        - If you get a red message saying something failed, it is very likely that your Consumer Key and Consumer Secret aren't matching with Twitter. Be **sure** there are no spaces before/after these in the spreadsheet.
    - You'll get a pop-up with a link. Click that.
    - You'll get an authorization page from Twitter. Click Authorize.
    - Wait for the message that says you can "close this window" and close that tab.
    - Back at the spreadsheet close the pop-up with the link.
    - From the "Bot" menu, click "Send a Test Tweet" again. The bot should tweet! Check your account.
    - **If the bot does not tweet** ...
        - In the spreadsheet open the "log" tab
        - If the first line in the log includes the words "Read-only application cannot POST" in the middle of the error message ...
            - Go back to https://apps.twitter.com
            - Click on your app
            - Click on "Keys and Access Tokens" tab
            - Scroll down the page ... you'll see another section about "Access Tokens"
            - In the "Token Access" box, click the button that reads "Regenerate my Access Token and Token Secret"
            - Reload this page
            - Make sure that in the two places on this page that now say "Access Level" both have "Read and write" after them. If they don't, you may need to do "Regenerate my Access Token and Token Secret" until "read-only" becomes "read and write."
            - Then repeat Step 5
        - If the first line in the log includes the words "Service not authorized" or "expired token":
            - Repeat Step 5
    
- Step 6: Check your Settings tab
    - Be sure all the settings are right on your settings page
    - Double-check the "Timing" value
    
- Step 7: Start posting!
    - Go to the Bot menu and click "Start posting tweets"
    - If you need to change the Timing value, you'll have to "Stop posting tweets" and "Start posting tweets" again to make that change take place.
    - Most other changes made will take place with the next tweet


## Inspiration for more complex bots

- Neil Freeman's [Fake is the New Real](http://fakeisthenewreal.org/)
- Darius Kazemi's [Tiny Subversions](http://tinysubversions.com/projects/)
- Also Kazemi's [collection of corpora](https://github.com/dariusk/corpora)


