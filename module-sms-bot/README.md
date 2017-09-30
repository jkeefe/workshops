# Connecting by Texting

## Make the dog-cat script in Dexter

We'll walk through making [this script]('./cat-dog-basic.rs').

- `%` trick
- `<star>`
- `<formal>`
- `<call>await_answer yesno dog-answer</call>`
- If-then: `* <get openended-answer> == yes => <set dogvar=yes> Cool. Do you like cats?`
- `<set dogvar=yes>`
- `<get dogvar>`
- `{@ another trigger}`

Cut and paste in this gnarly code at the very end:

```
> object encode_uri javascript
    return encodeURIComponent(args[0])
< object

// this sets values for the open-ended question
> object await_answer javascript
    var user = rs.currentUser();
    var userdata = rs.getUservars(user);
    var currenttopic = userdata.topic;
    var nexttrigger = args[1].replace(/[,!?;:']/g, "").replace(/[-_.]/g," ");
    
    rs.setUservar(user, "openended-type", args[0]);
    rs.setUservar(user, "openended-topic", currenttopic);
    rs.setUservar(user, "openended-next-trigger", nexttrigger);
    rs.setUservar(user, "openended-answer", "undefined");

    return;
< object
```

Try it!

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
    
## Add NLP (Optional)

Establish account with API.ai if that hasn't happened yet. See [this section](https://github.com/jkeefe/workshops/tree/master/module-build-a-chatbot#adding-natural-language-processing) for details!

Walk through changes in the [cat-dog-nlp](./cat-dog-nlp.rs) script.


Add the API key for API.ai! 

```
! var apiai = Bearer YourAPIaiClientAccessTokenGoesHere
```

Update the catchall trigger to:

```
+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* <get openended-type> == yesno => {@ handle yesno ${{result.action}} }
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
* ${{result.fulfillment.speech}} != "" => ${{result.fulfillment.speech}} 
- Sorry, I have no idea what you just said.
```

Update the yes/no handler to: 

```
+ handle yesno *
* <star> == smalltalk.confirmation.yes => <set openended-answer=yes> {@ openended reset and route}
* <star> == smalltalk.confirmation.no => <set openended-answer=no> {@ openended reset and route}
- Is that a yes or a no? ^buttons("Yes!", "No!")
```


