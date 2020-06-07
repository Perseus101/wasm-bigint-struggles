class Imports {
    constructor() {}

    api_get_contract_address() {
        console.log("api_get_contract_address");
        return 0;
    }
    api_get_sender_address() {
        console.log("api_get_sender_address");
        return 0;
    }
    api_get_timestamp() {
        console.log("api_get_timestamp");
        return 0;
    }

    api_get_i32(index) {
        console.log(`api_get_i32 ${index}`);
        return 0;
    }
    api_get_i64(index) {
        console.log(`api_get_i64 ${index}`);
        return 0n;
    }
    api_get_f32(index) {
        console.log(`api_get_f32 ${index}`);
        return 0.0;
    }
    api_get_f64(index) {
        console.log(`api_get_f64 ${index}`);
        return 0.0;
    }

    api_has_mapping(index, key) {
        console.log(`api_has_mapping: ${index}, ${key}`);
        return 0;
    }
    api_get_mapping(index, key) {
        console.log(`api_get_mapping: ${index}, ${key}`);
        return 0n;
    }

    api_set_i32(index, value) {
        console.log(`api_set_i32: ${index}, ${value}`);
    }
    api_set_i64(index, value) {
        console.log(`api_set_i64: ${index}, ${value}`);
    }
    api_set_f32(index, value) {
        console.log(`api_set_f32: ${index}, ${value}`);
    }
    api_set_f64(index, value) {
        console.log(`api_set_f64: ${index}, ${value}`);
    }

    api_set_mapping(index, key, value) {
        console.log(`api_set_mapping: ${index}, ${key}, ${value}`);
    }
}

function make_imports() {
    return { env: new Imports() };
}