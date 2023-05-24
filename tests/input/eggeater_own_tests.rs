use crate::*;

// Your tests go here!
success_tests! {
    {
        name: simple_examples_1,
        file: "input/simple_examples.snek",
        input: "0",
        expected: "1",
    },
    {
        name: simple_examples_2,
        file: "input/simple_examples.snek",
        input: "1",
        expected: "10",
    },
    {
        name: simple_examples_3,
        file: "input/simple_examples.snek",
        input: "2",
        expected: "100",
    },
    {
        name: points,
        file: "input/points.snek",
        expected: "11\n22\n0",
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
