# Alexa Fact Skill

Making a simple Alexa fact skill with Dexter, in the spirit of "[Cat Facts](https://www.amazon.com/deegles-co-Cat-Facts/dp/B017OBJI46)."

## Get started with Dexter

- Go to [rundexter.com](http://rundexter.com)
- Make an account
- Click "Make your first bot" button (or something similar)
- Enter your email
- Pick a password
- Click "Signup"
- Click the blue "+ New Bot" button.
- Name it as you wish (but don't use quotes or apostrophes)
- For template, Click "Blank Project"
- Click "Create Bot"
- Clear out what appeaers (we'll start from scratch for real

You can keep going to play with the Alexa skill-building, but if you're new to Dexter, you can get the basics and build your first chatbot by [heading over here](https://github.com/jkeefe/workshops/tree/master/module-build-a-chatbot).)

## Start a  Dexter.

- Make the first trigger "launchrequest" and provide a friendly introduction.
- Make the second trigger "factlist" and provide a list of facts. Remember, if there are multiple responses beginning with `-`, one will be picked at random.
- Here's an example you can just copy-and-paste into your Dexter bot script:

```
+ launchrequest
- Here's your Star Island fact: {@ factlist}

+ factlist
- Star Island is 10 miles off the coast of Portsmouth, New Hampshire.
- The hotel on the island is called "The Oceanic."
- There is no natural fresh water reserves on the island. Fresh water is collected from arriving boats, rainwater and a seawater desalination system.
- The pirate Blackbeard visited once and is said to have left both treasure and his wife on the neighboring island.
```

We also need a "farewell" section to close out our Alexa session, using the shortcut: `^alexaEndSession(true)`

```
+ farewell
- You will come back. You will come back. Have a great day. ^alexaEndSession(true)
```

Finally, we need to direct every response to the farewell trigger with `{@ farewell}` at the end of every line. (If you don't, Alexa will "hang" and leave the blue ring alive. Say "Alexa, quit" to escape).

So:

```
+ factlist
- Star Island is 10 miles off the coast of Portsmouth, New Hampshire. {@ farewell}
- The hotel on the island is called "The Oceanic." {@ farewell}
- There is no natural fresh water reserves on the island. Fresh water is collected from arriving boats, rainwater and a seawater desalination system. {@ farewell}
- The pirate Blackbeard visited once and is said to have left both treasure and his wife on the neighboring island. {@ farewell}
```

- The full script is here.
- Click "Publish Topic" to make sure your edits stick!

## Wiring up Alexa

- Chose "Platforms" from the menu at the top of the screen
- Chose "Alexa"
- In the instructions, follow the link to the "Amazon Developer Console."
- If you're not yet an Amazon developer, click the button to creatue your Amazon Developer Account
    - Provide the information requested
    - NOTE! If you have an Amazon Alexa device already, it's a lot easier down the road if you use the Amazon account (and email address) associated with that device.
- Log into your Amazon Developer Account
- Choose "Alexa Skills Kit"
- Click "Add a new skill"
- Skill type is "Custom Interaction Model" (the default)
- Give your app a name: This will be the one users see in their Alexa app.
- Give your app an invocation: This is the phrase people use to "open" or start your app.
    - I'm using "Star Island Facts" for both of them
- Leave the other settings as they are.
- Click SAVE
- You now have an Application ID. It probably starts with `amzn...`
- Copy your Application ID.
- Paste it into Dexter (make sure there are no trailing or leading spaces)
- Click NEXT
- Go back to your Amazon tab
- Click NEXT to get to the "Interaction Model" page
- Go back to Dexter
- Turn on the "Catch-All mode" which is in Beta
- Click Next

Follow the Dexter instructions to fill out the next fields:
    - Intent Schema: Use the Dexter code
    - Custom Slot types ...
        - Enter Type: `catchall`
        - Enter Values: copy and paste the list of apparently random words from Dexter
            - Click ADD
        - Sample Utterances: `CatchAllIntent {CatchAll}`
    - Click Save
    
You'll need to wait for a little bit for the model to be created.

    - Click Next

On the configuration page:

    - Pick "HTTPS"
    - Go back to dexter and get the "Webhook URL" and pasted it into the Alexa "Default" box
    - Click Next
    
On the SSL Certificate page, pick:

    - `My development endpoint is a sub-domain of a domain that has a wildcard certificate from a certificate authority`
    - Click SAVE then NEXT
    
On the Test page, make sure the skill is "Enabled"

Go back to Dexter, and click NEXT until you get to the "Deploy" button. Click "Deploy."

(You'll want to be sure you bot topic is published, too. Go back to the "Edit" and "Publish Topic" if you have to.)

## Test it out!

### Don't have an Alexa device handy?

On the Alexa "Test page" you can "Enter Utterance" to test how Alexa will respond.

You can see how Alexa will respond in the "Service Response" section, or you can _hear_ the response by clicking on the "Listen" button below the "Service Response" window.

### If you have an Alexa device matching your developer account

If you Alexa Developer Account matches the account with which you registered your Alexa device, just say the invocation phrase! In my example, it would be "Alexa, open Star Island facts."

Note that if the blue ring gets stuck "on," it probably means you forgot to use `^alexaEndSession(true)` as the last thing in your script.

### To invite others to try (or invite another Alexa device not tied to your dev account)

To beta test on an actual Alexa device:

- Must have **all** of the green checkmarks checked. 
- That includes a 108x108 and a 512x512 px logo.
- Invite beta testers 
- If the Amazon account you're using for development is not the one you use for your Alexa device, then you need to invite yourself at the address you use for the device, too.
- Beta testers visit http://alexa.amazon.com ... and should eventually get a pop-up inviting them to participate
- Then the skill gets added.

## Dexter Docs

[More information here](http://docs.rundexter.com/writing/advanced/alexa/)!

## Historical Reference

Here's where I used to begin when teaching this session, without Dexter:

Let's start with [this walkthrough](https://github.com/alexa/skill-sample-nodejs-fact/blob/master/step-by-step/1-voice-user-interface.md).
