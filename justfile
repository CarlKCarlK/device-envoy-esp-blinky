set shell := ["bash", "-cu"]

run arg1="c6" arg2="":
    ./scripts/device-action.sh run {{arg1}} {{arg2}}

check arg1="c6" arg2="":
    ./scripts/device-action.sh check {{arg1}} {{arg2}}

build arg1="c6" arg2="":
    ./scripts/device-action.sh build {{arg1}} {{arg2}}
