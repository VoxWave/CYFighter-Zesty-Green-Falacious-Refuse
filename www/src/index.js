const main = () => {
    let canvas = document.getElementById("game");
    if (canvas.getContext) {
        start(canvas.getContext('2d'));
    }
};

const start = (ctx) => {
    window.addEventListener("gamepadconnected", function(e) {
        console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
          e.gamepad.index, e.gamepad.id,
          e.gamepad.buttons.length, e.gamepad.axes.length);
    });
    window.addEventListener("gamepaddisconnected", function(e) {
        console.log("Gamepad disconnected from index %d: %s",
          e.gamepad.index, e.gamepad.id);
      });
    ctx.fillStyle = 'rgb(0, 200, 0)';
    ctx.fillRect(10, 10, 50, 50);
    ctx.fillStyle = 'rgb(0, 0, 200)';
    ctx.fillRect(10, 10, 50, 50);
};