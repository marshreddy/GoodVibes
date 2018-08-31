# GoodVibes


##A. Set up Rust and boot the rust server

install rust:
		$ curl https://sh.rustup.rs -sSf | sh

create a new project called hello, that listens for the Alexa post and responds accordingly.
		$ cargo new hello
		$ cd hello

main.rs contains all the code the RUST server needs to listen out for the Alexa prompt, respond appropriately, and trigger posts to the offerzen satellite and the relevant slack channel

 to run the hello listener:
 		$cargo run


##B. Run ngrok

install ngrok - https://dashboard.ngrok.com/get-started

 Download the binary file and unzip it: 
 $ unzip /path/to/ngrok.zip

 Copy the prompt from step 3 of the get started page to connect your account (it is already populated with your unique key)
$./ngrok authtoken 6vbXx7z5PcTCtTzuJTrUp_39rEdmpyRqK6Qfd8z38uB

To start ngrok on port 443:

$./ngrok http 443

Go to localhost:4040 to view the ngrok dashboard, where you can inspect all requests. 

## C. Invoke the Alexa skill and intent

In amazon developer console, create new custom skill and add this JSON file via the JSON editor. Save this setup. Later, you will set Endpoint to point to the ngrok forwarding address in step 5.

Called happy sat. 
Use the following requests:

    Alexa ask happy sat to :

        send us alpha brain waves
        send good vibes to our makers
        send good vibes to our maker space
        send good vibes to our make day
        please send us good vibes
        send good vibes to make
        send good vibes
        send us good vibes


##D. Connect Alexa to ngrok

Use your forwarding addresses in the ngrok terminal to configure Amazon Alexa to point to your server, under Endpoints in Alexa Skills . Append the forwarding address as follows, so that the server knows to handle this in a unique way.

https://a4a512d8.ngrok.io/alexa

##E. Configure Slack
ON APi.slack.com, creat a new app.
Enable incoming webhooks.
You will need to request admin approval from your slack workspace admin to be able to add this app and assign it to a channel.

Once admin approval is granted, you should be able to add new webhook to workspace, under App Settings > Features > Incoming Webhooks.

Click on Add New Webhook to Workspace. You will be directed to a new page where you can select the channel you would like to post to.

Select your channel, and you will be returned to incoming webhooks page. Copy the webhook URL for use in your external applciation, or use the Sample curl request.


##User's Experience
 - try to explain in detail what the user can expect to see at each step. I could add more detail and be a bit clearer, but I've spent more than four hours on it.


##Familiarity
Alexa Skills - none
ngrok - none
Rust - none
Slack Webhooks - some familiarity - connecting existing webhooks. 
