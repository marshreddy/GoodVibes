# GoodVibes


## A. Set up Rust and boot the rust server

To install rust:
		$ curl https://sh.rustup.rs -sSf | sh

create a new project called hello, that listens for the Alexa post and responds accordingly.
		$ cargo new hello
		$ cd hello

main.rs contains all the code the RUST server needs to listen out for the Alexa prompt, respond appropriately, and trigger posts to the offerzen satellite and the relevant slack channel

 to run the hello listener:
 		$cargo run
When you make edits to the main.rs file, ensure that you cntrl-c in the console to quite rust, and restart it using $ cargo run.


## B. Run ngrok

install ngrok - https://dashboard.ngrok.com/get-started

 Download the binary file and unzip it: 
 		$ unzip /path/to/ngrok.zip

 Copy the prompt from step 3 of the get started page to connect your account (it is already populated with your unique key)
 
		$./ngrok authtoken 6vbXx7z5PcTCtTzuJTrUp_39rEdmpyRqK6Qfd8z38uB

To start ngrok on port 443:

		$./ngrok http 443

Go to localhost:4040 to view the ngrok dashboard, where you can inspect all requests. 
Take note of the https:/... forwarding address in the terminal. You will use this to set up Alexa.

## C. Invoke the Alexa skill and intent

In amazon developer console, create new custom skill and add the happy sat.json file via the JSON editor. 

Happy sat is compatible with the following alexa prompts:

    Alexa ask happy sat to :

        send us alpha brain waves
        send good vibes to our makers
        send good vibes to our maker space
        send good vibes to our make day
        please send us good vibes
        send good vibes to make
        send good vibes
        send us good vibes


## D. Connect Alexa to ngrok

Under the endpoint tab Use your forwarding addresses in the ngrok terminal (Step B) to configure your amazon alexa skill to point to your server. 
Select the https button, then in the default region field, input your relevant ngrok address, appended as follows, so that the server knows to handle this in a unique way.
[ngrok forwarding address]/alexa
https://a4a512d8.ngrok.io/alexa


Save your endpoints and go back to invocation tab.
Click on save model and build model.
Once the build has successfully completed, click on Test, and test your skill using the phrases above. Provided your RUST server is running, you should recieve the response "Good vibes are coming!".

## E. Configure Slack
ON APi.slack.com, creat a new app.
Enable incoming webhooks.
You will need to request admin approval from your slack workspace admin to be able to add this app and assign it to a channel.

Once admin approval is granted, you should be able to add new webhook to workspace, under App Settings > Features > Incoming Webhooks.

Click on Add New Webhook to Workspace. You will be directed to a new page where you can select the channel you would like to post to.

Select your channel, and you will be returned to incoming webhooks page. Copy the webhook URL for use in your external applciation, or use the Sample curl request.

in main.rs, update the address for this slack channel using the URL you copied above.


## User's Experience
 - try to explain in detail what the user can expect to see at each step. Would have liked to include screenshots, but ran out of time.
 - i would have liked to move the strings that point to the josn files, and urls of various endpoints to the top of the main.rs file, but because I'm not so familiar with RUST, I couldn't do this without breaking stuff.


## Familiarity
Alexa Skills - none

ngrok - none

Rust - none

Slack Webhooks - some familiarity - connecting existing webhooks. 
