! var apiai = Bearer YourAPIaiClientAccessTokenGoesHere

+ (get started|hi|hello)
- Welcome to my bot. <send>If you're using this script, you can use this as a starting point. <send>Usually best to start with some buttons.

+ help
- I'm sorry you're having trouble! I should be offering you options here.
- I'll try to get you some help!

// the stuff below is some gnarly code. enter with care //

+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* ${{result.action}} == smalltalk.agent.can_you_help => {@ help}
* ${{result.fulfillment.speech}} != "" => ${{result.fulfillment.speech}} 
- Sorry, I have no idea what you just said.

> object encode_uri javascript
    return encodeURIComponent(args[0])
< object