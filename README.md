# radscanner

You've somehow landed on my pretty-much-alpha version of a Minecraft server scanner.
This is essentially my rust learning Project. Capable of scanning ~10000 IPs/h (on my 100Mbit/s connection).

## Usage

```sh
git clone https://github.com/radstevee/radscanner.git
```

Configure config.toml to your liking.

Put the list of IPs into your configured `input_file` (seperated by a newline)

You need a MongoDB instance running, in this case it comes with the docker-compose.

## Docker

First up, create the network:

```sh
docker network create web
```

Then change the `docker-compose.yml`:

Change the `docker-compose.yml`:

```yaml
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

```toml
mongo_hostname = "db" # hostname of the mongodb instance (leave this like this if using docker-compose)
mongo_root_username = "root"
mongo_root_passwd = "password" # format this accordingly

mongo_newuser_passwd = "password" # no need to format accordingly, password for a *new, created* user
```

You can format your passwords and usernames [here](https://www.charset.org/url-encode).

Then run

```sh
docker-compose up -d db
```

Wait a couple of seconds *after* it has finished pulling (30-60 seconds on first run)

And then start up everything else with

```sh
docker-compose up -d
```

Note that this might take up to ~5 minutes to build, depending on your machine and internet connection.

## Usage of Mongo express

Navigate to `localhost:8081` (of course replace localhost with the IP and 8081 with a different port if you changed it).

Log in with the Basic auth credentials from the `docker-compose.yml`.

Navigate to radscanner/servers

And there you go! Found servers will be saved in there.

Extra note: If you're looking for the LiveOverflow Let's Play, use this advanced search query:

```json
{
   "version": "Paper 1.19.2",
   "playerdata.max": { "$eq": 50 },
   "playerdata.online": { "$gte": 40 },
   "motd": "A Minecraft Server"
}
```

With the projection just being `{}`.

And that should query the DB for the LiveOverflow Let's Play if it has already been found.

## Todo

- [ ] *Custom* Web UI
- [ ] Banner output instead of Log-spam
- [ ] Progress in percent
- [ ] Logging in to the servers (via matdoesdev/azalea)

## Troubleshooting

- Check if the MongoDB formatting is correct. This is one of the most common mistakes.

- Look at the logs with `docker-compose logs`

- Submit an Issue, I'll have a look.

- Send me a PM on Discord (`@radstevee`)
- Run

```sh
docker compose up --build -d
```
