# radscanner

You've somehow landed on my pretty-much-alpha version of a Minecraft server scanner.
This is essentially my rust learning Project.

# Usage
```
git clone https://github.com/radstevee/radscanner.git
```

Configure config.toml to your liking.

Put the list of IPs into your configured `input_file` (seperated by a newline)

You need a MongoDB instance running, in this case it comes with the docker-compose.


## Docker
Change the `docker-compose.yml`: 
```
services:
    scanner:
        volumes:
            - ./config.toml:/usr/src/radscanner/config.toml:ro
        build: .
        networks:
            - internal
    db:
        image: mongo
        restart: always
        environment:
            MONGO_INITDB_DATABASE: radscanner
            MONGO_INITDB_ROOT_USERNAME: root
            MONGO_INITDB_ROOT_PASSWORD: password # change this, no mongo formatting
        networks:
             - internal
    frontend:
        image: mongo-express
        restart: always
        ports:
           - 8081:8081
        environment:
            # change these to your own values
            ME_CONFIG_BASICAUTH_USERNAME: username # username for website authentication
            ME_CONFIG_BASICAUTH_PASSWORD: password # no mongo formatting
            ME_CONFIG_MONGODB_ENABLE_ADMIN: true
            ME_CONFIG_MONGODB_ADMINUSERNAME: root
            ME_CONFIG_MONGODB_ADMINPASSWORD: password # no mongo formatting
            ME_CONFIG_MONGODB_URL: mongodb://root:password@db:27017/ # *with* mongo formatting
        networks:
            - web
            - internal
```

(technically it doesn't actually have to be changed since they are on an internal network, but please do it anyways.)

Change the `config.toml`: 

```
mongo_hostname = "db" # hostname of the mongodb instance (leave this like this if using docker-compose)
mongo_root_username = "root"
mongo_root_passwd = "password" # format this accordingly

mongo_newuser_passwd = "password" # no need to format accordingly, password for a *new, created* user
```

Whenever I say "format accordingly", follow [this guide](https://www.mongodb.com/docs/manual/reference/connection-string/#std-label-connections-standard-connection-string-format) and also [this guide](https://www.w3schools.com/tags/ref_urlencode.asp?_sm_au_=iVVDMg0TSmrMV6Dm).


Then run
```
docker-compose up
```

Note that this might take up to ~5 minutes to build, depending on your machine and internet connection.

# Troubleshooting
- Check if the MongoDB formatting is correct. This is one of the most common mistakes.

- Look at the logs with `docker-compose logs`

- Submit an Issue, I'll have a look.

- Send me a PM on Discord (`@radstevee`)