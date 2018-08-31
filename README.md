# GoodVibes


2 - User's Experience
 - try to explain, in detail what the user can expect to see at each step.


3 - Familiarity
Alexa Skills - none
ngrok - none
Rust - none
Slack Webhooks - some familiarity

4 - Alexa Skill
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

In amazon developer console, create new custom skill and add this JSON file via the JSON editor. Save this setup. Later, you will set Endpoint to point to the ngrok forwarding address in step 5.


5. Build a RUST Webserver

install rust:
$ curl https://sh.rustup.rs -sSf | sh

create a new project called hello, that listensfor the alexa post and responds accordingly.

$ cargo new hello
$ cd hello

in main.rs, create a listener on 127.0.0.1:443
 respond with a http 200 header, and a JSON packet to tell Alexa that everything's ok.

 to run the hello listener:
 $cargo run


 install ngrok - https://dashboard.ngrok.com/get-started

 Download the binary file and unzip it: 
 $ unzip /path/to/ngrok.zip

 Copy the promt from step 3 of this setup page to connect your account (it is already populated with your unique key)
$./ngrok authtoken 6vbXx7z5PcTCtTzuJTrUp_39rEdmpyRqK6Qfd8z38uB

To start ngrok on port 443:

$./ngrok http 443

Go to localhost:4040 to view the ngrok dashboard, where you can inspect all requests. 

6 - Connect Alexa to web server via ngrok
Use your forwarding addresses in the ngrok terminal to configure Amazon Alexa to point to your server. Append the forwarding address as follows, so that the server knows to handle this in a unique way.

https://a4a512d8.ngrok.io/alexa


