+ hi
- Hi! Do you like dogs?

+ *
% hi do you like dogs
* <star> == y => <set dogvar = yes>

- Do you like cats?

+ *
% do you like cats
- 

+ dog answer
* <get openended-answer> == yes => <set dogvar=yes> Cool. Do you like cats? <call>await_answer yesno cat-answer</call>
* <get openended-answer> == no => <set dogvar=no> OK. Do you like cats? <call>await_answer yesno cat-answer</call>

+ cat answer
* <get openended-answer> == yes => {@ combined <get dogvar> yes}
* <get openended-answer> == no => {@ combined <get dogvar> no}

+ combined yes yes
- Both can be truly great pets.

+ combined yes no
- Woof! Dogs are great pets.

+ combined no yes
- Meow! Just remember, dogs have owners. Cats have servants.

+ combined no no
- Maybe you're more of a bird person.

+ [*] help [*]
- I'm sorry you're having trouble!
- I'll try to get you some help!

+ *
* <get openended-type> == yesno => {@ handle yesno <star> }
- Sorry, I don't know the answer to that.

+ handle yesno *
* <star> == yes => <set openended-answer=yes> {@ openended reset and route}
* <star> == y => <set openended-answer=yes> {@ openended reset and route}
* <star> == si => <set openended-answer=yes> {@ openended reset and route}
* <star> == no => <set openended-answer=no> {@ openended reset and route}
* <star> == n => <set openended-answer=no> {@ openended reset and route}
* <star> == nope => <set openended-answer=no> {@ openended reset and route}
- Is that a yes or a no? ^buttons("Yes!", "No!")

+ openended reset and route
- <set openended-type=none> {@ <get openended-next-trigger>}

// the stuff below is some gnarly code. enter with care //

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
