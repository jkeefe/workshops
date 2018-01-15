# Careful walkthrough of RiveScript basics

Dexter is built on top of RiveScript, which is a language made for writing bots!

RiveScript, like many programming languages, is persnickety about how you write for it. And if you make a mistake, the errors can be quite cryptic. Here's one of them:

[show error]

I want to review some of the key points to remember as we write in Dexter.

## RiveScript Review

### Triggers and Bot Responses

- `+` is what the human says ... the trigger
- `-` is what the bot says ... the response
- There's a space after the `+` or `-` and before the text that follows.

```
+ hi
- I'm a bot that can answer your questions about Star Island. Ask 
me anything!
```
- I always put a blank line before each new `+` trigger, though it's not actually essential.

- All triggers -- the  `+` lines -- must be **lowercase**
- All triggers -- again, the `+` lines -- should **leave out punctuation**
    - There are some exceptions to this, but they're code-related
    - Pay special attention to apostrophes, commas, question marks, and periods -- leave them out

```
+ whats on star island
- There's a big, old hotel. Also a marine lab, some tennis courts,
 an old stone chapel and a historical museum. Also lots of seagulls!
```

- There is a single carriage return at the end of a `+` command and at the end of a `-` command.

### Order matters

When someone types something to your bot, RiveScript will go through your bot script top to bottom until it hits a match.

```
+ hi there
- Hi friend!

+ hi there
- This response would never be seen.
```

This is also why we put "catchall" functions at the end.



### "Pick one" from a response list

- If you have multiple `-` lines after a `+` trigger, RiveScript will pick one of the `-` lines at random.

```
+ tell me a cat fact
- Cats have four legs.
- Cats belong to the Felidae family.
- Dogs have owners, cats have servants.
```

### Catchalls

- An asterisk `*` in a trigger means "anything"

```
+ *
- I'm sorry, I don't understand what you said.
- If that's a question, I don't know the answer yet.
- Ooof. I don't understand. Maybe try asking in another way.
```

### Optionals

Brackets -- `[ ]` -- surround optional content in a trigger. We used this in the "help" trigger:

```
+ [*] help [*]
- Just type a question, and I'll give it my best shot.
```

Let's dissect this a moment, just to be clear:

`+ help` ... matches only the human typing "help" and only "help"; "help me" would not match.
`+ * help *` ... would match only "please help me," because there are words before and after "help"; "help me" would not match.
`+ [*] help [*]` ... matches any mention of "help," no matter if there are words before or after it -- because they are optional. So "help me" matches, as does "help" and "can you please help me?"

## Quiz Questions:

Examples of mistyped triggers, etc.

