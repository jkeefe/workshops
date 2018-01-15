# Facebook Part 1

- Update john-bot
- Add `get started` to `hi`

## Publish your bot

To get this ready to share (and wire up to Facebook), click the green "Publish Topic" button.

## Wire up to Facebook

Dexter is great about walking you through this entire process, under the "Platforms" button. 

- To start, click on the "Platforms" button.
- Choose Facebook.

### Setting up your Facebook Page

In Facebook, bots live on "pages" you add to your profile.

- You'll need a Facebook account (look on with someone if you don't have one)
- Open a new browser tab and log in to [Facebook](https://facebook.com)
- To make the page ...
   - Click "Pages" in your left-hand menu, or go to https://www.facebook.com/pages/create/
   - Make your new page. Don't worry about the pretty details, you can add them later.
   - Once it's made, look in the left column for "About" (you may need to click "see more"). Click on "About"
   - Scroll all the bottom
   - Highlight and copy the "Page Id"
   - Go back to the Dexter tab
- Paste the ID number into the box
- Click "Next"

### Setting up your Facebook Bot App

This is a very typical setup: The bot you make needs to connect to a new "app" in the platform you are using, such as Facebook.

![Bot & platform relationship](./images/bot_platform.png)

The part missing so far is the platform app. To make an app in Facebook, you need to register as a Facebook developer.

- Go to Facebook [developer portal](https://developers.facebook.com/apps/) and follow the instructions.  

After you are registered as a Facebook developer we need to make that app and then make a connection between the bot settings and the app settings.

- Click "Create New App"
- Give it a name
- Pick "Messenger" as the product
- Click on the "settings" at the side
- On the **Facebook** tab
    - Copy App ID 
- On the **Dexter** tab
    - Paste App ID paste into box
- On the **Facebook** tab 
    - Get App Secret by clicking "show" (you may have to confirm your password)
- On the **Dexter** tab
    - Paste into Dexter 
- Back at the **Facebook** tab ...
    - App Domains, put `rundexter.com`
    - Click "Add Platform" at the bottom
    - Make the site url `https://rundexter.com`
    - CLICK SAVE CHANGES
    
Your App should look like this:
![Facebook App Settings](./images/fb_app_setup.png)

One more Facebook Setting to deal with:

- From the left-hand rail on the Facebook developer page, click `+Add Product`
- Pick the Facebook Login _icon_
- Next click the "Facebook Login" _text_ in the left-hand rail
- Go back to the Dexter tab and copy the "Valid OAuth Redirect URI"
- Back to Facebook, paste it into the "Valid OAuth Redirect URIs"
- Click Save Changes

Almost there!

- Back to the **Dexter** tab
    - Click Next
    - Click Authenticate
    - Agree
    - Click Next
    - Click Deploy
    - (Do not click Redeploy)
    - Click Next
- Go to your bot! (Click the link in Dexter)
- Then try talking to your bot in Messenger

