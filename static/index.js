import(/* webpackChunkName: "ghash-wasm" */"../pkg").then(module => {
    const input_form = document.getElementById('input')
    const md2_output_form = document.getElementById('md2')
    const md4_output_form = document.getElementById('md4')
    const md5_output_form = document.getElementById('md5')
    const sha0_output_form = document.getElementById('sha0')
    const sha1_output_form = document.getElementById('sha1')
    function md2() {
        const input_string = (new TextEncoder).encode(input_form.value)
        md2_output_form.value = new module.Md2(input_string).digest()
    }
    function md4() {
        const input_string = (new TextEncoder).encode(input_form.value)
        md4_output_form.value = new module.Md4(input_string).digest()
    }
    function md5() {
        const input_string = (new TextEncoder).encode(input_form.value)
        md5_output_form.value = new module.Md5(input_string).digest()
        
    }
    function sha0() {
        const input_string = (new TextEncoder).encode(input_form.value)
        sha0_output_form.value = new module.Sha0(input_string).digest()
    }
    function sha1() {
        const input_string = (new TextEncoder).encode(input_form.value)
        sha1_output_form.value = new module.Sha1(input_string).digest()
        
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
