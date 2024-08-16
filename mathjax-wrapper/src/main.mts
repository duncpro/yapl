// # TeX to SVG Conversion Service
//
// This node script provides a TeX to SVG conversion service. This service communicates with the
// invoking program through stdin/stdout. To avoid repeatedly incurring the cost of Node.js VM 
// startup, a simple protocol is defined, allowing for the invoking program to keep the process 
// alive for as long there might be another TeX string to convert.
//
// Each time a TeX string is encountered, the calling program sends a request through stdin, and
// this script will respond with the rendered SVG via stdout, and then wait for the next request
// to come in.
//
// ## Protocol
//
// ### Startup
// When this script is started, it will initialize MathJax, and then wait for the first request
// to come in over stdin.
//
// ### Endianness
// All integer types which appear in the protocol are little endian.
//
// ### Incoming Packets
// An incoming packet is a variable-length sequence of bytes beginning with an unsigned 32-bit
// `IncomingPacketType` header, followed by the so-called packet body. The shape of the body
// is dependent on the packet type.
//
// ### Outgoing Packets
// An outgoing packet is a veriable-length sequence of bytes. But, unlike the aforementioned
// incoming packets, there is no type header. The type of an outgoing packet is implicit.
//
// #### Exhaustive List of `IncomingPacketType`
// - 0: `ConversionRequest`
// - 1: `StylesheetRequest`
// - 2: `ShutdownRequest`
//
// #### `ConversionRequest` Body
// 1. `PreserveAspectRatioLength`: An unsigned 32-bit integer equal to the length of the subsequent 
//                                 field `PreserveAspectRatio`. Or, if no `preserveAspectRatio`
//                                 attribute is desired set this value equal to zero.
//
// 2. `PreserveAspectRatio`:       The desired value of the SVG preserveAspectRatio attribute (if any).
//                                 If non-empty string a, preserveAspectRatio attribute will be included
//                                 in the generated SVG with this value.
//
// 3. `InputTeXLength`:            An unsigned 32-bit integer equal to the length of the 
//                                 subsequent field `InputTeX`.
//
// 4. `InputTeX`:                  The UTF-8 encoded string of TeX to be converted to SVG.
//
// #### `StylesheetRequest` Body
// This packet has no body. 
//
// #### `ShutdownRequest` Body
// This packet has no body. 
//
// #### The `ConversionRequest` Packet
// Upon receiving a `ConversionRequest` packet, MathJax will be invoked and `InputTex` will
// be converted into an SVG. Then, an outgoing `ConversionResponse` packet will be sent to the
// calling program over stdout.
//
// ##### The Outgoing `ConversionResponse` Packet
// The `ConversionResponse` packet has the following form...
// 1. `OutputSVGLength`: An unsigned 64-bit integer equal to the length of the
//                       subsequent `OutputSVG` field.
//
// 2. `OutputSVG`:       The UTF-8 encoded string of the SVG which MathJax emitted.
//
// #### The `StylesheetRequest` Packet
// Upon recieving a `StylesheetRequest` packet, the program will use MathJax to generate a
// a minimal CSS stylesheet containg all style rules needed to render the typography included
// in every `ConversionResponse` which has elapsed since the last `StylesheetRequest` packet.
// The program will then emit an outgoing `StylesheetResponse` packet over stdout.
//
// ##### The Outgoing `StylesheetResponse` Packet
// The `StylesheetResponse` packet has the following form...
// 1. `OutputCSSLength`: An unsigned 32-bit integer equal to the length of the
//                       subsequent `OutputCSS` field.
//
// 2. `OutputCSS`:       The UTF-8 encoded stirng of the CSS which MathJax emitted.
//
// #### The `ShutdownRequest` Packet
// Upon receiving a `ShutdownRequest` packet, the program will close stdin and stdout.
// Then the program will terminate with a exit code of zero.
//
// There is no corresponding response packet for `ShutdownRequest`.
import { readn, write, end } from "./io.mjs";
import { renderSVG, dumpCSS } from "./render.mjs";

async function handleConversionRequest() {
  const PreserveAspectRatioLength = (await readn(process.stdin, 4)).readUInt32LE();
  const PreserveAspectRatio = (await readn(process.stdin, PreserveAspectRatioLength))
    .toString('utf-8');
  const InputTeXLength = (await readn(process.stdin, 4)).readUInt32LE();
  const InputTeX = (await readn(process.stdin, InputTeXLength)).toString('utf-8');
  const OutputSVG = Buffer.from(renderSVG(InputTeX, PreserveAspectRatio), 'utf-8');
  const OutputSVGLength = Buffer.alloc(4);
  OutputSVGLength.writeUInt32LE(OutputSVG.byteLength);
  await write(OutputSVGLength, process.stdout);
  await write(OutputSVG, process.stdout);
}

async function handleStylesheetRequest() {
  const OutputCSS = Buffer.from(dumpCSS(), 'utf-8');
  const OutputCSSLength = Buffer.alloc(4);
  OutputCSSLength.writeUInt32LE(OutputCSS.byteLength);
  await write(OutputCSSLength, process.stdout);
  await write(OutputCSS, process.stdout);
}


// # Request Loop

let open = true;
while (open) {
  const reqid = (await readn(process.stdin, 4)).readUInt32LE();
  switch (reqid) {
    case 0: // ConversionRequest
      await handleConversionRequest();
      break;
    case 1: // StylesheetRequest
      await handleStylesheetRequest();
      break;
    case 2: // ShutdownRequest
      process.exitCode = 0;
      open = false;
      break;
    default:
      process.exitCode = 1;
      open = false;
  }
}

await end(process.stdout);
