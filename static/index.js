import(/* webpackChunkName: "ghash-wasm" */"../pkg").then(module => {
    const input_form = document.getElementById('input')
    const md2_output_form = document.getElementById('md2')
    const md4_output_form = document.getElementById('md4')
    const md5_output_form = document.getElementById('md5')
    function md2() {
        const input_string = (new TextEncoder).encode(input_form.value)
        const md2_digest = new module.Md2Ctx(input_string).digest()
        md2_output_form.value = md2_digest
    }
    function md4() {
        const input_string = (new TextEncoder).encode(input_form.value)
        const md4_digest = new module.Md4Ctx(input_string).digest()
        md4_output_form.value = md4_digest
    }
    function md5() {
        const input_string = (new TextEncoder).encode(input_form.value)
        const md5_digest = new module.Md5Ctx(input_string).digest()
        md5_output_form.value = md5_digest
    }
    function calculate() {
        md2()
        md4()
        md5()
    }
    input_form.onload = calculate()
    input_form.oninput = function() {
        calculate()
    }
})
