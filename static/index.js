import(/* webpackChunkName: "ghash-wasm" */"../pkg").then(module => {
    const input_form = document.getElementById('input')
    const md2_output_form = document.getElementById('md2')
    const md4_output_form = document.getElementById('md4')
    const md5_output_form = document.getElementById('md5')
    const sha0_output_form = document.getElementById('sha0')
    const sha1_output_form = document.getElementById('sha1')
    function calculate() {
        const input = (new TextEncoder).encode(input_form.value)
        md2_output_form.value = new module.Md2(input).digest()
        md4_output_form.value = new module.Md4(input).digest()
        md5_output_form.value = new module.Md5(input).digest()
        sha0_output_form.value = new module.Sha0(input).digest()
        sha1_output_form.value = new module.Sha1(input).digest()
    }
    input_form.onload = calculate()
    input_form.oninput = function() {
        calculate()
    }
})
