! var apiai = Bearer YourAPIaiClientAccessTokenGoesHere

+ (get started|hi|hello)
- Hi! What's your name? 

+ *
% hi whats your name
- Hi <formal>. Do you like dogs? <call>await_answer yesno dog-answer</call>

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

+ help
- I'm sorry you're having trouble!
- I'll try to get you some help!

// the stuff below is some gnarly code. enter with care //

+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* <get openended-type> == yesno => {@ handle yesno ${{result.action}} }
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
* ${{result.fulfillment.speech}} != "" => ${{result.fulfillment.speech}} 
- Sorry, I have no idea what you just said.

+ handle yesno *
* <star> == smalltalk.confirmation.yes => <set openended-answer=yes> {@ openended reset and route}
* <star> == smalltalk.confirmation.no => <set openended-answer=no> {@ openended reset and route}
- Is that a yes or a no? ^buttons("Yes!", "No!")

+ openended reset and route
- Got to here <set openended-type=none> {@ <get openended-next-trigger>}

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
