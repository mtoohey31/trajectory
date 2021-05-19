# Trajectory

A full-stack grade tracking app with end-to-end encryption. This project is currently in a very alpha state and is not yet ready for real-world use.

## Usage

To start the development version of the project, you only need to run three commands, assuming you have `docker-compose` installed:

```bash
git clone https://github.com/mtoohey31/trajectory
cd trajectory
docker-compose --file docker-compose.dev.yaml up
```

There currently appears to be some sort of issue though that results in the project being brought up correctly only around 1/4th of the time, the other 3/4ths the `/api` route is inaccessible and is redirected to the frontend. This isn't a huge problem since as soon as you get it to launch correctly once, you won't have to restart it until you're done working on the project since both the front and back ends have hot-reloading set up. If you know what the problem might be, please let me know!
