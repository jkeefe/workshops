+ hi
- Do you like dogs?

+ *
% do you like dogs
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* ${{result.action}} == smalltalk.confirmation.yes => <set dogvar=yes> Do you like cats?
* ${{result.action}} == smalltalk.confirmation.no => <set dogvar=no> Do you like cats?

+ *
% do you like cats
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* ${{result.action}} == smalltalk.confirmation.yes =>  <set catvar=yes> {@ answered <get dogvar> <get catvar>}
* ${{result.action}} == smalltalk.confirmation.no => <set catvar=no> {@ answered <get dogvar> <get catvar>}

+ answered yes yes
- Both can be great pets!

+ answered yes no
- Woof! Dogs are the best.

+ answered no yes
- Meow! Just remember, dogs have owners. Cats have servants.

+ answered no no
- Maybe you're more of a bird person.

+ *
$ GET https://api.api.ai/v1/query?v=20150910&query=<call>encode_uri <star></call>&lang=en&sessionId=<_platformId> {"headers":{"Content-Type":"application/json", "Authorization": "<bot apiai>"}}
* ${{result.fulfillment.speech}} != "" => ${{result.fulfillment.speech}} 
- Sorry, I have no idea what you just said.

> object encode_uri javascript
    return encodeURIComponent(args[0])
< object

! var apiai = Bearer putyourapikeyhere