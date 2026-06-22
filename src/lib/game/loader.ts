export class GameLoader {
  private instance: WebAssembly.Instance | null = null;
  private gl: WebGL2RenderingContext;
  private memory: WebAssembly.Memory | null = null;
  private websocket: WebSocket | null = null;

  constructor(gl: WebGL2RenderingContext) {
    this.gl = gl;
  }

  setWebsocket(websocket: WebSocket) {
    this.websocket = websocket;
  }

  async load(wasmUrl: string) {
    const importObject = {
      env: {
        gl_clearColor: (r: number, g: number, b: number, a: number) => {
          this.gl.clearColor(r, g, b, a);
        },
        gl_clear: (mask: number) => {
          this.gl.clear(mask);
        },
        gl_viewport: (x: number, y: number, width: number, height: number) => {
          this.gl.viewport(x, y, width, height);
        },
        js_log: (ptr: number, len: number) => {
          if (!this.memory) return;
          const buf = new Uint8Array(this.memory.buffer, ptr, len);
          const msg = new TextDecoder().decode(buf);
          console.log(`[Zig] ${msg}`);
        },
        send_message: (ptr: number, len: number) => {
          if(!this.memory) return;
          const buf = new Uint8Array(this.memory.buffer, ptr, len);
          if(!this.websocket) return;
          this.websocket.send(buf);
        }
      }
    };

    const { instance } = await WebAssembly.instantiateStreaming(fetch(wasmUrl), importObject);
    this.instance = instance;
    this.memory = instance.exports.memory as WebAssembly.Memory;

    return instance.exports;
  }
}
