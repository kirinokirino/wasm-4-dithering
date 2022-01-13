
The [Floyd-Steinberg dithering](https://thecodingtrain.com/CodingChallenges/090-floyd-steinberg-dithering.html)
test cart. It takes a source image (which has 255 possible colors, see input_image.png) and tries to represent it
with just 4 colors that wasm-4 provides. The results are not that great due to lack of resolution, 
the normal 4-color image looks better (see botan).
It works better for gradual change (see lines), maybe it might even work if you choose red-green-blue palette.
Probably there is an easier way to do that though.

from the target/wasm32-unknown-unknown/release folder:
```cargo build --release; stat cart.wasm | rg Size ; w4 run ./cart.wasm --no-open --no-qr```

src/image_preproc.rs is a file with witch I converted a png into a string that I copied
into the source code, and also generated a 4 colour version just to check how the end 
result might look like.

You can find Botan ch. [here](https://www.youtube.com/channel/UCUKD-uaobj9jiqB-VXt71mA).