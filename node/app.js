const rustWasm = require('../pkg/ssvm_nodejs_starter_lib.js');
const { createCanvas } = require('canvas')
const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  var canvasElement = createCanvas(rustWasm.get_width(), rustWasm.get_height(), "png");
  const canvasContext = canvasElement.getContext('2d');
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );
  var start = process.hrtime()
  const imageDataArray = rustWasm.render();
  var end = process.hrtime(start);
  console.info('Execution time (hr): %ds %dms', end[0], end[1] / 1000000)
  // Set the values to the canvas image data
  canvasImageData.data.set(imageDataArray);

  // Clear the canvas
  canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

  // Place the new generated checkerboard onto the canvas
  canvasContext.putImageData(canvasImageData, 0, 0);

  console.log(canvasElement);
  res.setHeader('Content-Type', 'image/png');
  res.end(canvasElement.toBuffer());
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
