
# list all tasks
default:
    @just --list

# build the plugin
build: build-bindings build-js 

# build the javascript
build-js:
    pnpm run build

# build the js -> rust bindings
build-bindings:
    cargo test export_types --features build-types