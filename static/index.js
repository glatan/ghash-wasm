import "./style.scss";
import(/* webpackChunkName: "ghash" */"../pkg").then(module => {
    module.run_app();
});
