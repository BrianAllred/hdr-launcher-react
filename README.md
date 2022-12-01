The HDR Launcher, this time written in the React framework with TypeScript with both a Switch and Desktop backend.
Switch backend runs as a skyline plugin using skyline-web.
Desktop application is an electron app for use with Ryujinx.
![launcher_main](https://user-images.githubusercontent.com/42820193/205082618-e6fbaf05-cced-4625-bbfb-372536f5f2aa.png)
![launcher_verify](https://user-images.githubusercontent.com/42820193/205082615-de3591f8-5054-4d26-98ff-706afe0f1159.png)
![launcher_logs](https://user-images.githubusercontent.com/42820193/205082612-b96e96c0-a93d-4519-a9e6-940d51c95fd7.png)

## Building

Use a package manager of your choice (npm, yarn, etc.) in order to install all dependencies

```bash
yarn install
```

## Usage

Just run `start` script to run as electron app.

```bash
yarn start
```
To package the electron app, and then compile the skyline plugin, use:
```bash
python3 build.py package <optionally, ip=0.0.0.0> <optionally, listen>
```

## Packaging

To generate the electron project package based on the OS you're running on, just run:

```bash
yarn package
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
