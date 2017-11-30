# Adding Natural Language Processing

This module adds Natural Language Processing to either the [web-based](../module-build-a-chatbot) or [Facebook Messenger](../module-messenger-bot) chatbots.

## Introduction to ~~API.ai~~ Dialogflow

There are lots of tools out there to use. We'll play with [Dialogflow](https://dialogflow.com) (which used to be called API.ai ... so there may be some notations here that still reference that).

## Setup

As usual, you'll need to sign up. It's free. And you'll need a Google/Gmail account.

- Click "Sign up for Free"
- Log in with Google (Dialogflow is a Google product).
- Choose "Create Agent" -- and you may need to authenticate with Google again here.
- Name it "Blank agent"
- Click "Create"
- If you don't see a sidebar on the left side, click the menu icon (the three horizontal lines)
- In the sidebar, chose "Prebuilt Agents"
- Then in the main area, find the logo for the "Small Talk" prebuilt agent (Note, this is _not_ the "Small Talk" option in the left-side menu)
- Click "Import" inside this Small Talk box.
- Just leave the "Link to Google Project" line empty and hit OK
- Wait and then click "Proceed to Agent"
- If you don't see a sidebar on the left side, click the menu icon (the three horizontal lines)
- This is tricky ... At the top of the sidebar, use the _drop-down_ menu to chose "Small-Talk." Again, not the "Small Talk" item that's always in the sidebar. Look for the hyphen in `Small-Talk`. That's the right one.
- Now, to end this craziness, let's rename it. Click the gear next to `Small-Talk` (with the hyphen)
- Call it "My-Workshop-Bot"
- Click "Save"

## Play a little

Find the "Try it now" box at the top and try typing some random phrases that might constitute small talk. What happens?

Pay close attention to the "Intent" and "Action" areas.

Also try things that might be casual synonyms for "yes" and "no."

## Connect it to your Dexter Bot

- On the Dialogflow settings page, copy the "Client Access Token"
- Switch to your Dexter bot
- Paste the "Client Access Token" at the very top of your bot script.
- In front of the token, add `! var apiai = Bearer ` so it looks something this:

```
! var apiai = Bearer ab12cd34ef56ab78cd90ef12
```

- Copy the code below and paste it to the bottom of your bot script:

```
+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<get _platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
- The action I detect is: ${{result.action}}

> object encode_uri javascript
    return encodeURIComponent(args[0])
< object
```

- OK! Now try saying some things into the sample phone on the Dexter console.

## Handle calls for help

Let's be sure that whenever someone says help, they get a kind response:

```
+ help
- I'm sorry you're having trouble!
- I'll try to get you some help!
```

Try it in the phone simulator!

But what about "Can you assist me?" For that, let's handle anything the natural language thinks is a call for help, or `smalltalk.agent.can_you_help`.

- Copy this line ...

```
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
```

- ... and paste it in your "catchall" trigger as the second-to-last line. Like this:

```
+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
- The action I detect is: ${{result.action}}
```

We've added an "if-then" statement to the block. It says: If `${{result.action}}` is equal to `smalltalk.agent.can_you_help` then go to a `help` trigger.

See it there?

Try it!

Note that anything _other_ than help ends up at the last line, which the bot uses.

## Got nothin'? Use Dialogflow's answer

Remember when you use Dialogflow in the testing box and it actually provides an answer? We can use that. It's 
`${{result.fulfillment.speech}}`. (I know this, because I clicked on the "Show JSON" button in API.ai and can see what would be sent back to our bot!)

So let's replace the last line of our messy code block with that. Instead of 

```
- The action I detect is: ${{result.action}}
```

use ...

```
- ${{result.fulfillment.speech}}
```

So now your covered ... pretty much.

Only issue remaining is that if the bot doesn't recognize anything it sends back blankness. So let's tweak the final lines in the "catchall" block one last time, pasting in:

```
+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* <get openended-type> == yesno => {@ handle yesno ${{result.action}} }
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
* ${{result.fulfillment.speech}} != "" => ${{result.fulfillment.speech}} 
- Sorry, I have no idea what you just said.
```

Those last two lines are: If the "speech" line is not equal (`!=`) to blankness (`""`) then respond with the "speech" line.

If not, we get the last line. You can add more of these `-` lines to add variety.

## Starting from scratch?

What we just built is a good starter script, incorporating the natural language processing for catching strangeness and letting you build from scratch. If you'd like to start over, start from [this file](./a-good-start.rs).



