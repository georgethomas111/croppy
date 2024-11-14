# What is this?

This is the code for a worker written in rust which gets deployed on commit and merge. 

The intention is to make them work in a way an image api can be exposed that crops images.

# crop-rest

Now the image crop program main.rs exists here which should scale and crop images. 

# Mobile net

Mobile net helps with classification and the python program hellps acheive it.


# Deployment

Option 1

Create a Docker file which can run the python program which can classify the images.
Push it to dockerhub
Make the droplet pull the new image and rerun the droplet.

Option 2 

Push the python program to the droplet
Make it run locally.
Have a go program route requests to this program so that we don't burden the runtime.

