clean-rom:
    rm -rf programs/target/

build-rom program: clean-rom
    mkdir -p programs/target
    ca65 -g -o programs/target/{{ program }}.o programs/{{ program }}.s
    ld65 -o programs/target/{{ program }} -C programs/ben_eater.cfg programs/target/{{ program }}.o
