Project for compiling code safely using docker containers.

The original idea was to spin the project up using docker-compose in the root directory. In this configurations the API would be handed the socket from the host machine, which enables it to create docker containers on the host machine. The problem this causes, is when I am trying to dynamically map folders to the new conainters. The new containers are created using the host's socket, and the files are no longer accesible. Of course I could map the api to a docker volume and share it between the containers, but this makes the project harder to run without docker, which I tink is even worse. There are probably workarounds for this, but I don't want to waste more time on it.

To run the api, navigate to backend/code-execution-engine-api and run `cargo run`

To run the frontend, navigate to frontend/code-execution-frontend and run `npm install && npm start`

the frontend is accesible at `localhost:3000` Write some code in the white field above the run button, choose the language from the drop-down, and hit run. The output should be visible below the run button. Pardon the UI, not a lot of effort went into it. Currently only python, Nodejs and Go are supported. I have only tested simple hello world applications.

The backend is written in Rust using the Rocket framework. This api handles the requests, matches the given language specified, and configures the run commands to compile and run the code. The dockerfile which executes the code is in the engine folder in the api folder. The image is also availible at `dockerhub.com/sigmundgranaas/code-execution-engine` The size of this image is huge, because I am importing a lot of compilers. Most of them are unused, but expanding language support is extyremely easy, I just need to add the correct field to the LangMap struct. The API copies a script into a volume which is accesible by the docker container and tells it to execute this script. The script takes two inputs, the filename, and code compilation instructions. The output is redirected to output.txt, which is watched by the watch function in the api. When the docker container has exited and the output file is created, the result is sent back to the client.

The easiest way to build the image is in the docker-compose file in the engine folder. Just run `docker-compose build`

The api is asynchronous, but the code execution is not. If you send requests to quickly, some of them will fail. This is because I am not handling the code execution in separate folders. The execution of code crashes if a folder with the name "test" already exists. This could be improved upon, but it's not that important.
