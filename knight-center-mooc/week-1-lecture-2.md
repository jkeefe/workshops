# Build a basic chat bot inside Dexter


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
- It's 10 miles off the coast of Portsmouth, New Hampshire, on the east coast of the United States.

+ how do you get there
- There are many boats that make regular trips from Portsmouth, New Hampshire.

+ whats on star island
- There's a big, old hotel. Also a marine lab, some tennis courts, an old stone chapel and a historical museum. Also lots of seagulls!
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
```

Another nice trick is to add multiple `-` lines. Dexter will randomly pick from among them to reply.

```
+ *
- I'm sorry, I don't understand what you said.
- If that's a question, I don't know the answer yet.
- Ooof. I don't understand. Maybe try asking in another way.
```
