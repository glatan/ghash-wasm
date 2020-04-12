import(/* webpackChunkName: "ghash-wasm" */"../pkg").then(module => {
    const input_form = document.getElementById('input')
    const md2_output_form = document.getElementById('md2')
    const md4_output_form = document.getElementById('md4')
    const md5_output_form = document.getElementById('md5')
    const sha0_output_form = document.getElementById('sha0')
    const sha1_output_form = document.getElementById('sha1')
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
    function sha0() {
        const input_string = (new TextEncoder).encode(input_form.value)
        const sha0_digest = new module.Sha0(input_string).digest()
        sha0_output_form.value = sha0_digest
    }
    function sha1() {
        const input_string = (new TextEncoder).encode(input_form.value)
        const sha1_digest = new module.Sha1(input_string).digest()
        sha1_output_form.value = sha1_digest
    }
    
    function calculate() {
        md2()
        md4()
        md5()
        sha0()
        sha1()
    }
    input_form.onload = calculate()
    input_form.oninput = function() {
        calculate()
    }
})
