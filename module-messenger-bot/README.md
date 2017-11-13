# Build a Chatbot

## Introduction

- Shortcut to this page: [jkef.me/build-chatbot](http://jkef.me/build-chatbot)
- We're going to make an "Ask me Anything" bot. Think about what you might make a bot to answer about. It should be something you know well. Could be about your project, the project's topic, your company, your favorite book, your favorite place, or you
- Your bot will eventually become public, so no secret stuff

## The Bot Engine

Facebook will be our interface. But the bot is built in a bot engine. There are LOTS of bot engines out there, and we're going to use [Dexter](https://rundexter.com), which is what we use for chatbots at Quartz. Dexter is free up to 100 users, so good for experimenting. For bigger projects, check out the [Dexter pricing here](https://rundexter.com/pricing).

- Go to [rundexter.com](http://rundexter.com)
- Make an account
- Click "Make your first bot" button (or something similar)
- Enter your email
- Pick a password
- Click "Signup"
- Click the blue "+ New Bot" button.
- Name it as you wish
- For template, Click "Blank Project"
- Click "Create Bot"
- Clear out what appeaers (we'll start from scratch for real)

## Basics

- Using a language called RiveScript

- `+` is what the human says ... the trigger
- `-` is what the bot says ... the response
- Note there's a space after the `+` or `-`

Let's start out with a good introductory phrase. Let people know right away what they'll get from this bot.

```
+ hi
- I'm a bot that can answer your questions about Star Island. Ask 
me anything!
```
- Try it in the demo phone!
- Note that if you can't see the demo phone, you may need to close the "helper" box that pops up in the lower right corner first.

## Simple Question and Answer

Come up with three questions a human might ask your bot, once that human knows what your bot is about.

- Make all the human questions (the triggers, or `+` lines ) **lowercase**
- In the human questions (triggers,  or `+` lines) **don't use punctuation**
- Put a blank line between each set. See below.
- Test your questions in the demo phone as you go.

```
+ where is star island
- It's 10 miles off the coast of Portsmouth, New Hampshire.

+ how do you get there
- Once you get yourself to Portsmouth, you can take one of the
boats that make regular trips.

+ whats on star island
- There's a big, old hotel. Also a marine lab, some tennis courts,
 an old stone chapel and a historical museum. Also lots of seagulls! 
 They're loud and think they own the island. But pretty harmless 
 otherwise. If you go hiking on the rocks away from the hotel, 
 tho, stay away from the small, gray ones. Parent gulls have 
 been known to attack people to protect their young! 😯
```

- Try it! Now we can test our bot in the "phone" that's on the side of the screen. Try typing your questions. They must be exact.

### A little help

Using a `*` means "anything" and [brackets] means it's optional. So here's a smart way to catch anyone saying the word "help" (with anything or nothing before or after "help"):

```
+ [*] help [*]
- Just type a question, and I'll give it my best shot.
```

Also, we probably want to say something nice when there is no match. You can use the "catchall" to match anything that hasn't already matched.

```
+ *
- I'm sorry, I don't understand what you said.
- If that's a question, I don't know the answer yet.
- Ooof. I don't understand. Maybe try asking in another way.
```

Another nice trick is to add multiple `-` lines. Dexter will randomly pick from among them to reply.

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

## Adding fun features

You can -- and should -- guide your user's path by providing buttons, which show up really nicely in Facebook. Here's how:

- Tip! You can insert buttons, links, images, and more using the "+Insert" button at the top of the editing window.

Add buttons ...

```
+ hi
- I'm a bot that can answer your questions about Star Island. What 
would you like to know about? ^buttons(Location, Getting There, What's on Star)
```

Add a link ...

```
+ location
- It's 10 miles off the coast of Portsmouth, New Hampshire. ^link("https://goo.gl/maps/T5qxWXTXLLF2","Star Island Map")
```

Add a picture ...

```
+ getting there
- Once you get yourself to Portsmouth, you can take one of the
boats that make regular trips ... like the Thomas Laighton ^image("http://media.johnkeefe.net/class-modules/boat.jpg")
```

Add chat timing ... adding `<delay seconds=2>` to break up a long line of text into separate chat bubbles.

```
+ whats on star
- There's a big, old hotel. Also a marine lab, some tennis courts,
 an old stone chapel and a historical museum.<delay seconds=2>Also lots of seagulls!<delay seconds=2>They're loud think they own the island. But pretty harmless otherwise. <delay seconds=2> If you go hiking on the rocks away 
 from the hotel, tho, stay away from the small, gray ones. <delay seconds=2>
 Parent gulls have been known to attack people to protect their young!<delay seconds=2>😯
```

And better help ...

```
+ [*] help [*]
- Here are the things I know how to do. Just pick one! ^buttons(Location, Getting There, What's on Star)
```

## Share it with others

You can allow other individual Facebook users to play with your bot. Just add them as "Testers":

- Go back to that Facebook tab
- Click "Roles"
- Click "Add Testers"
- Add people!

## Make it Public

To make the bot open to _anyone_ you need to make it public _and_ get Facebook's approval. For the little things we're doing (no broadcasts, no ads), you can expect to have it approved within 1-2 days.

- Go back to that Facebook tab
- Click "App Review" in the sidebar
- Click the big "Make [bot name] Public" switch
- Click "Submit for Review"

## Adding Natural Language Processing

### Introduction to ~~API.ai~~ Dialogflow

There are lots of tools out there to use. We'll play with [Dialogflow](https://dialogflow.com) (which used to be called API.ai ... so there may be some notations here that still reference that).

### Setup

As usual, you'll need to sign up. It's free. And you'll need a Google/Gmail account.

- Click "Sign up for Free"
- Log in with Google (Dialogflow is a Google product)
- Choose "Create Agent"
- Name it "Blank agent"
- In the sidebar, chose "Prebuilt Agents"
- Then in the main area, find the logo for the "Small Talk" prebuilt agent (Note, this is _not_ the "Small Talk" option in the left-side menu)
- Just leave the "Link to Google Project" line empty and hit OK
- Wait and then click "Proceed to Agent"
- This is tricky ... now in the _drop-down_ menu, chose "Small-Talk." Again, not the "Small Talk" item that's always in the sidebar. Look for the hyphen in `Small-Talk`. That's the right one.
- Now, to end this craziness, let's rename it. Click the gear next to `Small-Talk` (with the hyphen)
- Call it "My-Workshop-Bot"
- Click "Save"

### Play a little

Find the "Try it now" box at the top and try typing some random phrases that might constitute small talk. What happens?

Pay close attention to the "Intent" and "Action" areas.

Also try things that might be casual synonyms for "yes" and "no."

### Connect it to your Dexter Bot

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
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
- The action I detect is: ${{result.action}}

> object encode_uri javascript
    return encodeURIComponent(args[0])
< object
```

- OK! Now try saying some things into the sample phone on the Dexter console.

### Handle calls for help

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

### Got nothin'? Use API.ai's answer

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

## Pursuing more

- Dexter documentation is really good
- Read more about [RiveScript](https://www.rivescript.com/docs/tutorial), the language Dexter uses. (Most, but not all, RiveScript features are available in Dexter.)


