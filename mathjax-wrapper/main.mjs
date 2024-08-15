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
// When this script is started, it will initialize MathJax and then wait for the first request
// to come in.
//
// ### Incoming Packets
// An incoming packet is a variable-length sequence of bytes beginning with an unsigned 8-bit
// `IncomingPacketType` header, followed by an unsigned 64-bit integer called `PacketBodyLength`.
// The `PacketBodyLength` is the total length of the incoming packet less the length of the
// 8-bit header.
//
// ### Outgoing Packets
// An outgoing packet is a veriable-length sequence of bytes. But, unlike the aforementioned
// incoming packets, there is no type header. The type of an outgoing packet is implicit.
// In fact ther is no normal form of outgoing packet. See the packet descriptions below
// for an explanation.
//
// #### Exhaustive List of `IncomingPacketType`
// - 0: `ConversionRequest`
// - 1: `StylesheetRequest`
// - 2: `ShutdownRequest`
//
// #### `ConversionRequest` Body
// 1. `InputTeX`: The UTF-8 encoded string of TeX to be converted to SVG.
//
// #### `StylesheetRequest` Body
// This packet has no body. If the `PacketBodyLength` is non-zero the program will terminate
// with a non-zero error code.
//
// #### `ShutdownRequest` Body
// This packet has no body. If the `PacketBodyLength` is not zero the program will terminate
// with a non-zero error code.
//
// #### The `ConversionRequest` Packet
// Upon receiving a `ConversionRequest` packet, MathJax will be invoked and `InputTex` will
// be converted into an SVG. Then, an outgoing `ConversionResponse` packet will be sent to the
// calling program over stdout.
//
// ##### The Outgoing `ConversionResponse` Packet
// The `ConversionResponse` packet has the following form...
// 1. `OutputSVGLength`: An unsigned 64-bit integer equal to the length of the subsequent `OutputSVG` field.
// 2. `OutputSVG`: The UTF-8 encoded stirng of the SVG which MathJax emitted.
//
// #### The `StylesheetRequest` Packet
// Upon recieving a `StylesheetRequest` packet, the program will use MathJax to generate a
// a minimal CSS stylesheet containg all style rules needed to render the typography included
// in every `ConversionResponse` which has elapsed since the previous `StylesheetRequest` packet.
// The program will then emit an outgoing `StylesheetResponse` packet over stdout.
//
// ##### The Outgoing `StylesheetResponse` Packet
// The `StylesheetResponse` packet has the following form...
// The `StylesheetResponse` packet has the following form...
// 1. `OutputCSSLength`: An unsigned 64-bit integer equal to the length of the subsequent `OutputCSS` field.
// 2. `OutputCSS`: The UTF-8 encoded stirng of the CSS which MathJax emitted.
//
// #### The `ShutdownRequest` Packet
// Upon receiving a `ShutdownRequest` packet, the program will close stdin and stdout.
// Then the program will terminate with a success exit code of zero.
//
// There is no corresponding response packet for `ShutdownRequest` like there are for the other
// incoming packet types.


import * as mathjax from "mathjax";
import { read } from "./util.mjs";

// MathJax provides a variety of conversions, however yapl is concerned only with
// the TeX to SVG conversion.
const MathJax = await mathjax.init({ loader: { load: ['input/tex', 'output/svg'] } });

const tex = await read(process.stdin);

// MathJax wraps the SVG in and HTML container element.
// Since yapl generates pure SVGs, this container element must be removed.
const html_container = MathJax.tex2svg(tex, {display: false});
const svg = html_container.children[0];

// Remove the width and height attributes of the <svg> tag so that the SVG scales to fill.
// Then, on the Rust side, this SVG is wrapped in another SVG tag which will determine its size.
delete svg.attributes.height;
delete svg.attributes.width;



const args = process.argv.slice(2);
if (args.length == 1) { svg.attributes.preserveAspectRatio = args[0]; }

console.log(MathJax.startup.adaptor.outerHTML(svg));
