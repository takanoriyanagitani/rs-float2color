(() => {
  const name2wasm = (name) => {
    return Promise.resolve(name)
      .then(fetch)
      .then((res) => WebAssembly.instantiateStreaming(res));
  };

  const cnvs = document.getElementById("floats");
  const ctx = cnvs.getContext("2d");

  return Promise.resolve("rs_float2color.wasm")
    .then(name2wasm)
    .then((wasm) => {
      const {
        module,
        instance,
      } = wasm || {};
      const {
        memory,

        i_ptr,
        i_allocate,
        i_zero,

        o_ptr,
        o_allocate,

        convert_all_simple_le,
      } = instance?.exports || {};

      const isz = 320 * 180 * 4;
      const icap = i_allocate(isz);
      i_zero();
      const ocap = o_allocate(isz);

      const started = Date.now();

      const iview = new DataView(memory?.buffer, i_ptr(), isz);

      for (let i = 0; i < (isz >> 2); i++) {
        const rcp = 1.0 / (isz >> 2);
        const f = rcp * i;
        const p = 2.0 * Math.PI;
        const x = f * p;
        const y = 0.5 + 0.5 * Math.sin(x);
        iview.setFloat32(
          i << 2,
          y,
          true,
        );
      }

      const oview = new Uint8Array(memory?.buffer, o_ptr(), isz);

      const converted = convert_all_simple_le();
      const wasmTime = Date.now() - started;
      console.info({ wasmTime });

      const dat = ctx.getImageData(0, 0, 320, 180);
      const arr = dat.data;
      const itemCount = arr.length << 2;

      arr.set(oview);

      ctx.putImageData(dat, 0, 0);
      const all = Date.now() - started;
      console.info({ all });
    })
    .catch(console.warn);
})();
