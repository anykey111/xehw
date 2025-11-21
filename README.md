
`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

## Collaborative Coding

This project supports networked collaborative coding. To use it:

1.  Start the broadcast server:
    ```bash
    cargo run --release --bin server
    ```
2.  Run the application (multiple instances):
    ```bash
    cargo run --release
    ```
3.  In the app, go to **Network > Connection...**
4.  Enter the server URL (default: `ws://localhost:8080`) and click **Connect**.
5.  Edits to the code and agent activities will be synchronized between connected peers.

Screenshots:

![xehimg.png](imgs/xehimg.png)

![doomfire.gif](imgs/doomfire.gif)

![rollback.gif](imgs/rollback.gif)

![gbtile.gif](imgs/gbtile.gif)

![reversestep.gif](imgs/reversestep.gif)

![freeze.gif](imgs/freeze.gif)
