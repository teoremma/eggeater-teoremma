use crate::*;

// Your tests go here!
success_tests! {
    {
        name: fact,
        file: "input/simple_examples.snek",
        input: "1",
        expected: "2",
    },
}

runtime_error_tests! {}

static_error_tests! {
    // {
    //     name: duplicate_params,
    //     file: "diamondback_own/duplicate_params.snek",
    //     expected: "",
    // }
}
