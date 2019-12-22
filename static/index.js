import(/* webpackChunkName: "ghash-wasm" */"../pkg").then(module => {
    // Now, MD4 and MD5 does not work.
    const input_form = document.getElementById('input')
    const output_form = document.getElementById('digest')
    function md2() {
        let input_string = (new TextEncoder).encode(input_form.value)
        let md2_digest = new module.Md2Ctx(input_string).digest()
        output_form.value = md2_digest
    }
    input_form.onload = md2()
    input_form.oninput = function() {
        md2()
    }
})
