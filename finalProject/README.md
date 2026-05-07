### This is the README file for the Final Project!

It includes:

## -How to run the code

## -Summary of design

## -Summary of experiments

## Tool Use Disclosure


# How to run the code
The code is separated into two paragraph comments. So if you wanted to look at one over the other, you would need to comment one out.

# Summary of design
The code will have a generator that generates tasks for the dispatcher to recieve and send to the workers to work on. While the workers do their tasks, a monitor thread will take data on how many workers are there and how much CPU is being used. After a worker finished their task, the data on how long it took or what they worked on it send to the main thread and put into a struct to hold the information. Once all tasks are done, signals are sent and eventually the workers are terminated and the resulting data from the workers and monitor thread are printed on the screen. Two codes were made: one was FIFO optimized and one was optimized to have queues for each type of task (CPU or IO).

# Summary of experiments
To my surprise, the optimized code didnt do much better than the FIFO optimized code. While it did do better than the FIFO optimized code by a couple of seconds in the numerous tests that were taken, it came scarily close to the FIFO code.

# Tool Use Disclosure
I used AI and numerous websites to help me with this project, and while I used AI to help me with it, I asked for advice on how to proceed with certain parts and examples unrelated to the project to better illuminate how certain functions/syntax was used and/or could be used to aid me in the project. Advice I didn't accept was the AI trying to give me huge chunks of code. Because it gave me a lot I had to use the comments given in certain spots so I could digest what was going on in the more complex areas when it gave me unrelated examples. Examples would be when dealing with numeround channels sending information to different threads.

