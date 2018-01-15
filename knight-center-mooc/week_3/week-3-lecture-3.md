# Make an Alexa Fact skill

## Make a Dexter Bot Script

- Make the first trigger `+ launchrequest` and provide a friendly introduction. This will be the entry point for your bot script.
- Make the second trigger `+ factlist` and provide a list of facts. Remember, if there are multiple responses beginning with `-`, one will be picked at random.
- Here's an example you can just copy-and-paste into your Dexter bot script:

```
+ launchrequest
- Here's your Star Island fact: {@ factlist}

+ factlist
- Star Island is 10 miles off the coast of Portsmouth, New Hampshire.
- The hotel on the island is called "The Oceanic."
- There is no natural fresh water reserves on the island. Fresh water is collected from arriving boats, rainwater and a seawater desalination system.
- The pirate Blackbeard visited once and is said to have left both treasure and one of his wives on the neighboring island.
```

We also need a "farewell" section to close out our Alexa session, using the shortcut: `^alexaEndSession(true)`

```
+ farewell
- Have a great day. ^alexaEndSession(true)
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

- Click the "Publish Topic" button to make sure your edits stick!

## Wiring up Alexa

- Chose "Platforms" from the menu at the top of the screen
- Chose "Alexa"
- In the instructions, follow the link to the "Amazon Developer Console."
- If you're not yet an Amazon developer, click the button to create your Amazon Developer Account
    - Provide the information requested
    - NOTE! If you have an Amazon Alexa device already, it's a lot easier down the road if you use the Amazon account (and email address) associated with that device.
- Log into your Amazon Developer Account
- Choose "Alexa Skills Kit"
- Click "Add a new skill"
- Skill type is "Custom Interaction Model" (the default)
- Give your app a name: This will be the one users see in their Alexa app.
- Give your app an invocation: This is the phrase people use to "open" or start your app.
    - I'm using "Island Facts" for both of them
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

If you Alexa Developer Account matches the account with which you registered your Alexa device, just say the invocation phrase! In my example, it would be "Alexa, open Island facts."

Note that if the blue ring gets stuck "on," it probably means you forgot to use `^alexaEndSession(true)` as the last thing in your script.