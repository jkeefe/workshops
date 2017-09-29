# Connecting by Texting

## Pre-requisites

- [Build-a-Chatbot](../module-buid-a-chatbot) module
- [Advanced Dextering](../module-advanced-dextering) module

## Examples


## Wire it up to SMS with Twilio

### Set Up Twilio

We'll be using [Twilio](http://twilio.com) to make the connection between the telephone systems and your code.

- Open a new browser tab
- Go to [Twilio](http://twilio.com)
- "Sign up" to make a free trial account.
    - Use: SMS
    - Project: SMS Surveys
    - Language: node.js
    - Under 10,000 uses per month
- Enter the number of the cell phone you have with you
    - This will only be used for verification
    - You won't (and, in fact, can't) "send" from your personal number
    - Can check the box to avoid being called
- Enter verifcation number
- "Get a Number"
- "Choose this number"
    - Mine is (347) 768-7183
    - AKA +13477687183
- Trial account set up!
    - Your trial account has $14.50 remaining
    - Trial accounts can only send messages to verified numbers in these countries
    - Messages sent in trial will begin with "Sent from a Twilio Trial Account"
    - While you have a trial account, you're limited to one Twilio number
    
### Set up Dexter

- If you haven't already, open a new tab and start Dexter at [rundexter.com](http://rundexter.com)
- With your dog-cat script visible, click the "Platforms" button on top.
- Choose "Twilio SMS" on the left side
- We did Step 1!
    - Click "Next"
- Step 2: Link phone number
    - Go back to the Twilio tab and copy your new Twilio phone number
    - Go to the Dexter tab and paste your new Twilio phone number in the box on this page.
    - Phone number should begin with `1` (or your country code) without the `+` sign.
    - Phone number shouldn't have spaces.
    - So like this: `13477687183`
- Step 3: Link Twilio Credentials
    - Go to the Twilio Tab
    - Find the Twilio Dashboard, which you get to with the icon at the top of the lefthand column.
    - Copy the Account SID
    - Go to Dexter and paste it in the proper box
    - Go back to Twilio
    - Click the "eye" icon to show the Auth Token
    - Copy taht
    - Go to Dexter and paste it in the proper box
    - Click "Next"
- Step 4: Webhook from Dexter to Twilio
    - Click in the "Webhook URL" box to copy it to your clipboard
    - Go to Twilio
    - Click on "Phone numbers" (Not "Buy a Number")
    - Click on your new Twilio phone number, which is in red
    - Look carefully for the field called "When a Message Comes In"
    - Delete what's currently in that box
    - Paste the webhook from your clipboard into that box
    - Click "Save"
- Step 5: Deploy!
    - Go back to the Dexter Tab
    - Click "Next"
    - Click "Deploy"
    - Click "Next"
- Publish your Bot
    - Click "Edit" at the top of the Dexter page to go back to your script
    - Click "Publish Topic"
- Call your bot!
    - Text "hi" to your new Twilio phone number
- Did it work?
    - It'll have "Sent from your Twilio trial account" on every message
    - [picture, from email]
    - It'll only work from your personal phone unless/until (or other numbers you authorize) unless/until you upgrade
    
## SMS Survey with Airtable

### Prepare Dexter

Add this to your script:

```
> object currentDate javascript
  return new Date(Date.now()).toLocaleDateString()
< object
 
+ post_results
$ POST YOUR_AIRTABLE_URL_GOES_HERE {"headers": {"Authorization": "Bearer YOUR_AIRTABLE_API_KEY_GOES_HERE", "Content-Type": "application/json"}, "body": {"fields": {"Time": "<call>currentDate</call>", "User":"<get _platformId>", "Dogs": "<get dogs_answer>", "Cats": "<get cats_answer>" }}}
* ${{__status}} != 200 => ${{error.message}}
- See you soon!
```

Also need to adjust the `+ cat answer` code to set the `catvar` variable. See how we make the change below by adding `<set catvar=yes>` and `<set catvar=no>` below:

```
+ cat answer
* <get openended-answer> == yes => <set catvar=yes> {@ combined <get dogvar> yes}
* <get openended-answer> == no => <set catvar=no> {@ combined <get dogvar> no}
```

Also, after all four possible answers, we need to send folks to the `post_results` trigger. So add `{@ post_results}` to the end of all four possible answers, like this:

```
+ combined yes yes
- Both can be truly great pets. {@ post_results}
```

### Prepare Airtable

- Go to airtable.com and sign up for an account.
    - Use an email address you can verify today
- Skip the marketing questions
- Go to your email and verify your account
- Add a base
- Start from scratch
- Call it "My Events"
- Pick a color and an icon
- Click on the icon once it's set
- Clear all of the boxes that come up
- Edit this so that you have 
    - Time, User, Dogs, Cats
    - Make them all "Single line of Text"
- Open a new tab
- Go to https://airtable.com/api
- Find your new "base" and click it
- OK Look carefully at this. There's a URL in the darker part of this page. You need that URL. It'll look something like `https://api.airtable.com/v0/appn5FegAgnXtr0RG/Table%201`
- Paste that into your dexter code where it says `YOUR_AIRTABLE_URL_GOES_HERE`
- Go back to Airtable
- You need an Airtable API key. Get it by clicking the "account" link in the "Authentication section."
- Click "Generate an API Key"
- Copy the key
- Go back to Dexter and paste it where it says `YOUR_AIRTABLE_API_KEY_GOES_HERE`
- Try your script in the mock phone.
- Check your Airtable!


