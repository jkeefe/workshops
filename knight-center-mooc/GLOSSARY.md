**Airtable** A spreadsheet service we're using to collect survey data.

**Alexa** A voice-interaction service from Amazon. Note that "Alexa" is the core software, which can exist inside many different pieces of hardware, such as Amazon's Echo Dot.

**API** Short for "application programming interface," this is a way for computers to exchange information. For example, in this class, we connect Dexter to services such as Airtable and Dialogflow using the APIs provided by those services.

**API key** A code that gives you access to an API. You should keep this information secret, because revealing it could allow others to use an API as you.

**app** _In Facebook_ To connect Dexter bots to Facebook, we establish a link between Dexter and a Facebook *app*, or application. Apps live on a Facebook *page* which is added to your Facebook account. Visually, you might think of the setup like this:

    Facebook Account → Facebook Page → Facebook App ⟷ Dexter

We will add both a **page** and an **app** to your Facebook profile in this class. 

**bot** _In the context of this class_ A software program that responds to humans like a human, either through text or speech. It's controlled by the **bot script** we write in Dexter, and delivered via a **platform** such as Facebook Messenger or SMS text.

**bot script** _In Dexter_ The collection of **triggers**, **responses** and bot conversation we write to run the bot.

**catchall** _In Dexter_ A **trigger** that matches to any human input, designated by an asterisk: `*`. Used to provide a response for when the user says something that doesn't match any other trigger. 

**deploy** _In Dexter_ This establishes the connection between a Dexter bot and a platform. Once a bot is deployed, changes made to the bot script and the published with the **publish topic** button will be reflected in the bot. 

**Dexter**  The software platform we're using to write instructions for our bots. Usually the end user does not interact directly with Dexter. Instead, Dexter connects with **platforms** such as Facebook Messenger, Twilio, and Amazon Alexa, where the end user actually talks with our bots.

**Dialogflow** A natural language processing service, owned by Google. Formerly called "api.ai."

**Facebook** A global social media and advertising platform.

**intent**  _In natural language processing_ When a human writes or speaks, natural language processing can be used to detect the **intent** of the user to help figure out how to handle the statement -- often routing the issue to another part of the software. For example, the **intent** of _Book my flight to Paris_ might be "plan travel," while the intent of _When is my flight to Paris_ might be "search calendar." In this class, words such as "yes," "si," "yup," and "absolutely" are interpreted by Dialogflow as the intent: `smalltalk.confirmation.yes`.

**Messenger** And instant messaging service of Facebook.

**natural language processing** Software used to process human language, either written or spoken, so that it can be used by a computer. We're using a system called Dialogflow to detect the intent of a person using our bots. 

**NLP** Short for Natural Language Processing.

**page** _In Facebook_ If you use Facebook, you're familiar with your own "page," which has your picture, a header image, and your own Messenger window. To your profile you can also add a page, which have their own profile image, header image, and Messenger window. This is common for setting up a presence for businesses or interest groups. The bots we make for Facebook Messenger work in the Messenger window of this kind of a Facebook page (they don't work on your personal profile page). Additionally, for Dexter to work with the page, the page must also have a Facebook *App*. 

**platform** A service that links our bot script to our end users. In this class we're using using Twilio (for SMS texting), Amazon Alexa and Facebook messenger as our platforms.

**publish topic** _Button in Dexter_ After making changes to your bot script, pressing this button makes those changes live on any connected (**deployed**) platform. It also ensures your changes are saved properly to the Dexter platform.

**response** _In Dexter_ The bot's reaction when a human phrase matches the associated **trigger**. Responses begin with `-` and can contain any characters or punctuation, and they can also contain other special characters that route the user to another trigger, serve up buttons, display images and so on.

**reset** _Button in Dexter_ Clears any user information stored as you test your bot, making it as though a brand new user is using the bot. Only affects the test phone in the Dexter dashboard.

**RiveScript** An open-source programming language designed for making chatbots. It's the foundation for the commands we write in **Dexter**, though Dexter has added more features. For example **shortcodes** such as `^buttons` aren't part of RiveScript. More on RiveScript here: https://www.rivescript.com/about

**shortcode** _In Dexter_ A bit of code that performs a formatting function, such as adding buttons or images to our bot experience. Shortcodes begin with a `^`, such as: `^buttons(yes, no, nevermind)`. You can find shortcodes in the "Insert" menu.

**skill** _When referring to Alexa_ An Alexa "skill" is software that a user can add to Alexa. You can think of a skill as Amazon's version of an "app" in the Apple App Store or Google Play store. Much of Alexa's functionality, like, "Alexa, sing me a song," are built-in. Others, such as "Alexa, play Jeopardy," don't work unless you first add the skill.

**SMS** Stands for "short message service" and is a standard mobile phones use to exchange messages, also known as text messages. In this class, we're using the Twilio platform as the bridge between our Dexter bot and the **SMS** texting system.

**template** _In Dexter_ A bit of code, slightly more complex than a **shortcode**, that we can use to display nicely-formatted interactions and carousels in Facebook Messenger. Available from the "Insert" menu.

**token** A code that gives you access to an API or online platform. You should keep this information secret, because revealing it could allow others to use an API or platform as you.

**topic** _In Dexter_ Bots can have many topics, which can allow bot writers to create many different storylines. In this class, we're keeping things simple and just working out of the `default` topic.

**trigger** _In Dexter_ A word or phrase in our bot script that, when it matches something the human says, performs the associated **response**. In Dexter, triggers begin with a `+`, must not contain punctuation, and must consist of lower-case letters. Triggers may contain special characters such as `*`, which will match any word.

**Twilio** A platform we're using to connect our Dexter bots to the global **SMS** mobile-phone texting system.

**variable** A way to store values in any programming language, including **Dexter**. In Dexter, you store values using `set` and retrieve values using `get`. For example, you can store the word "collie" in the "dog" variable using `<set dog=collie>`. Later, in the script, the code `<get dog>` would be replaced by the word "collie." Read more here: http://docs.rundexter.com/writing/bot/data/






