fetch('./rustycheckers.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes, {
    // NB: Rust's WASM toolchain by default expects imports to come from `env` object.
    env: {
        notify_piecemoved: (fX, fY, tX, tY) => {
            console.log("A piece moved from (" + fX + "," + fY + ") to (" + tX + "," + tY + ")");
        },
        notify_piececrowned: (x, y) => {
            console.log("A piece was crowned at (" + x + "," + y + ")");
        },
    }
})).then(results => {
    instance = results.instance;

    console.log("At start, current turn is " + instance.exports.get_current_turn());
    // Q14: Is the runtime error sometime after this point due to the `todo!()`s in `game.rs`?
    // Q15: Is there a way to enable source maps from the compiled WASM to the original Rust source?
    let piece = instance.exports.get_piece(0, 7);

    let res = instance.exports.move_piece(0, 5, 1, 4); // B
    console.log("First move result: " + res);
    console.log("Turn after move: " + instance.exports.get_current_turn());

    let bad = instance.exports.move_piece(1, 4, 2, 3); // illegal move
    console.log("Illegal move result: " + bad);
    console.log("Turn after illegal move: " + instance.exports.get_current_turn());
}).catch(console.error);
