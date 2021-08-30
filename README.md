# Trajectory

A full stack grade tracking app with end-to-end encryption. This project is currently in a very alpha state and is not yet ready for real-world use.

## Usage

### Development

To start the development version of the project, you only need to run three commands, assuming you have `docker-compose` installed:

```sh
git clone https://github.com/mtoohey31/trajectory
cd trajectory
docker-compose --file docker-compose.dev.yaml up
```

### Production

To start the production version of the project, begin by running the following commands:

```sh
git clone https://github.com/mtoohey31/trajectory
cd trajectory
cp Caddyfile.example Caddyfile
```

Next, edit the `Caddyfile` to replace `<hostname>` with the hostname you will be serving the app on. Caddy should automatically negotiate TLS certificates once the app is started, but if you run into issues, visit [Caddy's documentation](https://caddyserver.com/docs/quick-starts/https).

Then start, the app by running:

```sh
docker-compose --file docker-compose.yaml up
```

Assuming your DNS and ports are set up correctly the app should now be available on the host you specified.
